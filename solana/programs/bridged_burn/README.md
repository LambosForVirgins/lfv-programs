# Bridged Burn Program

A Solana program that burns tokens and sends confirmation messages to Sui via Wormhole for cross-chain bridging.

## Overview

This program implements a secure token burn mechanism that:

1. **Burns tokens** from a Solana token account
2. **Closes the token account** and transfers lamports to a vault
3. **Sends a confirmation message** to Sui via Wormhole containing burn details
4. **Emits events** for off-chain tracking

## Features

### Core Functionality

- **Token Burning**: Securely burns all tokens in a specified token account
- **Account Closure**: Closes the token account and sends rent to a vault
- **Cross-Chain Messaging**: Sends structured burn confirmation to Sui via Wormhole
- **Event Emission**: Emits detailed events for tracking and verification

### Security Features

- **Input Validation**: Ensures token account has tokens before burning
- **Structured Payloads**: Uses Borsh serialization for reliable data encoding
- **Consistent Message Format**: 113-byte payload with fixed structure
- **Error Handling**: Comprehensive error codes for different failure scenarios

## Payload Structure

The burn confirmation message sent via Wormhole has the following structure:

```
[1 byte]  - Message Type (always 1 for burn confirmation)
[32 bytes] - Sui Receiver Address  
[32 bytes] - Solana Sender Address
[32 bytes] - Token Mint Address
[8 bytes]  - Amount Burned (u64, little-endian)
[8 bytes]  - Timestamp (u64, little-endian)
```

**Total: 113 bytes**

## Usage

### Prerequisites

1. **Solana token account** with tokens to burn
2. **Sui address** (32 bytes) to receive the confirmation
3. **Vault account** to receive lamports from closed token account

### Instruction: `burn_and_close`

```rust
pub fn burn_and_close(
    ctx: Context<BurnAndClose>,
    sui_address: [u8; 32],
) -> Result<()>
```

**Parameters:**
- `sui_address`: 32-byte Sui address that will receive the burn confirmation

**Accounts:**
- `owner`: Signer and authority of the token account
- `token_account`: Token account to burn and close (must have tokens)
- `mint`: Token mint of the tokens being burned
- `vault`: Account to receive lamports from closed token account
- `token_program`: SPL Token program
- `system_program`: Solana system program

### Example Usage

```typescript
import { PublicKey } from '@solana/web3.js';
import { BN } from '@coral-xyz/anchor';

// Sui address (32 bytes)
const suiAddress = Buffer.from([
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
  0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
  0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
  0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
]);

await program.methods
  .burnAndClose(Array.from(suiAddress))
  .accounts({
    owner: ownerKeypair.publicKey,
    tokenAccount: tokenAccountAddress,
    mint: mintAddress,
    vault: vaultAddress,
    tokenProgram: TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
  })
  .signers([ownerKeypair])
  .rpc();
```

## Events

### BridgeBurnEvent

Emitted after successful burn and message preparation:

```rust
pub struct BridgeBurnEvent {
    pub sui_receiver: [u8; 32],
    pub sol_sender: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}
```

### WormholeMessageEvent

Contains the Wormhole message details:

```rust
pub struct WormholeMessageEvent {
    pub target_chain: u16,      // 21 for Sui
    pub payload: Vec<u8>,       // 113-byte payload
    pub consistency_level: u8,  // 1 for finalized
}
```

## Error Codes

| Code | Error | Description |
|------|-------|-------------|
| 6000 | `NothingToBurn` | Token account is empty |
| 6001 | `InvalidPayloadSize` | Payload size is not 113 bytes |
| 6002 | `InvalidSuiAddress` | Sui address format is invalid |
| 6003 | `WormholePostFailed` | Wormhole message posting failed |
| 6004 | `PayloadSerializationFailed` | Failed to serialize payload |

## Implementation Details

### Wormhole Integration

The program integrates with Wormhole to send cross-chain messages:

- **Target Chain**: Sui (Chain ID: 21)
- **Consistency Level**: 1 (finalized)
- **Message Format**: Structured binary payload with Borsh serialization

### Private Function: `send_burn_confirmation_message`

This private function handles the Wormhole integration:

1. Creates a structured `BurnConfirmationPayload`
2. Serializes using Borsh for consistent encoding
3. Validates payload size (must be 113 bytes)
4. Emits `WormholeMessageEvent` for off-chain relayers
5. Logs detailed information for debugging

### Payload Serialization

Uses Borsh for deterministic serialization:
- **Reproducible**: Same input always produces same output
- **Efficient**: Compact binary format
- **Cross-platform**: Compatible with other ecosystems

## Testing

The program includes comprehensive tests:

### Unit Tests

- Payload serialization/deserialization
- Different address formats
- Amount handling (including edge cases)
- Round-trip payload verification

### Integration Tests

- Full burn and close workflow
- Event emission verification
- Error condition handling
- Multiple burn scenarios

### Running Tests

```bash
# Run unit tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_burn_confirmation_payload_serialization
```

## Security Considerations

1. **Token Account Validation**: Ensures account has tokens before burning
2. **Authority Verification**: Only account owner can initiate burn
3. **Payload Integrity**: Fixed size and structure prevent manipulation
4. **Event Tracking**: All operations emit events for auditability

## Wormhole Reception on Sui

The Sui smart contract should expect the following payload structure:

```move
public struct BurnConfirmation has copy, drop {
    message_type: u8,        // Always 1
    sui_receiver: address,   // 32 bytes
    solana_sender: vector<u8>, // 32 bytes  
    mint: vector<u8>,        // 32 bytes
    amount: u64,             // 8 bytes
    timestamp: u64,          // 8 bytes
}
```

## Future Enhancements

1. **Full Wormhole CPI**: Direct integration with Wormhole Core contracts
2. **Fee Handling**: Automatic Wormhole fee calculation and payment
3. **Retry Mechanism**: Handling of failed Wormhole messages

## Dependencies

- `anchor-lang = "0.31.1"`
- `anchor-spl = "0.31.1"`
- `solana-program = "2.1.1"`
- `wormhole-anchor-sdk = "0.2.0"`
- `borsh = "0.10.3"`

## License

This program is part of the LFV ecosystem and follows the project's licensing terms. 