#include <memory>
#include <rtxtool/include/radio_factory.h>

namespace radio_tool::radio
{
    // Function returning a unique_ptr to use a opaque type from Rust
    // https://cxx.rs/tutorial.html#calling-a-c-function-from-rust
    std::unique_ptr<RadioFactory> new_radiofactory() {
      return std::make_unique<RadioFactory>();
}

} // namespace radio_tool::radio
