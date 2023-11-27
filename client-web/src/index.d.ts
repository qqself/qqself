declare module "*.png" {
  const value: string
  export = value
}
declare module "*.css" {
  const value: never
  export = value
}
declare module "*.svg" {
  const value: string
  export = value
}
declare module "*.ttf" {
  const value: string
  export = value
}
interface ImportMeta {
  readonly env: {
    DEV: boolean
    VITE_API_HOST: string
    VITE_LOG_LEVEL: string
  }
}
