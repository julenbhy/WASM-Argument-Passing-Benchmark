char* func(char* json) {
    int a, b;

    sscanf(strstr(json, "\"param1\":") + strlen("\"param1\":"), "%d", &a);
    sscanf(strstr(json, "\"param2\":") + strlen("\"param1\":"), "%d", &b);

    // Build the result json
    char *result = malloc(20);
    sprintf(result, "{\"result\": %d}", a+b);

    return result;
}