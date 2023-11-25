#include <stdlib.h>

typedef struct {
    void *value;
    enum {
        Int,
        Unit,
    } type;
} Variable;

struct VarList {
   Variable value;
   struct VarList *next;
};

void var_enqueue(struct VarList *end_list, Variable var) {
    struct VarList *node = malloc(sizeof(struct VarList));
    node->value = var;
    node->next = NULL;
    end_list->next = node;
}

struct VarList *var_dequeue(struct VarList *begin_list) {
    free(begin_list->value.value);
    struct VarList *next = begin_list->next;
    free(begin_list);
    return next;
}

void var_delete(struct VarList *list) {
    if (list == NULL) return;
    struct VarList *next = list->next;
    free(list);
    var_delete(next);
}
