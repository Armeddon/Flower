#pragma once

#include "varlist.h"

extern Variable *identity(Variable **args, VarList *lst);

extern Variable *readInt(Variable **args, VarList *lst);

extern Variable *println(Variable **args, VarList *lst);

extern Variable *add(Variable **args, VarList *lst);
