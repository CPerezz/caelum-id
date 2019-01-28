pragma solidity ^0.5.0;

contract ERC735 {

    // Basic Struct.
    struct Claim {
        uint256 topic;
        uint256 scheme;
        address issuer;
        bytes32 signature;
        bytes32 data;
        bytes32 uri;
    }

    // Setters.
    function addClaim(uint256 topic, uint256 scheme, bytes32 signature, bytes32 data, bytes32 uri) public returns (bytes32 claimRequestId);
    function changeClaim(bytes32 claimId, uint256 scheme, bytes32 signature, bytes32 data, bytes32 uri) public;
    function removeClaim(bytes32 claimId) public;

    // Getters.
    function getClaim(bytes32 claimId) public view returns(uint256 topic, uint256 scheme, address issuer, bytes32 signature, bytes32 data, bytes32 uri);
    function getClaimIdsByTopic(uint256 topic) public view returns(bytes32[] memory claimIds);

    // Events.
    event ClaimAdded(bytes32 indexed claimId, uint256 indexed topic, uint256 scheme, address indexed issuer, bytes32 signature, bytes32 data, bytes32 uri);
    event ClaimRemoved(bytes32 indexed claimId, uint256 indexed topic, uint256 scheme, address indexed issuer, bytes32 signature, bytes32 data, bytes32 uri);
    event ClaimChanged(bytes32 indexed claimId, uint256 indexed topic, uint256 scheme, address indexed issuer, bytes32 signature, bytes32 data, bytes32 uri);

}

