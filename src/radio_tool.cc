/**
 * This file contains all the code needed to execute radio_tool wrap and flash operations
 */

#include <memory>
#include <rtxtool/include/radio_tool.h>
#include <radio_tool/radio/radio_factory.hpp>

namespace radio_tool::radio
{
    void radio_tool_flash(){
        auto rdFactory = RadioFactory();
        // We flash the first radio
        uint16_t index = 0;
        auto radio = rdFactory.OpenDevice(index);
        auto in_file = "./test.bin";
        radio->WriteFirmware(in_file);
}

} // namespace radio_tool::radio
