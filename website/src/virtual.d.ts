declare module "virtual:translation-status" {
  import type { TranslationManifest } from "../plugins/vite-plugin-translation-status";
  const manifest: TranslationManifest;
  export default manifest;
}
