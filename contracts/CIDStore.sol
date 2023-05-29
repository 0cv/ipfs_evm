pragma solidity ^0.8.19;

contract CIDStore {
    mapping(address => string[]) private _cids;

    // Function to store a list of CIDs for the sender
    function store(string memory cid) public {
        _cids[msg.sender].push(cid);
    }

    // Function to get a list of CIDs for a given address
    function retrieve(address userAddress) public view returns (string[] memory){
        return _cids[userAddress];
    }
}
