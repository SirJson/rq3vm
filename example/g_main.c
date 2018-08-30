

void print(const char* fmt, int fmt_count, ...);
void message(const char* msg);
void file_write(const char* path, const char* content);

int vmMain(int command, int arg0, int arg1, int arg2, int arg3, int arg4,
           int arg5, int arg6, int arg7, int arg8, int arg9, int arg10,
           int arg11)
{
    float vals[10];
    int i;
    int b = 3;
    for (i = 0; i < 10; i++) {
        vals[i] = 0.45f;
    }

    print("Hi {1}. This is a more complex example of formating strings in {2}. Because I really don't like {3}", 3, "Mr. Internet guy", "Rust", "printf()");
    print("I will try to write {1} on your harddrive...", 1, "something");
    file_write("secret.txt","Don't click on that link!\n\nhttps://www.youtube.com/watch?v=2Z4m4lnjxkY");
    message("The file is written and it looks like we don't have to implement everything as a syscall");
    return 0;
}

void message(const char* msg) {
    print(msg,0);
}