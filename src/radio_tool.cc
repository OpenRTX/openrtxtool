/**
 * This file contains all the code needed to execute radio_tool wrap and flash operations
 */

#include <memory>
#include <rtxtool/include/radio_tool.h>
#include <radio_tool/radio/radio_factory.hpp>
#include <radio_tool/radio/tyt_radio.hpp>
#include <radio_tool/radio/yaesu_radio.hpp>

namespace radio_tool::radio
{

    // List compatible connected devices
    void list_devices(){
        auto rdFactory = RadioFactory();
        const auto &d = rdFactory.ListDevices();
        for(auto i : d)
        {
            std::wcout << i->ToString() << std::endl;
        }
    }

    // Flash the first connected radio
    void flash_radio(){
        auto rdFactory = RadioFactory();
        const auto &d = rdFactory.ListDevices();
        if(d.size() <= 0)
            throw std::runtime_error("No radio detected");
        // We flash the first radio
        uint16_t index = 0;
        auto radio = rdFactory.OpenDevice(index);
        auto in_file = "./test.bin";
        radio->WriteFirmware(in_file);
    }

    // Reboot the first connected radio
    void reboot_radio(){
        auto rdFactory = RadioFactory();
        const auto &d = rdFactory.ListDevices();
        if(d.size() <= 0)
            throw std::runtime_error("No radio detected");
        // We flash the first radio
        uint16_t index = 0;
        auto radio = rdFactory.OpenDevice(index);
        auto tyt_radio = dynamic_cast<const radio_tool::radio::TYTRadio *>(radio);
        auto yaesu_radio = dynamic_cast<const radio_tool::radio::YaesuRadio *>(radio);
        auto device = tyt_radio->GetDevice();
        auto dfu = device->GetDFU();
        dfu.Reboot();
    }

} // namespace radio_tool::radio
