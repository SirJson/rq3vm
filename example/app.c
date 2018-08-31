#include "app.h"
#include "syslib.h"

void test_vm()
{
    float vals[10];
    
    int converted_value = -1;
    int base_value = 6;
    int result;
    int my_string;

    int i;
    for (i = 0; i < 10; i++) {
        vals[i] = 0.45f;
    }

    print("Hi {0}. This is a more complex example of formating strings in {1}. Because I really don't like {2}", "Mr. Internet guy", "Rust", "printf()");
    print("I will try to write {0} on your harddrive...", "something");
    file_write("secret.txt","Don't click on that link!\n\nhttps://www.youtube.com/watch?v=2Z4m4lnjxkY");
    message("Now we try to convert a string literal into an int and do some math...");

    converted_value = to_int32("12");
    result = converted_value + base_value;

    if(result == 18) {
        message("Yep that worked!");
    }
    else {
        message("Oh no!");
    }

    message("Now we try the string lib!");
    my_string = string_new("Rust strings! Now available in your C code");
    str(my_string);
    string_free(my_string);
    message("And we are done!");
}
