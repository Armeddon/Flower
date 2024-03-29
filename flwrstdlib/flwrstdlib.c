#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "flwrstdlib.h"
#include "varlist.c"
#include "varlist.h"

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

Variable *flwr_readString(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 1));
    Variable *_arg0 = var_get(lst, 0);
    if (var_get_type(_arg0) != Int) {
        return NULL;
    }
    int limit = *(int*)_arg0->value;
    char *input = malloc(limit + 1);
    scanf("%s", input);
    Variable *var = malloc(sizeof(Variable));
    var->value = malloc(sizeof(string)+strlen(input));
    ((string*)var->value)->len  = strlen(input);
    strcpy(((string*)var->value)->str, input);
    var->type = String;
    var_take_delete(&lst, min(var_len(args), 1));
    return var;
}

Variable *flwr_println(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 1));
    Variable *_arg0 = var_get(lst, 0);
    
    switch (_arg0->type) {
        case Int:
            printf("%d\n", *(int*)_arg0->value);
            break;
        case String:
            printf("%s\n", ((string*)_arg0->value)->str);
            break;
        case Bool:
            if (*(_Bool*)_arg0->value)
                printf("True\n");
            else
                printf("False\n");
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

Variable *flwr_eq(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 2));
    Variable *_arg0 = var_get(lst, 0);
    Variable *_arg1 = var_get(lst, 1);
    if (var_get_type(_arg0) != var_get_type(_arg1)) {
        var_delete(lst);
        return NULL;
    }
    Variable *eq = malloc(sizeof(Variable));
    eq->type = Bool;
    eq->value = malloc(sizeof(_Bool));
    switch (var_get_type(_arg0)) {
        case Int:
            *(_Bool*)eq->value = *(int*)_arg0->value == *(int*)_arg1->value;
            break;
        case String:
            *(_Bool*)eq->value = string_eq((string*)_arg0->value, (string*)_arg1->value);
            break;
        case Bool:
            *(_Bool*)eq->value = (char)*(_Bool*)_arg0->value == (char)*(_Bool*)_arg1->value;
            break;
        case Undefined:
        case Unit:
        default:
            *(_Bool*)eq->value = 1;
    }
    var_take_delete(&lst, min(var_len(args), 2));
    return eq;
}
