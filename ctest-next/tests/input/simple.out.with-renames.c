/* This file was autogenerated by ctest; do not modify directly */

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#include <simple.h>

static char *ctest_const_A_val_static = A;

// Define a function that returns a pointer to the value of the constant to test.
// This will later be called on the Rust side via FFI.
char *ctest_const_cstr__A(void) {
    return ctest_const_A_val_static;
}

static char *ctest_const_B_val_static = C_B;

// Define a function that returns a pointer to the value of the constant to test.
// This will later be called on the Rust side via FFI.
char *ctest_const_cstr__B(void) {
    return ctest_const_B_val_static;
}
