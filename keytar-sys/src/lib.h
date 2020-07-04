#pragma once
#include "rust/cxx.h"
#include "node-keytar/src/keytar.h"

struct Password;

void set_password(rust::Str service, rust::Str account, rust::Str password);
Password get_password(rust::Str service, rust::Str account);
bool delete_password(rust::Str service, rust::Str account);
Password find_password(rust::Str service);
