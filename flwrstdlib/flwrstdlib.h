#pragma once

#include "varlist.h"

extern Variable *flwr_id(Variable **args, VarList *lst);

extern Variable *flwr_readInt(Variable **args, VarList *lst);

extern Variable *flwr_readString(Variable **args, VarList *lst);

extern Variable *flwr_println(Variable **args, VarList *lst);

extern Variable *flwr_add(Variable **args, VarList *lst);

extern Variable *flwr_lt(Variable **args, VarList *lst);

extern Variable *flwr_and(Variable **args, VarList *lst);

extern Variable *flwr_not(Variable **args, VarList *lst);

extern Variable *flwr_if(Variable **args, VarList *lst);
