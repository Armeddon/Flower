#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

#include "flwrstdlib.h"
#include "varlist.c"
#include "varlist.h"

Variable *identity(Variable **args, VarList *lst) {
    if (*args == NULL) {
        Variable *var = malloc(sizeof(Variable));
        var->type = lst->value->type;
        switch (var->type) {
            case Int:
                var->value = malloc(sizeof(int));
                *(int*)var->value = *(int*)lst->value->value;
                break;
            default:
                break;
        }
        return var;
    } else {
        Variable *var = malloc(sizeof(Variable));
        var->type = (*args)[0].type;
        switch (var->type) {
            case Int:
                var->value = malloc(sizeof(int));
                *(int*)var->value = *(int*)(*args)[0].value;
                break;
            default:
                break;
        }
        return var;
    }
}

Variable *readInt(Variable **args, VarList *lst) {
    int *input = malloc(sizeof(int));
    scanf("%d", input);
    Variable *var = malloc(sizeof(Variable));
    var->value = input;
    var->type = Int;
    return var;
}

Variable *println(Variable **args, VarList *lst) {
    Variable *_arg0;
    switch ((size_t)*args) {
        case (size_t)NULL:
            _arg0 = lst->value;
            break;
        default:
            _arg0 = &(*args)[0];
            break;
    }
    
    switch (_arg0->type) {
        case Int:
            printf("%d\n", *(int*)_arg0->value);
            break;
        default:
            break;
    }

    return NULL;
}

Variable *add(Variable **args, VarList *lst) {
    Variable *_arg0;
    switch ((size_t)*args) {
        case (size_t)NULL:
            _arg0 = lst->value;
            break;
        default:
            _arg0 = &(*args)[0];
            break;
    }
    Variable *_arg1;
    switch ((size_t)(*args + 1 * (_arg0 == *args))) {
        case (size_t)NULL:
            if (_arg0 == lst->value) {
                _arg1 = lst->next->value;
            } else {
                _arg1 = lst->value;
            }
            break;
        default:
            _arg1 = &(*args)[(_arg0 == lst->value)];
            break;
    }
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
    return sum;
}
