#include <stdlib.h>
#include "varlist.h"

enum Type {
    Int,
    Unit,
};

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
