#pragma once

enum Type {
    Int,
    Unit,
};

struct Variable;
struct VarList;

typedef struct Variable Variable;
typedef struct VarList VarList;

extern int min(int a, int b);

extern Variable *var_create(enum Type tp, void *value);

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
