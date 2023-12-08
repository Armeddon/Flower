#pragma once

struct Variable;
struct VarList;

typedef struct Variable Variable;
typedef struct VarList VarList;

extern void var_enqueue(VarList **begin_list, Variable *var);

extern void var_dequeue(VarList **begin_list);

extern Variable *var_get(VarList **begin_list, int n);

extern void var_delete(VarList *list);
