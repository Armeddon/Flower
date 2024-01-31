pub const STDLIB_H: &[u8] = r#"#pragma once

#include "varlist.h"

extern Variable *identity(Variable **args, VarList *lst);

extern Variable *readInt(Variable **args, VarList *lst);

extern Variable *println(Variable **args, VarList *lst);

extern Variable *add(Variable **args, VarList *lst);
"#.as_bytes();
pub const STDLIB_C: &[u8] = r#"#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include "flwrstdlib.h"
#include "varlist.c"
#include "varlist.h"

Variable *identity(Variable **args, VarList *lst) {
    var_pextend(&lst, args);
    Variable *copy = var_cpy(lst->value);
    var_take_delete(&lst, var_len(args));
    return copy;
}

Variable *readInt(Variable **args, VarList *lst) {
    int *input = malloc(sizeof(int));
    scanf("%d", input);
    Variable *var = malloc(sizeof(Variable));
    var->value = input;
    var->type = Int;
    return var;
}

Variable *println(Variable **args, VarList *lst) {
    var_pextend(&lst, args);
    Variable *_arg0 = var_get(lst, 0);
    
    switch (_arg0->type) {
        case Int:
            printf("%d\n", *(int*)_arg0->value);
            break;
        default:
            break;
    }

    var_take_delete(&lst, var_len(args));
    return NULL;
}

Variable *add(Variable **args, VarList *lst) {
    var_pextend(&lst, args);
    Variable *_arg0 = var_get(lst, 0);
    Variable *_arg1 = var_get(lst, 1);
    if (_arg0->type != Int) {
        return NULL;
    }
    if (_arg1->type != Int) {
        return NULL;
    }
    Variable *sum = malloc(sizeof(Variable));
    sum->type = Int;
    sum->value = malloc(sizeof(int));
    *(int*)sum->value = *(int*)(_arg0->value) + *(int*)(_arg1->value);
    var_take_delete(&lst, var_len(args));
    return sum;
}
"#.as_bytes();
pub const VARLIST_H: &[u8] = r#"#pragma once

enum Type {
    Int,
    Unit,
};

struct Variable;
struct VarList;

typedef struct Variable Variable;
typedef struct VarList VarList;

extern Variable *var_create(enum Type tp, void *value);

extern void var_enqueue(VarList **begin_list, Variable *var);

extern void var_dequeue(VarList **begin_list);

extern Variable *var_get(VarList *begin_list, int n);

extern void var_delete(VarList *list);

extern Variable *var_cpy(Variable *var);

extern void var_take_delete(VarList **list, int n);

extern void var_prepend(VarList **list, Variable *val);

extern void var_pextend(VarList **list, Variable **args);

extern void var_free(Variable *var);
"#.as_bytes();
pub const VARLIST_C: &[u8] = r#"#include <stdlib.h>
#include <string.h>
#include "varlist.h"

size_t type_size(enum Type type) {
    switch (type) {
        case Int:
            return sizeof(int);
        case Unit:
        default:
            return 0;
    }
}

struct Variable {
    void *value;
    enum Type type;
};

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

Variable *var_cpy(Variable *var) {
    if (!var) return NULL;
    Variable *cpy = malloc(sizeof(Variable));
    cpy->type = var->type;
    cpy->value = malloc(type_size(cpy->type));
    memcpy(cpy->value, var->value, type_size(cpy->type));
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

void var_pextend(VarList **lst, Variable **args) {
    for (int i = 0; i < var_len(args); i++) {
        var_prepend(lst, args[i]);
    }
}
"#.as_bytes();
