import type { ComponentType } from "react";

type MdxDocModule = {
  default: ComponentType<{ components?: Record<string, ComponentType> }>;
  frontmatter?: Record<string, unknown>;
  toc?: Array<{ depth: number; text: string; slug: string }>;
};

const mdxDocModules = import.meta.glob("../content/**/*.mdx");

export function resolveDocMdxModulePath(relativePath: string): string | undefined {
  const modulePath = `../content/${relativePath}`;
  return modulePath in mdxDocModules ? modulePath : undefined;
}

export async function loadDocMdxModule(modulePath: string): Promise<MdxDocModule> {
  const loader = mdxDocModules[modulePath];
  if (!loader) {
    throw new Error(`Unknown MDX document module: ${modulePath}`);
  }

  return (await loader()) as MdxDocModule;
}
