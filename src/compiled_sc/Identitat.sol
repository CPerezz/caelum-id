pragma solidity ^0.5.0;

import "./ERC725.sol";
import "./ERC735.sol";


/** @title Self-sovereign identity. */
contract Identitat is ERC725, ERC735 {

    bytes32 public version = "0.2";

    // Mapping of addres.
    mapping (bytes32 => Key) keysStruct;
    mapping (uint256 => bytes32[]) keysByPurpose;

    // Mapping of Claims.
    mapping (bytes32 => Claim) claims;
    mapping (uint256 => bytes32[]) claimIdsByTopic;

    /*
     * MUST d only be done by keys of purpose 1, or the identity itself.
     */
    modifier managerOnly {
        require(keysStruct[keccak256(abi.encodePacked(msg.sender))].purpose == MANAGEMENT_KEY, "Must be Management Key"); //1
        _;
    }

    /**
     * @dev Constructor that create a MANAGEMENT_KEY to the contract deployer.
     */
    constructor() public {
        bytes32 key = keccak256(abi.encodePacked(msg.sender));
        keysStruct[key] = Key({
            purpose: 1,
            keyType: 1,
            //key: msg.sender
            key: key
        });
        keysByPurpose[1].push(key);
        emit KeyAdded(key, keysStruct[key].purpose,keysStruct[key].keyType);
    }



    /** @dev Adds a _key to the identity. The _purpose specifies the purpose of key. Initially we propose four purposes
      * @param _key the new key to want to create.
      * @param _purpose the purpose of the key
      * @param _keyType the type of encryptation of the key.
      * @return success true if the key succefully created or false if not.
      */
    function addKey( bytes32 _key, uint256 _purpose, uint256 _keyType) public managerOnly returns (bytes32, uint256, uint256) {
        bytes32 key = keccak256(abi.encodePacked(_key));

        // Key must not exist
        require(keysStruct[key].key != key, "Key must not exist");

        // Add Key
        keysStruct[key] = Key( {
            purpose: _purpose,
            keyType: _keyType,
            key: key
        });

        // Update array keys by purpose.
        keysByPurpose[_purpose].push(key);
        emit KeyAdded(key, _purpose, _keyType);
        return (key, _purpose, _keyType);
    }

    /** @dev Removes key from the identity.
      * @param _key key that want to delete.
      */
    function removeKey(bytes32 _key) public managerOnly {
        bytes32 key = keccak256(abi.encodePacked(_key));


        // Key must exist.
        require(keysStruct[key].key == key, "Key must exist");

        // Save purpose and remove Key.
        uint256 purpose = keysStruct[key].purpose;
        delete keysStruct[key];

        // Remove from KeysByPurpose.
        for(uint i = 0; i<keysByPurpose[purpose].length; i++){
            if(keysByPurpose[purpose][i] == key)
                delete keysByPurpose[purpose][i];
        }

        emit KeyRemoved(key, purpose);
    }

    /** @dev Requests the ADDITION of a claim from an issuer.
      * @param _topic is the type of claim.
      * @param _scheme the estructure of the claim
      * @param _signature is a signed message of the following structure: keccak256(address subject_address, uint256 topic, bytes data).
      * @param _data data of the claim if exist
      * @param _uri link for more information of the claim
      * @return claimId Unique identifier for this claim,
      */
    function addClaim(uint256 _topic, uint256 _scheme, bytes32 _signature, bytes32 _data, bytes32 _uri) public returns (bytes32 claimRequestId) {

        // Only Claimer Keys
        //require((keysStruct[keccak256(abi.encodePacked(msg.sender))].purpose == CLAIM_SIGNER_KEY) || (keysStruct[keccak256(abi.encodePacked(msg.sender))].purpose == MANAGEMENT_KEY), "Only CLAIM signer Key can add Claims");

        // Create Claim ID as a hash of the topic and the issuer.
        bytes32 claimId = keccak256(abi.encodePacked(msg.sender, _topic));

        // Avoid collisions in th e hash.
        require(claims[claimId].topic == 0, "Collision found");

        // adds the new claim.
        claims[claimId] = Claim( {
            topic: _topic,
            scheme: _scheme,
            issuer: msg.sender,
            signature: _signature,
            data: _data,
            uri: _uri
        });

        // New Id by Topic.
        claimIdsByTopic[_topic].push(claimId);
        emit ClaimAdded(claimId, _topic, _scheme, msg.sender, _signature, _data, _uri);
        return claimId;
    }

    // function getSigner() public returns(bytes32 sender, bytes32 claimer) {
        // return ((bytes32) (msg.sender), )
    // }

    /**
      * @dev Requests the CHANGE of a claim from an issuer.
      * @param _claimId id of the claim.
      * @param _scheme the estructure of the claim
      * @param _signature is a signed message of the following structure: keccak256(address subject_address, uint256 topic, bytes data).
      * @param _data data of the claim if exist
      * @param _uri link for more information of the claim
      */
    function changeClaim(bytes32 _claimId, uint256 _scheme, bytes32 _signature, bytes32 _data, bytes32 _uri) public {
        // Only Claimer Keys
        /* require(keysStruct[keccak256(abi.encodePacked(msg.sender))].purpose == CLAIM_SIGNER_KEY, "Only CLAIM signer Key can add Claims"); */
        require((keysStruct[keccak256(abi.encodePacked(msg.sender))].purpose == CLAIM_SIGNER_KEY) || (keysStruct[keccak256(abi.encodePacked(msg.sender))].purpose == MANAGEMENT_KEY), "Only CLAIM signer Key can add Claims");

        // Claim must exist.
        require(claims[_claimId].topic > 0, "Claim must exist to be changed");

        // Update Claim.
        claims[_claimId].scheme = _scheme;
        claims[_claimId].signature = _signature;
        claims[_claimId].data = _data;
        claims[_claimId].uri = _uri;
        emit ClaimChanged(_claimId, claims[_claimId].topic, _scheme, msg.sender, _signature, _data, _uri);
    }

    /**
      * @dev Removes a claim. Can only be removed by the claim issuer, or the claim holder itself.
      * @param _claimId id of the claim.
      */
    function removeClaim(bytes32 _claimId) public  {
        // Only Claimer Keys
        /* require(keysStruct[keccak256(abi.encodePacked(msg.sender))].purpose == CLAIM_SIGNER_KEY, "Only CLAIM signer Key can add Claims"); */
        require((keysStruct[keccak256(abi.encodePacked(msg.sender))].purpose == CLAIM_SIGNER_KEY) || (keysStruct[keccak256(abi.encodePacked(msg.sender))].purpose == MANAGEMENT_KEY), "Only CLAIM signer Key can add Claims");

        // Claim must exist.
        require(claims[_claimId].topic > 0, "Claim must exist to be changed");

        // Remove Claim.
        delete claims[_claimId];
        for(uint i = 0; i<claimIdsByTopic[claims[_claimId].topic].length; i++){
            if(claimIdsByTopic[claims[_claimId].topic][i] == _claimId)
                delete claimIdsByTopic[claims[_claimId].topic][i];
        }
        emit ClaimRemoved(_claimId, claims[_claimId].topic, claims[_claimId].scheme, claims[_claimId].issuer, claims[_claimId].signature, claims[_claimId].data, claims[_claimId].uri);
    }

    /**
      * @dev Returns the key data, if hold by the identity.
      * @param _key keccack256 of original key.
      * @return Purpose e.g., MANAGEMENT_KEY = 1, ACTION_KEY = 2, CLAIM_SIGNER_KEY = 3, ENCRYPTION_KEY = 4;
      * @return keyType e.g. 1 = ECDSA, 2 = RSA, etc.
      * @return The key.
      */
    function getKey(bytes32 _key) public view returns(uint256 purpose, uint256 keyType, bytes32 return_key) {
        bytes32 key = keccak256(abi.encodePacked(_key));
        return(keysStruct[key].purpose, keysStruct[key].keyType, keysStruct[key].key);
    }

    /**
      * @dev Returns the TRUE if a key has is present and has the given purpose. If key is not present it returns FALSE.
      * @param _key keccack256 of original key.
      * @param _purpose purpose to check.
      * @return exist True if exist, False if not.
      */
    function keyHasPurpose(bytes32 _key, uint256 _purpose) public view returns(bool exists) {
        bytes32 key = keccak256(abi.encodePacked(_key));
        return (keysStruct[key].purpose == _purpose);
    }

    /**
      * @dev Returns an array of public key bytes32 hold by this identity.
      * @param _purpose purpose to check.
      * @return keys list of keys from one purpose.
      */
    function getKeysByPurpose(uint256 _purpose) public view returns (bytes32[] memory keys) {
        return keysByPurpose[_purpose];
    }

    /** @dev Returns a claim by ID.
      * @param _claimId id of the claim.
      * @return topic is the type of claim.
      * @return scheme the estructure of the claim
      * @return issuer from who comes the claim
      * @return signature is a signe_ message of the following structure: keccak256(address subject_address, uint256 topic, bytes data).
      * @return data data of the claim if exist
      * @return uri link for more information of the claim
      */
    function getClaim(bytes32 _claimId) public view returns ( uint256 topic, uint256 scheme, address issuer, bytes32 signature, bytes32 data, bytes32 uri) {
        return (claims[_claimId].topic, claims[_claimId].scheme, claims[_claimId].issuer, claims[_claimId].signature, claims[_claimId].data, claims[_claimId].uri);
    }

    /** @dev Return a list of all claims with same topic
      * @param _topic topic we want to check
      * @return claimIds list of the claims with the same topic
      */
    function getClaimIdsByTopic( uint256 _topic) public view returns(bytes32[] memory claimIds) {
        return claimIdsByTopic[_topic];
    }
}
