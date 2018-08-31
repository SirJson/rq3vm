#ifndef SYSLIB_H
#define SYSLIB_H

void print(const char* fmt, ...);
void message(const char* msg);
void file_write(const char* path, const char* content);
int to_int32(const char* str);
int string_new(const char* str);
void string_free(int string);
int string_len(int string);
void str(int string);

#endif // SYSLIB_H
