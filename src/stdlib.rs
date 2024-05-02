pub const STDLIB_H: &[u8] = r#"#pragma once

#include "varlist.h"

extern Variable *flwr_id(Variable **args, VarList *lst);

extern Variable *flwr_readInt(Variable **args, VarList *lst);

extern Variable *flwr_readString(Variable **args, VarList *lst);

extern Variable *flwr_println(Variable **args, VarList *lst);

extern Variable *flwr_add(Variable **args, VarList *lst);

extern Variable *flwr_lt(Variable **args, VarList *lst);

extern Variable *flwr_and(Variable **args, VarList *lst);

extern Variable *flwr_not(Variable **args, VarList *lst);

extern Variable *flwr_if(Variable **args, VarList *lst);
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
        var_take_delete(&lst, min(var_len(args), 1));
        return NULL;
    }
    int limit = *(int*)_arg0->value;
    char *input = malloc(limit + 1);
    scanf("%s", input);
    Variable *var = malloc(sizeof(Variable));
    var->value = malloc(sizeof(string)+strlen(input));
    ((string*)var->value)->len  = strlen(input);
    strcpy(((string*)var->value)->str, input);
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
        case Bool:
            if (*(_Bool*)_arg0->value)
                printf("True\n");
            else
                printf("False\n");
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
        var_take_delete(&lst, min(var_len(args), 2));
        return NULL;
    }
    if (var_get_type(_arg1) != Int) {
        var_take_delete(&lst, min(var_len(args), 2));
        return NULL;
    }
    Variable *sum = malloc(sizeof(Variable));
    sum->type = Int;
    sum->value = malloc(sizeof(int));
    *(int*)sum->value = *(int*)(_arg0->value) + *(int*)(_arg1->value);
    var_take_delete(&lst, min(var_len(args), 2));
    return sum;
}

Variable *flwr_lt(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 2));
    Variable *_arg0 = var_get(lst, 0);
    Variable *_arg1 = var_get(lst, 1);
    if (var_get_type(_arg0) != var_get_type(_arg1)) {
        var_take_delete(&lst, min(var_len(args), 2));
        return NULL;
    }
    Variable *cmp = malloc(sizeof(Variable));
    cmp->type = Bool;
    cmp->value = malloc(sizeof(_Bool));
    switch (var_get_type(_arg0)) {
        case Int:
            *(_Bool*)cmp->value = *(int*)_arg0->value < *(int*)_arg1->value;
            break;
        case String:
            *(_Bool*)cmp->value = string_lt((string*)_arg0->value, (string*)_arg1->value);
            break;
        case Bool:
            *(_Bool*)cmp->value = (char)*(_Bool*)_arg0->value < (char)*(_Bool*)_arg1->value;
            break;
        case Undefined:
        case Unit:
        default:
            *(_Bool*)cmp->value = 0;
    }
    var_take_delete(&lst, min(var_len(args), 2));
    return cmp;
}

Variable *flwr_not(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 1));
    Variable *_arg0 = var_get(lst, 0);
    if (var_get_type(_arg0) != Bool) {
        var_take_delete(&lst, min(var_len(args), 1));
        return NULL;
    }
    Variable *res = malloc(sizeof(Variable));
    res->type = Bool;
    res->value = malloc(sizeof(_Bool));
    *(_Bool*)res->value = !*(_Bool*)_arg0->value;
    var_take_delete(&lst, min(var_len(args), 1));
    return res;
}

Variable *flwr_and(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 2));
    Variable *_arg0 = var_get(lst, 0);
    Variable *_arg1 = var_get(lst, 1);
    if (var_get_type(_arg0) != Bool) {
        var_take_delete(&lst, min(var_len(args), 2));
        return NULL;
    }
    if (var_get_type(_arg1) != Bool) {
        var_take_delete(&lst, min(var_len(args), 2));
        return NULL;
    }
    Variable *res = malloc(sizeof(Variable));
    res->type = Bool;
    res->value = malloc(sizeof(_Bool));
    *(_Bool*)res->value = *(_Bool*)_arg0->value && *(_Bool*)_arg1->value;
    var_take_delete(&lst, min(var_len(args), 2));
    return res;
}

Variable *flwr_if(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 3));
    Variable *_arg0 = var_get(lst, 0);
    Variable *_arg1 = var_get(lst, 1);
    Variable *_arg2 = var_get(lst, 2);
    if (var_get_type(_arg0) != Bool) {
        var_take_delete(&lst, min(var_len(args), 3));
        return NULL;
    }
    if (var_get_type(_arg1) != var_get_type(_arg2)) {
        var_take_delete(&lst, min(var_len(args), 3));
        return NULL;
    }
    Variable *res = NULL;
    if (*(_Bool*)_arg0->value) {
        res = var_cpy(_arg1);
    } else {
        res = var_cpy(_arg2);
    }
    var_take_delete(&lst, min(var_len(args), 3));
    return res;
}
"#.as_bytes();
pub const VARLIST_H: &[u8] = r#"#pragma once
#include <stdbool.h>

enum Type {
    Undefined,
    Int,
    Unit,
    String,
    Bool,
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

extern int var_null(Variable *var);

extern _Bool var_get_bool(Variable *var);
"#.as_bytes();
pub const VARLIST_C: &[u8] = r#"#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include "varlist.h"
#include "string.c"

size_t type_size(enum Type type) {
    switch (type) {
        case Int:
            return sizeof(int);
        case String:
            return sizeof(string);
        case Bool:
            return sizeof(_Bool);
        case Unit:
        case Undefined:
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
    if (!var) return;
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
             int len = strlen(str);
             string *res = malloc(sizeof(string) + len);
             res->len = len;
             strncpy(res->str, str, len);
             return res;
         }
        case Unit:
        case Undefined:
             return NULL;
        case Int:
        case Bool:
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
    for (int i = n-1; i >= 0; i--) {
        var_prepend(lst, args[i]);
    }
}

int var_null(Variable *var) {
    return !var || !var->value;
}

_Bool var_get_bool(Variable *var) {
    switch (var_get_type(var)) {
        case Bool:
            return *(_Bool*)var->value;
        default:
            break;
    }
    return false;
}
"#.as_bytes();
pub const STRING_H: &[u8] = r#"#include "stdbool.h"

struct string;

typedef struct string string;

extern string *string_new(int len, char *str);

extern void string_delete(string *str);

extern _Bool string_lt(string *s1, string *s2);
"#.as_bytes();
pub const STRING_C: &[u8] = r#"#include <stdlib.h>
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
"#.as_bytes();
pub const STD_FLWR: &[u8] = r#"define ge :>
T -> T -> Bool :>
    lt => not
;>

define eq :>
T -> T -> Bool :>
    id |>
    id =>
    id |>
    ge =>
    ge =>
    and
;>

define neq :>
T -> T -> Bool :>
    eq => not
;>

define or :>
Bool -> Bool -> Bool :>
    not =>
    not =>
    and =>
    not
;>

define gt :>
T -> T -> Bool :>
    ge |>
    eq =>
    id =>
    not =>
    and
;>

define le :>
T -> T -> Bool :>
    lt |>
    eq =>
    or
;>
"#.as_bytes();
