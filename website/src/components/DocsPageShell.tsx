import type { ComponentType } from "react";

import { DocContent } from "~/components/DocContent";
import { Layout } from "~/components/Layout";
import { I18nContext, type LocaleCode } from "~/i18n";

import type { TranslationStatus } from "../../plugins/vite-plugin-translation-status";

interface PageLink {
  label: string;
  href: string;
}

interface DocsPageShellProps {
  locale: LocaleCode;
  pathname: string;
  html?: string;
  MdxContent?: ComponentType<{ components?: Record<string, ComponentType> }>;
  frontmatter: Record<string, unknown>;
  toc: Array<{ depth: number; text: string; slug: string }>;
  translationStatus?: TranslationStatus;
  slug: string;
  prevPage?: PageLink;
  nextPage?: PageLink;
}

export function DocsPageShell({
  locale,
  pathname,
  html,
  MdxContent,
  frontmatter,
  toc,
  translationStatus,
  slug,
  prevPage,
  nextPage,
}: DocsPageShellProps) {
  return (
    <I18nContext.Provider value={{ locale }}>
      <Layout locale={locale} pathname={pathname}>
        <DocContent
          html={html}
          MdxContent={MdxContent}
          frontmatter={frontmatter}
          toc={toc}
          translationStatus={translationStatus}
          slug={slug}
          prevPage={prevPage}
          nextPage={nextPage}
          locale={locale}
        />
      </Layout>
    </I18nContext.Provider>
  );
}
