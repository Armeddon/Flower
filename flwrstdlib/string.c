#include <stdlib.h>
#include <string.h>
#include "string.h"

struct string {
    int len;
    char *str;
};

string *new_string(int len, char* str) {
    string *res = malloc(sizeof(string));
    res->len = len;
    res->str = malloc(len);
    strncpy(res->str, str, len);
    return res;
}

void delete_string(string *str) {
    free(str->str);
    free(str);
}
