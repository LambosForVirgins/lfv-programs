const { Buffer } = require('buffer');
const { createVAA, serialize, UniversalAddress, encoding } = require('@wormhole-foundation/sdk');
const { mocks } = require("@wormhole-foundation/sdk-definitions/testing");

/**
 * Creates a Wormhole VAA message
 * @param {string} solanaProgramId - The Solana program ID (base58 string)
 * @param {string} suiReceiver - The Sui receiver address (hex string with 0x prefix)
 * @returns {Uint8Array} The VAA message as a Uint8Array
 */
const createVaaMessage = (solanaProgramId, suiReceiver) => {
    console.log("Address: ", Buffer.from(new UniversalAddress(solanaProgramId, 'base58').toUint8Array()).toString('hex'));
    
    let vaa = createVAA('Uint8Array', {
        guardianSet: 0,
        timestamp: Math.floor(Date.now() / 1000),
        nonce: 0,
        emitterChain: 'Solana',
        emitterAddress: new UniversalAddress(solanaProgramId, 'base58'),
        sequence: 0n,
        consistencyLevel: 0,
        signatures: [], 
        payload: encoding.bytes.encode(suiReceiver)
    });

    // Serialize the VAA
    return serialize(vaa);
}

/**
 * Converts a byte array to a hex string
 * @param {Uint8Array} bytes - The byte array to convert
 * @returns {string} The hex string representation
 */
const bytesToHex = (bytes) => {
    return Buffer.from(bytes).toString('hex');
}

/**
 * Converts a hex string to a byte array
 * @param {string} hex - The hex string to convert
 * @returns {Uint8Array} The byte array
 */
const hexToBytes = (hex) => {
    return new Uint8Array(Buffer.from(hex, 'hex'));
}

// Example usage
const solanaProgramId = "9QZ5nMuz1cH4Nb7mWwSDrXy5zMWg1DT6TSjdgga933wU";
const suiReceiver = "0x17e542b4cff0b80d6171c5e062b0bc029a88eefd46c5697044e968da7fb5e8ce";

// Create and output new VAA
const vaaMessage = createVaaMessage(solanaProgramId, suiReceiver);

const guardians = mocks.devnetGuardianSet();

console.log("Guardians: ", guardians.signers);
const signedVaa = guardians.setSignatures(vaaMessage);  

console.log("New VAA Message (hex):", bytesToHex(vaaMessage));

console.log("Signed VAA Message (hex):", bytesToHex(serialize(signedVaa)));

// Export the functions for use in other modules
module.exports = { 
    createVaaMessage,
    bytesToHex,
    hexToBytes
}; 