declare module "*.md" {
  export const html: string;
  export const frontmatter: Record<string, unknown>;
  export const toc: Array<{ depth: number; text: string; slug: string }>;
  declare const content: {
    html: string;
    frontmatter: Record<string, unknown>;
    toc: Array<{ depth: number; text: string; slug: string }>;
  };
  export default content;
}

declare module "*.mdx" {
  export const html: string;
  export const frontmatter: Record<string, unknown>;
  export const toc: Array<{ depth: number; text: string; slug: string }>;
  declare const content: {
    html: string;
    frontmatter: Record<string, unknown>;
    toc: Array<{ depth: number; text: string; slug: string }>;
  };
  export default content;
}
