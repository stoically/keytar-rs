#pragma once
#include "rust/cxx.h"
#include "node-keytar/src/keytar.h"
#include <memory>
#include <string>

struct Password;

void set_password(rust::String service, rust::String account, rust::String password);
Password get_password(rust::String service, rust::String account);
bool delete_password(rust::String service, rust::String account);
Password find_password(rust::String service);