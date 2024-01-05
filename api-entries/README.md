# api-entries

API service that handles synchronization between different clients. All data is encrypted and service sees only encrypted data. 

There are two implementation - endpoints implemented as AWS Lambda and one combined REST service implemented with `actix-web` in `webservice` folder. All of them are very thin and actual logic is shared between implementation and placed in `services` to guarantee the same behavior.

Currently `api.qqself.com` is running `webservice` using `AWS App Runner` and persists data using `DynamoDB`. For local development running `webservice` is simple as `cargo run` which will use in memory storage by default, so no AWS credentials is needed.
