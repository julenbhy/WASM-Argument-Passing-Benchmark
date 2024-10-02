#include <stdio.h>
#include <stdlib.h>
#include <string.h>


char* func(char* json) {
    int a, b;

    // VERSION 1: Get the inputs from the command line
    //a = atoi(argv[1]);
    //b = atoi(argv[2]);

    // VERSION 2: Get the inputs from the json
    sscanf(strstr(json, "\"param1\":") + strlen("\"param1\":"), "%d", &a);
    sscanf(strstr(json, "\"param2\":") + strlen("\"param1\":"), "%d", &b);

    // Build the result json
    char *result = malloc(20);
    sprintf(result, "{\"result\": %d}", a+b);

    return result;
}


const char *RESULT;
char __attribute__((export_name("get_result"))) *get_result() { return strdup(RESULT); }
size_t __attribute__((export_name("get_result_len"))) get_result_len() { return strlen(RESULT); }


int main(int argc, char *argv[]) { 
    // Manage the input to json and pass the json to the function
    char* result = func(argv[1]); 

    RESULT = result;

    printf("From WASM: \n\tResult ptr: %d\n\tResult len: %d\n\tResult content: %s\n", &result, strlen(result), result);

    return 0; 
}


