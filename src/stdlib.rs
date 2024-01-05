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
    if (*args == NULL) {
        Variable *var = malloc(sizeof(Variable));
        var->type = lst->value->type;
        switch (var->type) {
            case Int:
                var->value = malloc(sizeof(int));
                *(int*)var->value = *(int*)lst->value->value;
                break;
            default:
                break;
        }
        return var;
    } else {
        Variable *var = malloc(sizeof(Variable));
        var->type = (*args)[0].type;
        switch (var->type) {
            case Int:
                var->value = malloc(sizeof(int));
                *(int*)var->value = *(int*)(*args)[0].value;
                break;
            default:
                break;
        }
        return var;
    }
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
    Variable *_arg0;
    switch ((size_t)*args) {
        case (size_t)NULL:
            _arg0 = lst->value;
            break;
        default:
            _arg0 = &(*args)[0];
            break;
    }
    
    switch (_arg0->type) {
        case Int:
            printf("%d\n", *(int*)_arg0->value);
            break;
        default:
            break;
    }

    return NULL;
}

Variable *add(Variable **args, VarList *lst) {
    Variable *_arg0;
    switch ((size_t)*args) {
        case (size_t)NULL:
            _arg0 = lst->value;
            break;
        default:
            _arg0 = &(*args)[0];
            break;
    }
    Variable *_arg1;
    switch ((size_t)(*args + 1 * (_arg0 == *args))) {
        case (size_t)NULL:
            if (_arg0 == lst->value) {
                _arg1 = lst->next->value;
            } else {
                _arg1 = lst->value;
            }
            break;
        default:
            _arg1 = &(*args)[(_arg0 == lst->value)];
            break;
    }
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
    return sum;
}
"#.as_bytes();
pub const VARLIST_H: &[u8] = r#"#pragma once

struct Variable;
struct VarList;

typedef struct Variable Variable;
typedef struct VarList VarList;

extern void var_enqueue(VarList **begin_list, Variable *var);

extern void var_dequeue(VarList **begin_list);

extern Variable *var_get(VarList **begin_list, int n);

extern void var_delete(VarList *list);

extern Variable *var_cpy(Variable *var);
"#.as_bytes();
pub const VARLIST_C: &[u8] = r#"#include <stdlib.h>
#include <string.h>
#include "varlist.h"

enum Type {
    Int,
    Unit,
};

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

static void var_free(Variable *var) {
    free(var->value);
    free(var);
}

void var_dequeue(VarList **begin_list) {
    var_free((*begin_list)->value);
    VarList *next = (*begin_list)->next;
    free(*begin_list);
    *begin_list = next;
}

Variable *var_get(VarList **begin_list, int n) {
    if (!*begin_list) return NULL;
    if (n == 0) {
        return (*begin_list)->value;
    }
    return var_get(&(*begin_list)->next, n - 1);
}

void var_delete(VarList *list) {
    if (list == NULL) return;
    var_free(list->value);
    VarList *next = list->next;
    free(list);
    var_delete(next);
}

Variable *var_cpy(Variable *var) {
    if (!var) return NULL;
    Variable *cpy = malloc(sizeof(Variable));
    cpy->type = var->type;
    cpy->value = malloc(type_size(cpy->type));
    memcpy(cpy->value, var->value, type_size(cpy->type));
    return cpy;
}
"#.as_bytes();
