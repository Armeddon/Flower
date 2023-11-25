#pragma once

struct Variable;
struct VarList;

typedef struct Variable Variable;

extern void var_enqueue(struct VarList **begin_list, struct VarList **end_list, Variable var);

extern struct VarList *var_dequeue(struct VarList *begin_list);

extern void var_delete(struct VarList *list);
