import { unified } from "unified";
import remarkParse from "remark-parse";
import remarkRehype from "remark-rehype";
import rehypeStringify from "rehype-stringify";
import remarkGfm from "remark-gfm";
import remarkFrontmatter from "remark-frontmatter";
import { matter } from "vfile-matter";
import rehypeSanitize from "rehype-sanitize";
import rehypeHighlight from "rehype-highlight";
import rehypeSlug from "rehype-slug";

function remarkExtractFrontmatter() {
  return function (_tree: any, file: any) {
    matter(file);
  };
}

export function createBasicProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeStringify);
}

export function createGfmProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkRehype)
    .use(rehypeStringify);
}

export function createFrontmatterProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkFrontmatter, ["yaml"])
    .use(remarkExtractFrontmatter)
    .use(remarkRehype)
    .use(rehypeStringify);
}

export function createSanitizeProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeSanitize)
    .use(rehypeStringify);
}

export function createHighlightProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeHighlight)
    .use(rehypeStringify);
}

export function createTocProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeSlug)
    .use(rehypeStringify);
}

export function createCombinedProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkFrontmatter, ["yaml"])
    .use(remarkExtractFrontmatter)
    .use(remarkRehype)
    .use(rehypeSanitize)
    .use(rehypeHighlight)
    .use(rehypeSlug)
    .use(rehypeStringify);
}
