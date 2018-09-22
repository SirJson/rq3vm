#include "app.h"

void test_vm()
{
    float vecf[3];

    int converted_value = -1;
    int base_value = 6;
    int result;
    int my_string;

    int i;
    vecf[0] = 0.45f;
    for (i = 0; i < 2; i++)
    {
        vecf[i] *= 0.45f;
    }

    print("Hi {0}. This is a more complex example of formating strings in {1}. Because I really don't like {2}", "Mr. Internet guy", "Rust", "printf()");
    print("Did you know we support floats? This could be a vector: Vector(x: {0}, y: {1}, z: {2})", vecf[0], vecf[1], vecf[2]);
    print("I will try to write {0} on your harddrive...", "SOME TEXT FILE WITH A SECRET");
    print("secret.txt", "Don't click on that link!\n\nhttps://www.youtube.com/watch?v=2Z4m4lnjxkY");
    print("Now we try to convert a string literal into an int and do some math...");

    converted_value = to_int32("12");
    result = converted_value + base_value;

    if (result == 18)
    {
        message("Yep that worked!");
    }
    else
    {
        message("Oh no!");
    }

    print("Now we try string allocation");
    string_free(my_string);
    print("And we are done!");
}
