# qqself

Reaching mastery takes decades, and only a few succeed. Why do people give up, and what can we do to solve it?

- qqself is a book `On The Path To Mastery`, which analyses why only a few succeed in becoming masters, what common failures occur, and how to solve these common issues. Why do a few keep their motivation all the way, but most lose it eventually? What helps us to keep going, and what prevents us from succeeding?
- qqself is also an app, an interactive journal that incorporates all the ideas from the book to mitigate the reasons why people give up and provides the motivation boost to persist in practical learning.

## core
A shared Rust core handles journal entry parsing, encryption, all metric and identity calculations. All other services and apps heavily use it either directly or through WebAssembly or via C bindings.

## core-bindings-c
C bindings for the `core`. Used by the Swift `client-ios` and in the future by Kotlin `client-android`.

## core-bindings-wasm
WebAssembly bindings for the `core`. Used by the `client-web`.

## api-sync
A Rust web service based on `actix-web` persists journal entries in `AWS DynamoDB` and allows multiple clients to synchronize. It operates with already encrypted payloads and cannot access user data.

## client-cli
A Rust command-line client works with plain text journal files and allows you to have full control of the data. It also supports importing all data from `api-sync` to the plain text journal file and exporting the journal file back to the cloud. You are free to move from or to the synchronization server at any time.

## client-web
A TypeScript/LitElement/WebAssembly web version of the journal app is served on [app.qqself.com](https://app.qqself.com). It supports most of the interactive features and handles synchronization with the cloud automatically.

## www
This contains [www.qqself.com](https://www.qqself.com) landing page and also includes a working draft of `On The Path To Mastery`, which serves as the basis for all the features in the journaling apps.
