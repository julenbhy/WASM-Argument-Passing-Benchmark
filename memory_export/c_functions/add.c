#include <stdio.h>
#include <stdlib.h>
#include <string.h>


char* func(char* json) {
    int a, b;

    sscanf(strstr(json, "\"param1\":") + strlen("\"param1\":"), "%d", &a);
    sscanf(strstr(json, "\"param2\":") + strlen("\"param1\":"), "%d", &b);

    // Build the result json
    char *result = malloc(20);
    sprintf(result, "{\"result\": %d}", a+b);

    return result;
}


const char *INPUT;
char* __attribute__((export_name("set_input"))) *set_input(size_t size) {
    INPUT = (char*) malloc(size);
    return INPUT;
}

const char *RESULT;
char __attribute__((export_name("get_result"))) *get_result() { return strdup(RESULT); }
size_t __attribute__((export_name("get_result_len"))) get_result_len() { return strlen(RESULT); }


int main(int argc, char *argv[]) { 

    RESULT = func(INPUT); 

    printf("From WASM: \n\tResult ptr: %d\n\tResult len: %d\n\tResult content: %s\n", RESULT, strlen(RESULT), RESULT);
    return 0; 
}


