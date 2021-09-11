# Sync-Server

Responsible for syncing data between and persisting on server. End-to-end encryption - there is no way server can read data in any way

## Description

Encryption happens on a client using RSA-OAEP - similar that used in SSH with public/private keys. Among other tools it's supported by web as well via  
https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto.

Each client has public and private key and while public key transferred to server - private key never leaves the client and server will never receive it, which means it will never be able to decrypt the data even in case if it got compromised.

## API

#### POST /sync

Request
```
{
  client_id: [UUID],          // Random UUID which identifies particular client
  operation_index: [Number]   // Each client starts with 0 index and increments it on every operation creted on a client
  sync_status: {              // Data about other clients and last operation_index that this client is aware about 
    [UUID]: [Number],
    [UUID]: [Number],
    ...
  },
  user: [PUBLIC_KEY],         // User public key
  payload?: [ENCRYPTED_DATA], // Encrypted operation. May be empty if no operation was creted but client just want to sync and fetch latest data from server
}  
```
Response is using JSON streaming, basically response contains lines separated by `\n` where each line is a JSON document of structure:
```
{
  client_id: [UUID],         // Client ID where operation was created
  operation_index: [Number], // Client ID operation index that was assigned to this operation
  payload: [ENCRYPTED_DATA], // Encrypted operation 
}  
```

#### Implementation

For each received operation we first persist payload on durable storage (S3 for now) with name pattern like:
`[PUBLIC_KEY].[CLIENT_ID].[OPERATION_INDEX].[PAYLOAD_MD5_HASH]`. Then we fetch all existing operation for given `PUBLIC_KEY`. Operation skipped if `CLIENT_ID` and `OPERATION_INDEX` less than one specified in `sync_status` or in request itself in `client_id` and `operation_index` fields.  

#### Security

- Privacy: 
  - As private key never leaves the client it's not possible for server to know what payload actually contains. Even in case of server being compromised there is no way for it to decrypt the data
- DDOS:
  - In case server being compromised it may send garbage to the client - although such payload will be ignored as public key check will fail.
  - In case public key leaks from client it's possible generate a lot of new client_id, re-send existing operation, fetch all existing operations. It will be mitigated by rate limiting for each user

#### Open questions

- Basically it is Basic Authentication with extra steps. While everything goes through HTTPS risk is the same as with basic auth
- Querying S3 for many small objects is rather expansive operation and cache may dramatically reduce total costs and latency 
