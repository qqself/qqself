declare module "*.png" {
  const value: any
  export = value
}
declare module "*.css" {
  const value: any
  export = value
}
interface ImportMeta {
  readonly env: {
    DEV: boolean
    VITE_API_HOST: string
  }
}
