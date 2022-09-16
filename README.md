# qqself

Personal tracking system to support long term skill development:

- Universal language to support tracking of almost any kind of data
- Local first - all data stored on the client
- Optional end-to-end encryption synchronization - data is encrypted on a client before sending
- Shared core Rust library with all the logic e.g. parsing/encryption with Web-Assembly support
- Multiple experimental frontends - CLI/PWA and the one with gamification elements

## Status

- `core`
    - [x] Language parser
    - [x] Encryption
- `api-sync`
    - [x] API defined and implemented with `actix-web`
    - [x] Store/retrieve encrypted data
    - [x] End-to-end encryption with RSA+AES
    - [ ] Deploy to AWS
- `client-game`
    - [x] Working prototype with Web-Assembly, web components, lit-element
    - [ ] Rewrite prototype with more flexible widgets support
    - [ ] UI/UX, design, mobile-friendly
- `skill development research`
    - [x] Gamification principles
    - [x] Long term skill development and main issues
    - [ ] List main issues and main instruments to mitigate it