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
import rehypeShiki from "@shikijs/rehype";
import rehypeExternalLinks from "rehype-external-links";
import rehypeAutolinkHeadings from "rehype-autolink-headings";
import remarkSmartypants from "remark-smartypants";
import remarkEmoji from "remark-emoji";
import remarkBreaks from "remark-breaks";
import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import remarkGithubAlert from "remark-github-blockquote-alert";
import remarkDirective from "remark-directive";
import remarkWikiLink from "remark-wiki-link";
import remarkAbbr from "remark-abbr";
import rehypeRemoveComments from "rehype-remove-comments";
import rehypeClassNames from "rehype-class-names";
import rehypePresetMinify from "rehype-preset-minify";
import readingTimeFn from "reading-time";
import { rehypeAccessibleEmojis } from "rehype-accessible-emojis";
import rehypeImageNativeLazyLoading from "rehype-plugin-image-native-lazy-loading";
import rehypeFigure from "rehype-figure";

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

export async function createShikiProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeShiki, {
      theme: "github-dark",
      langs: ["javascript", "typescript", "rust", "bash", "json", "html", "css", "python", "yaml", "toml"],
    })
    .use(rehypeStringify);
}

export function createExternalLinksProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeExternalLinks, { rel: ["nofollow", "noopener", "noreferrer"] })
    .use(rehypeStringify);
}

export function createAutolinkHeadingsProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeSlug)
    .use(rehypeAutolinkHeadings, { behavior: "prepend" })
    .use(rehypeStringify);
}

export function createSmartypantsProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkSmartypants)
    .use(remarkRehype)
    .use(rehypeStringify);
}

export function createEmojiProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkEmoji)
    .use(remarkRehype)
    .use(rehypeStringify);
}

export function createBreaksProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkBreaks)
    .use(remarkRehype)
    .use(rehypeStringify);
}

export function createMathProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkMath)
    .use(remarkRehype)
    .use(rehypeKatex)
    .use(rehypeStringify);
}

export function createGithubAlertProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkGithubAlert)
    .use(remarkRehype)
    .use(rehypeStringify);
}

export function createDirectiveProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkDirective)
    .use(remarkRehype)
    .use(rehypeStringify);
}

export function createWikiLinkProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkWikiLink)
    .use(remarkRehype)
    .use(rehypeStringify);
}

export function createAbbrProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkAbbr)
    .use(remarkRehype)
    .use(rehypeStringify);
}

export function createCommentRemovalProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeRemoveComments)
    .use(rehypeStringify);
}

export function createAddClassesProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeClassNames, { "h1": "title", "p": "text", "a": "link" })
    .use(rehypeStringify);
}

export function createMinifyProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypePresetMinify)
    .use(rehypeStringify);
}

export function createReadingTimeProcessor() {
  const processor = unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeStringify);
  return {
    processSync(md: string) {
      const result = processor.processSync(md);
      readingTimeFn(md);
      return result;
    },
  };
}

export function createAccessibleEmojiProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeAccessibleEmojis)
    .use(rehypeStringify);
}

export function createImgLazyLoadingProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeImageNativeLazyLoading)
    .use(rehypeStringify);
}

export function createFigureProcessor() {
  return unified()
    .use(remarkParse)
    .use(remarkRehype)
    .use(rehypeFigure)
    .use(rehypeStringify);
}
