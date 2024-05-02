#pragma once
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
