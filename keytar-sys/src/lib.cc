#include "src/lib.h"
#include "src/lib.rs.h"
#include <stdexcept>

void set_password(rust::Str service, rust::Str account, rust::Str password)
{
  std::string error;
  keytar::KEYTAR_OP_RESULT result = keytar::SetPassword(std::string(service),
                                                        std::string(account),
                                                        std::string(password),
                                                        &error);

  if (result == keytar::FAIL_ERROR)
  {
    throw std::logic_error(error);
  }
}

Password get_password(rust::Str service, rust::Str account)
{
  std::string error;
  std::string password;
  keytar::KEYTAR_OP_RESULT result = keytar::GetPassword(std::string(service),
                                                        std::string(account),
                                                        &password,
                                                        &error);

  if (result == keytar::FAIL_ERROR)
  {
    throw std::logic_error(error);
  }
  else if (result == keytar::FAIL_NONFATAL)
  {
    return Password{false, rust::String("")};
  }
  else
  {
    return Password{true, rust::String(password)};
  }
}

bool delete_password(rust::Str service, rust::Str account)
{
  std::string error;
  keytar::KEYTAR_OP_RESULT result = keytar::DeletePassword(std::string(service), std::string(account), &error);

  if (result == keytar::FAIL_ERROR)
  {
    throw std::logic_error(error);
  }
  else if (result == keytar::FAIL_NONFATAL)
  {
    return false;
  }
  else
  {
    return true;
  }
}

Password find_password(rust::Str service)
{
  std::string error;
  std::string password;
  keytar::KEYTAR_OP_RESULT result = keytar::FindPassword(std::string(service),
                                                         &password,
                                                         &error);
  if (result == keytar::FAIL_ERROR)
  {
    throw std::logic_error(error);
  }
  else if (result == keytar::FAIL_NONFATAL)
  {
    return Password{false, rust::String("")};
  }
  else
  {
    return Password{true, rust::String(password)};
  }
}
