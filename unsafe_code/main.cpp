#include "mandelbrot_wrapper.h"
#include "rust_stuff.h"
#include <iostream>

int main() {
  // rusty string stuff
  const char *rust_greeting = get_string_from_rust();
  std::cout << rust_greeting << '\n';
  free_rust_string(rust_greeting);

  // thing struct
  Thing *thing = create_thing();
  std::cout << "Thing created\n";
  std::cout << "Thing int: " << thing->int_value << " = "
            << thing_get_int_value(thing) << '\n';

  std::cout << "Thing float: " << thing->float_value << " = "
            << thing_get_float_value(thing) << '\n';

  std::cout << "Thing string: '" << thing->string_value << "' = '"
            << thing_get_string_value(thing) << "'\n";

  thing_set_int_value(thing, 2);
  thing_set_float_value(thing, 6.28f);
  thing_set_string_value(thing, "bonjour from c++");

  thing_print(thing);
  destroy_thing(thing);

  MandelbrotWrapper mandelbrot;
  mandelbrot.generate(9000, 6000, 100);
  mandelbrot.save("mandelbrot.png");

  return 0;
}
