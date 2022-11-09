# api-sync

API service that handles synchronisation between different clients. All data is encrypted and service sees nothing but encrypted data.

Built using Rust and `actix-web`. If you want to host your own API then you can use provided `Dockerfile` and run your service whenever you like. 

You are expected to implement `qqself_api_sync::storage::payload::PayloadStorage` trait to support your own storage implementation and supply it's in `main.rs`. 

Currently `api.qqself.com` is running using `AWS App Runner` and pertists data using `DynamoDB`
