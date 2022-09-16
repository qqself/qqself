# qqself

Personal tracking system to support long term skill development:

- Universal language to support tracking of almost any kind of data
- Local first - all data stored on the client
- Optional end-to-end encryption synchronization - data is encrypted on a client before sending
- Shared core Rust library with all the logic e.g. parsing/encryption with Web-Assembly support
- Multiple experimental frontends - CLI/PWA and the one with gamification elements

## Research about long-term skill development

Do you know a good book/paper/article/research about long-term skill development:

- How to achieve mastery?
- What causes us to give up, what helps us keep pushing and continue?
- How to start learning new skill, overcome fear?
- How to live through plateau, when no progress is visible?

Have a look what we have in [library.md](research/library.md) and create PR with your suggestions. Thank you!

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
- `research`
    - [x] [Gamification principles](research/gamification.md)
    - [x] [Long term skill development](research/skill_development.md)
    - [ ] [Solution](research/solutions.md) - which instruments to use/how to solve common issues with long-term skill
      development
    