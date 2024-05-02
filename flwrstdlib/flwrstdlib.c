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
        var_take_delete(&lst, min(var_len(args), 1));
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
        var_take_delete(&lst, min(var_len(args), 2));
        return NULL;
    }
    if (var_get_type(_arg1) != Int) {
        var_take_delete(&lst, min(var_len(args), 2));
        return NULL;
    }
    Variable *sum = malloc(sizeof(Variable));
    sum->type = Int;
    sum->value = malloc(sizeof(int));
    *(int*)sum->value = *(int*)(_arg0->value) + *(int*)(_arg1->value);
    var_take_delete(&lst, min(var_len(args), 2));
    return sum;
}

Variable *flwr_lt(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 2));
    Variable *_arg0 = var_get(lst, 0);
    Variable *_arg1 = var_get(lst, 1);
    if (var_get_type(_arg0) != var_get_type(_arg1)) {
        var_take_delete(&lst, min(var_len(args), 2));
        return NULL;
    }
    Variable *cmp = malloc(sizeof(Variable));
    cmp->type = Bool;
    cmp->value = malloc(sizeof(_Bool));
    switch (var_get_type(_arg0)) {
        case Int:
            *(_Bool*)cmp->value = *(int*)_arg0->value < *(int*)_arg1->value;
            break;
        case String:
            *(_Bool*)cmp->value = string_lt((string*)_arg0->value, (string*)_arg1->value);
            break;
        case Bool:
            *(_Bool*)cmp->value = (char)*(_Bool*)_arg0->value < (char)*(_Bool*)_arg1->value;
            break;
        case Undefined:
        case Unit:
        default:
            *(_Bool*)cmp->value = 0;
    }
    var_take_delete(&lst, min(var_len(args), 2));
    return cmp;
}

Variable *flwr_not(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 1));
    Variable *_arg0 = var_get(lst, 0);
    if (var_get_type(_arg0) != Bool) {
        var_take_delete(&lst, min(var_len(args), 1));
        return NULL;
    }
    Variable *res = malloc(sizeof(Variable));
    res->type = Bool;
    res->value = malloc(sizeof(_Bool));
    *(_Bool*)res->value = !*(_Bool*)_arg0->value;
    var_take_delete(&lst, min(var_len(args), 1));
    return res;
}

Variable *flwr_and(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 2));
    Variable *_arg0 = var_get(lst, 0);
    Variable *_arg1 = var_get(lst, 1);
    if (var_get_type(_arg0) != Bool) {
        var_take_delete(&lst, min(var_len(args), 2));
        return NULL;
    }
    if (var_get_type(_arg1) != Bool) {
        var_take_delete(&lst, min(var_len(args), 2));
        return NULL;
    }
    Variable *res = malloc(sizeof(Variable));
    res->type = Bool;
    res->value = malloc(sizeof(_Bool));
    *(_Bool*)res->value = *(_Bool*)_arg0->value && *(_Bool*)_arg1->value;
    var_take_delete(&lst, min(var_len(args), 2));
    return res;
}

Variable *flwr_if(Variable **args, VarList *lst) {
    var_take_pextend(&lst, args, min(var_len(args), 3));
    Variable *_arg0 = var_get(lst, 0);
    Variable *_arg1 = var_get(lst, 1);
    Variable *_arg2 = var_get(lst, 2);
    if (var_get_type(_arg0) != Bool) {
        var_take_delete(&lst, min(var_len(args), 3));
        return NULL;
    }
    if (var_get_type(_arg1) != var_get_type(_arg2)) {
        var_take_delete(&lst, min(var_len(args), 3));
        return NULL;
    }
    Variable *res = NULL;
    if (*(_Bool*)_arg0->value) {
        res = var_cpy(_arg1);
    } else {
        res = var_cpy(_arg2);
    }
    var_take_delete(&lst, min(var_len(args), 3));
    return res;
}
