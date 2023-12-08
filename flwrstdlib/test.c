#include "flwrstdlib.c"
#include "varlist.h"

#include <stdlib.h>

/* 
define main :>
() :>
    readInt =>
    readInt =>
    add =>
    println
;>
*/

int main() {
    {
        VarList *_begin_list = NULL;
        VarList *_end_list = NULL;
        {
            Variable *_params[] = { NULL };
            Variable *_res = readInt(_params, _begin_list);
            if (_res != NULL) {
                var_enqueue(&_begin_list, &_end_list, _res);
            }
        }
        {
            Variable *_params[] = { NULL };
            Variable *_res = readInt(_params, _begin_list);
            if (_res != NULL) {
                var_enqueue(&_begin_list, &_end_list, _res);
            }
        }
        {
            Variable *_params[] = { NULL };
            Variable *_res = add(_params, _begin_list);
            var_dequeue(&_begin_list);
            var_dequeue(&_begin_list);
            if (_res != NULL) {
                var_enqueue(&_begin_list, &_end_list, _res);
            }
        }
        {
            Variable *_params[] = { NULL };
            Variable *_res = println(_params, _begin_list);
            var_dequeue(&_begin_list);
            if (_res != NULL) {
                var_enqueue(&_begin_list, &_end_list, _res);
            }
        }
        var_delete(_begin_list);
    }
    return 0;
}
