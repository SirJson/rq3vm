

void print(const char* fmt, int fmt_count, ...);
void message(const char* msg);
void file_write(const char* path, const char* content);
int to_int(const char* str);
void test_vm();


int vmMain(int command, int arg0, int arg1, int arg2, int arg3, int arg4,
           int arg5, int arg6, int arg7, int arg8, int arg9, int arg10,
           int arg11)
{
    switch(command) {
        case 0:
            test_vm();
            break;
        default:
            message("Unknown command. VM is confused now");
            break;
    }
        
    return 0;
}


void test_vm()
{
    float vals[10];
    
    int converted_value = -1;
    int base_value = 6;
    int result;

    int i;
    for (i = 0; i < 10; i++) {
        vals[i] = 0.45f;
    }

    print("Hi {0}. This is a more complex example of formating strings in {1}. Because I really don't like {2}", 3, "Mr. Internet guy", "Rust", "printf()");
    print("I will try to write {0} on your harddrive...", 1, "something");
    file_write("secret.txt","Don't click on that link!\n\nhttps://www.youtube.com/watch?v=2Z4m4lnjxkY");
    message("Now we try to convert a string literal into an int and do some math...");

    converted_value = to_int("12");
    result = converted_value + base_value;

    if(result == 18) {
        message("Yep that worked!");
    }
    else {
        message("Oh no!");
    }
}

void message(const char* msg) {
    print(msg,0);
}
