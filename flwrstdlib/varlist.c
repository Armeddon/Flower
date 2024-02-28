#include <stdlib.h>
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

int var_null(Variable *var) {
    return !var || !var->value;
}
