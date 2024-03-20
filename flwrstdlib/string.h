#include "stdbool.h"

struct string;

typedef struct string string;

extern string *string_new(int len, char *str);

extern void string_delete(string *str);

extern _Bool string_eq(string *s1, string *s2);
