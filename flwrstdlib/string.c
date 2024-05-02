#include <stdlib.h>
#include <string.h>
#include "string.h"

struct string {
    int len;
    char str[];
};

string *string_new(int len, char* str) {
    string *res = malloc(sizeof(string)+len);
    res->len = len;
    strncpy(res->str, str, len);
    return res;
}

void string_delete(string *str) {
    free(str);
}

_Bool string_lt(string *s1, string *s2) {
    if (!s1 && !s2)
        return 0;
    if (!s1)
        return 1;
    if (!s2)
        return 0;
    if (s1->len > s2->len)
        return 0;
    for (int i = 0; i < s1->len; i++)
        if (s1->str[i] >= s2->str[i])
            return 0;
    return 1;
}
