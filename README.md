# qqself

Local first personal tracking system. Idea inspired by [Qantified Self](https://quantifiedself.com/) movement, while implementation idea came from paper [Local-First Software:
You Own Your Data, in spite of the Cloud](https://martin.kleppmann.com/papers/local-first.pdf)

## Features

**Privacy first** 

All data stored on a client. No one has access to your data

**Client synchronization with end-to-end encryption** 

It's convenient to have multiple clients synchronization e.g. enter data on mobile and view it on desktop. To support it while maintaining privacy all synchronization happens via zero-trust server: All data is encrypted on a client using RSA-OAEP and only then send to server. There is no way for server to read that data even if it becomes compromised.

**Flexibility**

To support tracking any kind of data new language is defined which is easy to enter and parse, examples:
```
# Simple value: [tag] [property] [value]
exercise pullups 20

# Multiple properties. Include optional `=` for better readability
run pace=6:30 distance=9.8

# Multiple tags can be entered together
qqself scope=client. development lang=rust

# Add optional comment at the end
eat carbs=200g. Coudn't resist pizza  
```
Reference parser is implemented in Rust and shared between many platforms. For example PWA is using that via WebAssembly. 

**Multiple clients**

Currently, we have console and PWA clients. We have tools to support more custom clients and highly encourage that: qqself is a personal tracking system, and only you know what works best for you. We have tools to support custom clients: Shared Rust core and reference implementations. 

**No lock in**

It's open source, language is simple to parse, sync server is easy to replicate. Maybe just use the language and keep the data in plain text files.

## Status

Proof of concept. While language and parser is defined most of the work happening right now around sync server and PWA functionality
