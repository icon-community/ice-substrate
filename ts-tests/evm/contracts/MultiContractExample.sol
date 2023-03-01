pragma solidity >=0.8.0;

import {Storage} from "./Storage.sol";

contract MultiContractExample {

    Storage public store;

    constructor(address storageAddress) {
        store = Storage(storageAddress);
    }

    function setStorage() external {
        store.setStorage(
            0x2000000000000000000000000000000000000000000000000000000000000000,
            0x2000000000000000000000000000000000000000000000000000000000000000
        );
    }

}