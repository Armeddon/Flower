#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include "flwrstdlib.h"
#include "varlist.c"

Variable *flwr_id(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 1));
    Variable *copy = var_cpy(lst->value);
    var_take_delete(&lst, min(var_len(args), 1));
    return copy;
}

Variable *flwr_readInt(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 0));
    int *input = malloc(sizeof(int));
    scanf("%d", input);
    Variable *var = malloc(sizeof(Variable));
    var->value = input;
    var->type = Int;
    var_take_delete(&lst, min(var_len(args), 0));
    return var;
}

Variable *flwr_println(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 1));
    Variable *_arg0 = var_get(lst, 0);
    
    switch (_arg0->type) {
        case Int:
            printf("%d\n", *(int*)_arg0->value);
            break;
        default:
            break;
    }

    var_take_delete(&lst, min(var_len(args), 1));
    return NULL;
}

Variable *flwr_add(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 2));
    Variable *_arg0 = var_get(lst, 0);
    Variable *_arg1 = var_get(lst, 1);
    if (var_get_type(_arg0) != Int) {
        var_delete(lst);
        return NULL;
    }
    if (var_get_type(_arg1) != Int) {
        var_delete(lst);
        return NULL;
    }
    Variable *sum = malloc(sizeof(Variable));
    sum->type = Int;
    sum->value = malloc(sizeof(int));
    *(int*)sum->value = *(int*)(_arg0->value) + *(int*)(_arg1->value);
    var_take_delete(&lst, min(var_len(args), 2));
    return sum;
}
