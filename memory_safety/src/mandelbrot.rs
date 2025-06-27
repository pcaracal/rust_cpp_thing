use std::{
    io::Write as _,
    sync::{Arc, Mutex},
};

#[derive(Default)]
pub struct Mandelbrot {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
}

#[unsafe(no_mangle)]
pub extern "C" fn mandelbrot_new() -> *mut Mandelbrot {
    Box::into_raw(Box::new(Mandelbrot::default()))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mandelbrot_free(ptr: *mut Mandelbrot) {
    if ptr.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(ptr)) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mandelbrot_generate(
    ptr: *mut Mandelbrot,
    width: usize,
    height: usize,
    iterations: usize,
) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let mandelbrot = &mut *ptr;
        mandelbrot.generate(width, height, iterations);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mandelbrot_save(ptr: *mut Mandelbrot, path: *const u8) {
    if ptr.is_null() || path.is_null() {
        return;
    }

    unsafe {
        let mandelbrot = &*ptr;
        let c_str = std::ffi::CStr::from_ptr(path.cast::<i8>());
        let path_str = c_str.to_str().expect("Invalid UTF-8 string");
        mandelbrot.save(path_str);
    }
}

#[allow(
    clippy::many_single_char_names,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss
)]
impl Mandelbrot {
    pub fn generate(&mut self, width: usize, height: usize, iterations: usize) {
        self.width = width;
        self.height = height;

        let chunk_count = 16;
        let chunk_size = height.div_ceil(chunk_count);
        let chunks = Arc::new(Mutex::new(Vec::with_capacity(chunk_count)));
        let mut handles = Vec::with_capacity(chunk_count);

        for (chunk_index, chunk) in (0..chunk_count).enumerate() {
            let start_y = chunk * chunk_size;
            let end_y = ((chunk + 1) * chunk_size).min(height);
            let chunks = Arc::clone(&chunks);

            let ci = chunk_index;
            handles.push(std::thread::spawn(move || {
                if end_y <= start_y {
                    return;
                }
                let mut chunk_buffer = vec![0u8; (end_y - start_y) * width * 4];

                for y in start_y..end_y {
                    for x in 0..width {
                        let mut zx = 0.0;
                        let mut zy = 0.0;
                        let c_x = (x as f64 / width as f64) * 3.0 - 2.0;
                        let c_y = (y as f64 / height as f64) * 2.0 - 1.0;
                        let mut i = 0;
                        while zx * zx + zy * zy < 4.0 && i < iterations {
                            let tmp = zx * zx - zy * zy + c_x;
                            zy = 2.0 * zx * zy + c_y;
                            zx = tmp;
                            i += 1;
                        }

                        let color = Self::color_for_pixel(iterations, i, zx, zy);
                        let chunk_pixel_index = ((y - start_y) * width + x) * 4;
                        chunk_buffer[chunk_pixel_index] = color[0];
                        chunk_buffer[chunk_pixel_index + 1] = color[1];
                        chunk_buffer[chunk_pixel_index + 2] = color[2];
                        chunk_buffer[chunk_pixel_index + 3] = color[3];
                    }
                }

                let mut chunks = chunks.lock().unwrap();
                chunks.push((ci, chunk_buffer));

                println!("Chunk {ci} done");
            }));
        }

        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        let mut chunks = chunks.lock().unwrap();
        chunks.sort_by_key(|&(index, _)| index);

        self.buffer.clear();
        for (_, chunk_buffer) in chunks.drain(..) {
            self.buffer.extend(chunk_buffer);
        }
    }

    pub fn save(&self, path: &str) {
        let a = image::RgbaImage::from_vec(self.width as _, self.height as _, self.buffer.clone())
            .expect("Couldnt imagine the image");
        a.save(path).expect("Failed to save image");
    }

    fn color_for_pixel(max_i: usize, i: usize, zx: f64, zy: f64) -> [u8; 4] {
        if i == max_i {
            [0, 0, 0, 255]
        } else {
            let zn = (zx * zx + zy * zy).sqrt();
            let nu = (zn.ln() / 2.0_f64.ln()).ln() / 2.0_f64.ln();
            let smooth = i as f64 + 1.0 - nu;
            let t = smooth / max_i as f64;

            let hue = 360.0 * t;
            let (r, g, b) = Self::hsv_to_rgb(hue, 1.0, 1.0);

            [r, g, b, 255]
        }
    }

    fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        let (r1, g1, b1) = match h as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };
        (
            ((r1 + m) * 255.0) as u8,
            ((g1 + m) * 255.0) as u8,
            ((b1 + m) * 255.0) as u8,
        )
    }
}

mod tests {

    #[test]
    fn test_mandelbrot() {
        let mut mandelbrot = super::Mandelbrot::default();
        mandelbrot.generate(9000, 6000, 100);
        mandelbrot.save("test_mandelbrot.png");
    }
}
