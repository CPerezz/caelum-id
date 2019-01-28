pragma solidity ^0.5.0;

contract ERC725 {

    // Data structs.
    uint256 constant MANAGEMENT_KEY = 1;
    uint256 constant ACTION_KEY = 2;
    uint256 constant CLAIM_SIGNER_KEY = 3;

    struct Key {
        uint256 purpose; //e.g., MANAGEMENT_KEY = 1, ACTION_KEY = 2, etc.
        uint256 keyType; // e.g. 1 = ECDSA, 2 = RSA, etc.
        bytes32 key;
    }

    // Setters.
    function addKey(bytes32 key, uint256 purpose, uint256 keyType) public returns (bytes32, uint256, uint256);
    function removeKey(bytes32 key) public;

    // Getters.
    function getKey(bytes32 key) public view returns(uint256 purpose, uint256 keyType, bytes32 return_key);
    function keyHasPurpose(bytes32 key, uint256 purpose) public view returns(bool exists);
    function getKeysByPurpose(uint256 purpose) public view returns(bytes32[] memory keys);

    // Events.
    event KeyAdded(bytes32 indexed key, uint256 indexed purpose, uint256 keyType);
    event KeyRemoved(bytes32 indexed key, uint256 indexed purpose);

}
