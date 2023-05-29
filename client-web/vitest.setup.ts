// getrandom requires custom setup for ESM support: https://docs.rs/getrandom/latest/getrandom/#nodejs-es-module-support
import { webcrypto } from "node:crypto"
;(globalThis as any).crypto = webcrypto
