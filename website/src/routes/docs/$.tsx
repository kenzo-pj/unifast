import { createFileRoute } from "@tanstack/react-router";
import type { ComponentType } from "react";

import { DocContent } from "~/components/DocContent";
import { NotFound } from "~/components/NotFound";
import { useTranslation, localePath } from "~/i18n";
import type { Locale } from "~/i18n/locales/en";
import { flattenNav } from "~/navigation";

type MdModule = {
  html: string;
  frontmatter: Record<string, unknown>;
  toc: Array<{ depth: number; text: string; slug: string }>;
};

type MdxModule = {
  default: ComponentType;
  frontmatter: Record<string, unknown>;
  toc: Array<{ depth: number; text: string; slug: string }>;
};

const mdModules = import.meta.glob<MdModule>("../../../content/en/**/*.md");

const mdxModules = import.meta.glob<MdxModule>("../../../content/en/**/*.mdx");

const allPages = flattenNav();

export const Route = createFileRoute("/docs/$")({
  loader: async ({ params }) => {
    const slug = params._splat!;
    const mdKey = `../../../content/en/${slug}.md`;
    const mdxKey = `../../../content/en/${slug}.mdx`;
    const [mdMod, mdxMod] = await Promise.all([mdModules[mdKey]?.(), mdxModules[mdxKey]?.()]);
    return { mdMod: mdMod ?? null, mdxMod: mdxMod ?? null, slug };
  },
  component: DocsPage,
});

function DocsPage() {
  const { mdMod, mdxMod, slug } = Route.useLoaderData();
  const { t, locale } = useTranslation();

  if (!mdMod && !mdxMod) {
    return <NotFound />;
  }

  const frontmatter = mdxMod?.frontmatter ?? mdMod?.frontmatter ?? {};
  const toc = mdxMod?.toc ?? mdMod?.toc ?? [];
  const currentIndex = allPages.findIndex((p) => p.slug === slug);
  const prevPage = currentIndex > 0 ? allPages[currentIndex - 1] : undefined;
  const nextPage = currentIndex < allPages.length - 1 ? allPages[currentIndex + 1] : undefined;

  return (
    <DocContent
      html={mdMod?.html}
      MdxContent={mdxMod?.default}
      frontmatter={frontmatter}
      toc={toc}
      slug={slug}
      prevPage={
        prevPage
          ? {
              label: t(`nav.${prevPage.labelKey}` as `nav.${keyof Locale["nav"]}`),
              href: localePath(prevPage.href, locale),
            }
          : undefined
      }
      nextPage={
        nextPage
          ? {
              label: t(`nav.${nextPage.labelKey}` as `nav.${keyof Locale["nav"]}`),
              href: localePath(nextPage.href, locale),
            }
          : undefined
      }
    />
  );
}
