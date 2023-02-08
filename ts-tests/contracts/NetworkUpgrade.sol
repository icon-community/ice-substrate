// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.2 <0.9.0;

contract NetworkUpgrade {

    uint256 number;
    string message;
    uint256[] testArray;
    
    struct TS {
        uint256 num;
        string message;
    }
    TS testStruct;

    struct ContractState {
        uint256 number;
        string message;
        uint256[] testArray;
        TS testStruct;
    }

    constructor(uint256 _num, string memory _msg) {
        number = _num;
        message = _msg;
        testStruct = TS(_num, _msg);
        testArray.push(_num);
        testArray.push(_num * 2);
    }

    function get() public view returns(ContractState memory) {
        return ContractState(number, message, testArray, testStruct);
    }
}