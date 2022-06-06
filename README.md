# pft
sim**p**le **f**ile **t**ransfer server/client built to learn more about Rust, TCP and protocols.

<img src="demo.gif" height="350"/>

## Usage
Start the server with `pft server <ADDRESS>`.
Use the client with the adress to the server, the password the server generated and a path to a file to upload. `pft client <ADDRESS> <OTP> <FILE_PATH>`

E.g.:
- `pft server 0.0.0.0:3333`
- `pft client phib.io:3333 8NCtXNKp9vxPudq165Foa6mKBvmzqg testbig.zip`
## Application Layer Protocol
### Client Requests 
#### AnnounceFileTransfer
| u8   | u16                           | [u8]        |  u16    | [u8]        |
| ---- | ------ | ----------- | ---------- | --------------- |
| Request type | filename length | filename value bytes | password length | password value bytes |

#### UploadFile
| u8           | u8            |  u16   | [u8]                 | [u8]             |
| ------------ | ------------- | --- | -------------------- | ---------------- |
| Request type | transfer type |  filename length   | filename value bytes | file value bytes |

#### Request Type
- AnnounceFileTransfer = 1
- UploadFile = 2

#### Transfer Type
- Normal = 1
- Replace = 2
- KeepBoth = 3

### Server Response
#### Response
| u16   | [u8]     | u8        |
| ---- | ------ | ----------- | 
| message length | message value bytes | error type | 

#### Error Type 
- None = 0,
- InvalidPassword = 1,
- FileAlreadyExists = 2
