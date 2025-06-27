#pragma once

#include <cstddef>
#include <cstdio>

extern "C" {
typedef struct Mandelbrot Mandelbrot;

Mandelbrot *mandelbrot_new();
void mandelbrot_free(Mandelbrot *mandelbrot);

void mandelbrot_generate(Mandelbrot *mandelbrot, size_t width, size_t height,
                         size_t iterations);

void mandelbrot_save(Mandelbrot *mandelbrot, const char *path);
}

struct MandelbrotWrapper {
  Mandelbrot *mandelbrot;
  MandelbrotWrapper() : mandelbrot(mandelbrot_new()) {}

  ~MandelbrotWrapper() { mandelbrot_free(mandelbrot); }
  void generate(size_t width, size_t height, size_t iterations) {
    mandelbrot_generate(mandelbrot, width, height, iterations);
  }
  void save(const char *path) { mandelbrot_save(mandelbrot, path); }
};
