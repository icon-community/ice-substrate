// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.2 <0.9.0;

contract NetworkUpgrade {

    uint256 number;
    string message;
    bytes message_bytes;
    address addr;
    mapping(string => uint16) chain_id;
    uint256[] num_array;
    string[4] fixed_str_array = ["ICE", "SNOW", "Arctic", "Frost"];

    enum Days
    {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
     }
     Days d;
    
    struct TS {
        int8 num1;
        bytes1 bt;
        bool b1;
        address addr;
        string name;
    }
    TS simple_struct;

    struct BS {
        bytes1 one_char;
        bytes3 three_char;
        bytes4 four_char;
        bytes16 sixteen_char;
        bytes32 thirtytwo_char;
    }
    BS bytes_struct;

    struct Contract_state {
        uint256 number;
        string message;
        uint16[2] chain_id;
        
        uint256[] num_array;
        string[4] fixed_str_array;
        address addr;
        Days d;
        TS simple_struct;
        BS bytes_struct;
        bytes message_bytes;
    }

    constructor(uint256 _num, string memory _msg) {
        number = _num;
        message = _msg;
        num_array.push(_num);
        num_array.push(_num * 2);
        addr = 0x8eFcaF2C4eBbf88Bf07f3BB44a2869C4C675AD7A;
        d = Days.Sunday;
        bytes_struct = BS("a", "123", "a1b2", "!@#$%^&*()123456", "abcdefghijklmnopqrstuvwxyz123456");
        simple_struct = TS(127, "1", true, 0x8eFcaF2C4eBbf88Bf07f3BB44a2869C4C675AD7A, "SNOW");
        message_bytes = bytes(_msg);
        chain_id["snow"] = 552;
        chain_id["arctic"] = 553;
    }

    function get() public view returns(Contract_state memory) {
        return Contract_state(number, message, [chain_id["snow"], chain_id["arctic"]], num_array, fixed_str_array, addr, d, simple_struct, bytes_struct, message_bytes);
    }
}