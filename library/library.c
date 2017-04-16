#include "library.h"

int hello_world_static() {
    printf("Hello World from C! (inline static string)\n");
    return 0;
}

char* hello_world_return_pointer() {
    char* hello = malloc(BUFSIZE);
    strcpy(hello, "Hello World!");
    return hello;
}
