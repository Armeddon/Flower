pub const STDLIB_H: &[u8] = r#"#pragma once

#include "varlist.h"

extern Variable *flwr_id(Variable **args, VarList *lst);

extern Variable *flwr_readInt(Variable **args, VarList *lst);

extern Variable *flwr_readString(Variable **args, VarList *lst);

extern Variable *flwr_println(Variable **args, VarList *lst);

extern Variable *flwr_add(Variable **args, VarList *lst);
"#.as_bytes();
pub const STDLIB_C: &[u8] = r#"#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "flwrstdlib.h"
#include "varlist.c"
#include "varlist.h"

Variable *flwr_id(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 1));
    Variable *copy = var_cpy(lst->value);
    var_take_delete(&lst, min(var_len(args), 1));
    return copy;
}

Variable *flwr_readInt(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 0));
    int *input = malloc(sizeof(int));
    scanf("%d", input);
    Variable *var = malloc(sizeof(Variable));
    var->value = input;
    var->type = Int;
    var_take_delete(&lst, min(var_len(args), 0));
    return var;
}

Variable *flwr_readString(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 1));
    Variable *_arg0 = var_get(lst, 0);
    if (var_get_type(_arg0) != Int) {
        return NULL;
    }
    int limit = *(int*)_arg0->value;
    char *input = malloc(limit + 1);
    scanf("%s", input);
    Variable *var = malloc(sizeof(Variable));
    var->value = malloc(sizeof(string));
    *(string*)var->value = (string) {
        .len = strlen(input),
        .str = input
    };
    var->type = String;
    var_take_delete(&lst, min(var_len(args), 1));
    return var;
}

Variable *flwr_println(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 1));
    Variable *_arg0 = var_get(lst, 0);
    
    switch (_arg0->type) {
        case Int:
            printf("%d\n", *(int*)_arg0->value);
            break;
        case String:
            printf("%s\n", ((string*)_arg0->value)->str);
            break;
        default:
            break;
    }

    var_take_delete(&lst, min(var_len(args), 1));
    return NULL;
}

Variable *flwr_add(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 2));
    Variable *_arg0 = var_get(lst, 0);
    Variable *_arg1 = var_get(lst, 1);
    if (var_get_type(_arg0) != Int) {
        var_delete(lst);
        return NULL;
    }
    if (var_get_type(_arg1) != Int) {
        var_delete(lst);
        return NULL;
    }
    Variable *sum = malloc(sizeof(Variable));
    sum->type = Int;
    sum->value = malloc(sizeof(int));
    *(int*)sum->value = *(int*)(_arg0->value) + *(int*)(_arg1->value);
    var_take_delete(&lst, min(var_len(args), 2));
    return sum;
}
"#.as_bytes();
pub const VARLIST_H: &[u8] = r#"#pragma once

enum Type {
    Undefined,
    Int,
    Unit,
    String,
};

struct Variable;
struct VarList;

typedef struct Variable Variable;
typedef struct VarList VarList;

extern int min(int a, int b);

extern enum Type var_get_type(Variable *var);

extern Variable *var_create(enum Type tp, void *value);

extern VarList *var_take_copy(VarList *lst, int n);

extern void var_enqueue(VarList **begin_list, Variable *var);

extern void var_dequeue(VarList **begin_list);

extern Variable *var_get(VarList *begin_list, int n);

extern void var_delete(VarList *list);

extern Variable *var_cpy(Variable *var);

extern void var_take_delete(VarList **list, int n);

extern void var_prepend(VarList **list, Variable *val);

extern void var_take_pextend(VarList **list, Variable **args, int n);

extern void var_free(Variable *var);

extern int var_len(Variable **args);
"#.as_bytes();
pub const VARLIST_C: &[u8] = r#"#include <stdlib.h>
#include <string.h>
#include "varlist.h"
#include "string.c"

size_t type_size(enum Type type) {
    switch (type) {
        case Int:
            return sizeof(int);
        case Unit:
        case Undefined:
        case String:
            return sizeof(string);
        default:
            return 0;
    }
}

int min(int a, int b) {
    return a < b ? a : b;
}

struct Variable {
    void *value;
    enum Type type;
};

enum Type var_get_type(Variable *var) {
    if (!var) return Undefined;
    return var->type;
}

Variable *var_create(enum Type tp, void *value) {
    Variable *var = malloc(sizeof(Variable));
    *var = (Variable) {
        .value = value,
        .type = tp
    };
    return var;
}

struct VarList {
   Variable *value;
   struct VarList *next;
};

VarList *var_take_copy(VarList *lst, int n) {
    if (!lst) return NULL;
    if (!n) return NULL;
    VarList *cpy = malloc(sizeof(VarList));
    *cpy = (VarList) {
        .value = var_cpy(lst->value),
        .next = var_take_copy(lst->next, n - 1)
    };
    return cpy;
}

void var_enqueue(VarList **begin_list, Variable *var) {
    VarList *node = malloc(sizeof(VarList));
    node->value = var;
    node->next = NULL;
    if (*begin_list == NULL) {
        *begin_list = node;
    } else {
        VarList *cur = *begin_list;
        while (cur->next != NULL) {
            cur = cur->next;
        }
        cur->next = node;
    }
 }

void var_free(Variable *var) {
    free(var->value);
    free(var);
}

void var_dequeue(VarList **begin_list) {
    var_free((*begin_list)->value);
    VarList *next = (*begin_list)->next;
    free(*begin_list);
    *begin_list = next;
}

Variable *var_get(VarList *begin_list, int n) {
    if (!begin_list) return NULL;
    if (n == 0) {
        return begin_list->value;
    }
    return var_get(begin_list->next, n - 1);
}

void var_delete(VarList *list) {
    if (list == NULL) return;
    var_free(list->value);
    VarList *next = list->next;
    free(list);
    var_delete(next);
}

void var_take_delete(VarList **list, int n) {
    if (!n) return;
    var_dequeue(list);
    var_take_delete(list, n - 1);
}

void *var_value_cpy(void *src, enum Type tp) {
    switch (tp) {
        case String: {
             char *str = ((string*)src)->str;
             string *res = malloc(sizeof(string));
             res->str = malloc(strlen(str));
             strcpy(res->str, str);
             return res;
         }
        case Unit:
        case Undefined:
             return NULL;
        case Int:
        default: {
             void *dest = malloc(type_size(tp));
             memcpy(dest, src, type_size(tp));
             return dest;
        }
    }
}

Variable *var_cpy(Variable *var) {
    if (!var) return NULL;
    Variable *cpy = malloc(sizeof(Variable));
    cpy->type = var->type;
    cpy->value = var_value_cpy(var->value, var->type);
    return cpy;
}

void var_prepend(VarList **lst, Variable *var) {
    if (!var) return;
    VarList *new_lst = malloc(sizeof(VarList));
    *new_lst = (VarList) {
        .value = var_cpy(var),
        .next = *lst
    };
    *lst = new_lst;
}

int var_len(Variable **args) {
    if (!args) return 0;
    int cnt = 0;
    while (*(args++)) {
        cnt++;
    }
    return cnt;
}

void var_take_pextend(VarList **lst, Variable **args, int n) {
    for (int i = 0; i < n; i++) {
        var_prepend(lst, args[i]);
    }
}
"#.as_bytes();
pub const STRING_H: &[u8] = r#"struct string;

typedef struct string string;

extern string *new_string(int len, char *str);

extern void delete_string(string *str);
"#.as_bytes();
pub const STRING_C: &[u8] = r#"#include <stdlib.h>
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
"#.as_bytes();
