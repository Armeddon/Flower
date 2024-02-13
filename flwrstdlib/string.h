struct string;

typedef struct string string;

extern string *new_string(int len, char *str);

extern void delete_string(string *str);
