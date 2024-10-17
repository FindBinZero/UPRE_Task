//
// Created by FindBin on 2024/10/17.
//

#include <iostream>
#include <string>
#include <map>
#include <thread>

void fn_task_one() {
    std::string input_str{};
    std::cin >> input_str;
    std::map<std::string, std::string> command_map{
            {"name",  "FBResistor"},
            {"class", "MPS-Class"},
            {"emil",  "1578557118@qq.com"},
            {"phone", "13620464128"},
            {"exit",  "bye"}
    };
    for (;;) {
        auto result = command_map.find(input_str);
        if (result != command_map.end()) {
            std::cout << result->second << std::endl;
            if (result->first == std::string{"exit"}) {
                break;
            }
        } else {
            std::cout << "try again" << std::endl;
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }
}