import { createFileRoute } from "@tanstack/react-router";
import type { ComponentType } from "react";
import translationStatus from "virtual:translation-status";

import { DocContent } from "~/components/DocContent";
import { NotFound } from "~/components/NotFound";
import { useTranslation, localePath, SUPPORTED_LOCALES } from "~/i18n";
import type { LocaleCode } from "~/i18n";
import type { Locale } from "~/i18n/locales/en";
import { flattenNav } from "~/navigation";

import type { TranslationStatus } from "../../../../plugins/vite-plugin-translation-status";

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

const mdModules = import.meta.glob<MdModule>("../../../../content/**/*.md");

const mdxModules = import.meta.glob<MdxModule>("../../../../content/**/*.mdx");

const allPages = flattenNav();

export const Route = createFileRoute("/$locale/docs/$")({
  loader: async ({ params }) => {
    const { locale, _splat: slug } = params;
    if (!SUPPORTED_LOCALES.includes(locale as LocaleCode)) {
      return { mdMod: null, mdxMod: null, hasLocale: false, slug: slug! };
    }
    const localeMdKey = `../../../../content/${locale}/${slug}.md`;
    const localeMdxKey = `../../../../content/${locale}/${slug}.mdx`;
    const enMdKey = `../../../../content/en/${slug}.md`;
    const enMdxKey = `../../../../content/en/${slug}.mdx`;
    const [localeMd, localeMdx, enMd, enMdx] = await Promise.all([
      mdModules[localeMdKey]?.(),
      mdxModules[localeMdxKey]?.(),
      mdModules[enMdKey]?.(),
      mdxModules[enMdxKey]?.(),
    ]);
    return {
      mdMod: localeMd ?? enMd ?? null,
      mdxMod: localeMdx ?? enMdx ?? null,
      hasLocale: Boolean(localeMd || localeMdx),
      slug: slug!,
    };
  },
  component: LocaleDocsPage,
});

function LocaleDocsPage() {
  const { mdMod, mdxMod, hasLocale, slug } = Route.useLoaderData();
  const { locale } = Route.useParams();
  const { t } = useTranslation();

  if (!SUPPORTED_LOCALES.includes(locale as LocaleCode) || (!mdMod && !mdxMod)) {
    return <NotFound />;
  }

  const status: TranslationStatus = hasLocale
    ? (translationStatus[slug]?.status ?? "translated")
    : "missing";
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
      translationStatus={status}
      slug={slug}
      prevPage={
        prevPage
          ? {
              label: t(`nav.${prevPage.labelKey}` as `nav.${keyof Locale["nav"]}`),
              href: localePath(prevPage.href, locale as LocaleCode),
            }
          : undefined
      }
      nextPage={
        nextPage
          ? {
              label: t(`nav.${nextPage.labelKey}` as `nav.${keyof Locale["nav"]}`),
              href: localePath(nextPage.href, locale as LocaleCode),
            }
          : undefined
      }
    />
  );
}
