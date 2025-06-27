#pragma once

extern "C" {
// String stuff
const char *get_string_from_rust();
void free_rust_string(const char *s);

// Thing stuff
typedef struct Thing {
  int int_value;
  float float_value;
  const char *string_value;
} Thing;

Thing *create_thing();
void destroy_thing(Thing *thing);
void thing_print(const Thing *thing);

void thing_set_int_value(Thing *thing, int value);
void thing_set_float_value(Thing *thing, float value);
void thing_set_string_value(Thing *thing, const char *value);

int thing_get_int_value(const Thing *thing);
float thing_get_float_value(const Thing *thing);
const char *thing_get_string_value(const Thing *thing);
}
