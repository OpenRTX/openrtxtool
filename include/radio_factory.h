#pragma once
#include "rust/cxx.h"
#include <memory>
#include <radio_tool/radio/radio_factory.hpp>

namespace radio_tool::radio {

std::unique_ptr<RadioFactory> new_radiofactory();

} // namespace radio_tool::radio
