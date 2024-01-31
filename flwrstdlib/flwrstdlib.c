#include <stddef.h>
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
    var_pextend(&lst, args);
    int *input = malloc(sizeof(int));
    scanf("%d", input);
    Variable *var = malloc(sizeof(Variable));
    var->value = input;
    var->type = Int;
    var_take_delete(&lst, var_len(args));
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
