#pragma once

#include "varlist.h"

extern Variable *flwr_identity(Variable **args, VarList *lst);

extern Variable *flwr_readInt(Variable **args, VarList *lst);

extern Variable *flwr_println(Variable **args, VarList *lst);

extern Variable *flwr_add(Variable **args, VarList *lst);
