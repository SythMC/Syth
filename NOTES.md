# Type Sizes:
| Type | Size (bytes) | Type |
|---|---|---|
| Byte | 1 | u8 |
| Short | 2 | i16 |
| String | 64 | [u8; 64] |


# Heartbeat
TODO

# Classic Protocol

## Client → server packets

### Player Identification
When the player tries connecting to the server it will immeditely send a Player Identification packet which looks like:

| Field | Type  | Example Data |
|---|---|---|
| Packet ID | Byte | `0x00` |
| Protocol version | Byte | 7 |
| Username | String | "Pendonym" |
| Verification key | String | Can be none |
| Unused | Byte | |

## Server → client packets

### Server Identification
This is the response sent to a player joining.

| Field | Type | Example Data |
|---|---|---|
| Packet ID | Byte | `0x00` |
| Protocol version | Byte | 7 |
| Server name | String | "Syth Server" |
| Server MOTD | String | "Welcome to my server!" |
| User type | Byte | OP (0x64) or Not (0x00) |

### Ping
This should be sent to the client every ~30s to let the client know the server is still open.

| Field | Type | Example Data |
|---|---|---|
| Packet ID | Byte | `0x01` |

### Level Initialize 
Lets the client know level data is going to be sent.

| Field | Type | Example Data |
|---|---|---|
| Packet ID | Byte | `0x02` |

# Classic Protocol Extension

## Server -> client packets

### Hack Control

This allows the server to control weather the client is allowed to use specific cheats on the client.

Packet ID: 0x20

| Field | Type | Example Data | Notes |
|---|---|---|---|
| Flying | Byte | `1` | 0 = Prevent, 1 = Allow |
| NoClip | Byte | `1` | 0 = Prevent, 1 = Allow |
| Speeding | Byte | `1` | 0 = Prevent, 1 = Allow |
| SpawnControl | Byte | `1` | 0 = Prevent, 1 = Allow|
| ThirdPersonView | Byte | `1` | 0 = Prevent, 1= Allow|
| JumpHeight | Short | `40` | Maximum height, in terms of player movement units (1/32nds of a block), to which the player is allowed to jump. Negative value (e.g. `-1`) means that the client should use its default jump height.  |