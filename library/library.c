#include "library.h"

int hello_world_static() {
    printf("Hello World from C! (inline static string)");
    return 0;
}

char* hello_world_return_pointer() {
    char* hello = malloc(100);
    hello = "Hello World!";
    return hello;
}
