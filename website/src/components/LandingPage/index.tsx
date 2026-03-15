import { Avatar } from "@base-ui/react/avatar";
import { Tabs } from "@base-ui/react/tabs";
import { ArrowRight01Icon } from "hugeicons-react";

import { CopyButton } from "~/components/CopyButton";
import { Footer } from "~/components/Footer";
import { GitHubIcon } from "~/components/GitHubIcon";
import { LanguageSwitcher } from "~/components/LanguageSwitcher";
import { MobileMenu } from "~/components/MobileMenu";
import { PackageInstall } from "~/components/PackageInstall";
import { SearchDialog } from "~/components/SearchDialog";
import { ThemeToggle } from "~/components/ThemeToggle";
import sponsorsJson from "~/data/sponsors.json";
import { I18nContext, useTranslation, localePath, type LocaleCode } from "~/i18n";

import styles from "./LandingPage.module.css";

interface Sponsor {
  login: string;
  avatarUrl: string;
  url: string;
}

const sponsors = sponsorsJson as Sponsor[];

const BENCH_TABS = [
  {
    id: "basic",
    label: "Basic",
    rows: [
      { label: "120 lines", unifast: 0.18, unified: 4.78 },
      { label: "560 lines", unifast: 0.41, unified: 11.22 },
      { label: "2,000 lines", unifast: 1.2, unified: 31.51 },
    ],
  },
  {
    id: "plugins",
    label: "Plugins",
    rows: [
      { label: "GFM", unifast: 0.42, unified: 16.82 },
      { label: "Frontmatter", unifast: 0.46, unified: 11.53 },
      { label: "Math", unifast: 0.43, unified: 11.46 },
      { label: "Sanitize", unifast: 0.42, unified: 11.73 },
      { label: "Highlighting", unifast: 2.28, unified: 18.49 },
    ],
  },
] as const;

const k = "sy-keyword";
const s = "sy-string";
const c = "sy-comment";
const f = "sy-variable sy-function";
const n = "sy-constant sy-numeric";
const p = "sy-punctuation";

function h(cls: string, text: string): string {
  return `<span class="${cls}">${text}</span>`;
}

function wrapLines(lines: string[]): string {
  return lines
    .map((line, i) => `<span class="line" data-line="${i + 1}">${line}</span>`)
    .join("\n");
}

function stripHtml(html: string): string {
  return html
    .replaceAll(/<[^>]+>/g, "")
    .replaceAll("&lt;", "<")
    .replaceAll("&gt;", ">")
    .replaceAll("&amp;", "&");
}

const SHOWCASES = [
  {
    id: "compile",
    label: "Compile",
    html: wrapLines([
      `${h(k, "import")} ${h(p, "{")} ${h(f, "compile")} ${h(p, "}")} ${h(k, "from")} ${h(s, '"@unifast/node"')}${h(p, ";")}`,
      "",
      `${h(k, "const")} result = ${h(f, "compile")}${h(p, "(")}${h(s, String.raw`"# Hello\n\nThis is **unifast**."`)}${h(p, ")")}${h(p, ";")}`,
      "",
      `${h(f, "console")}.${h(f, "log")}${h(p, "(")}result.output${h(p, ")")}${h(p, ";")}`,
      `${h(c, '// &lt;h1 id="hello"&gt;Hello&lt;/h1&gt;')}`,
      `${h(c, "// &lt;p&gt;This is &lt;strong&gt;unifast&lt;/strong&gt;.&lt;/p&gt;")}`,
    ]),
  },
  {
    id: "plugins",
    label: "Plugins",
    html: wrapLines([
      `${h(k, "import")} ${h(p, "{")} ${h(f, "compile")}${h(p, ",")} ${h(f, "gfm")}${h(p, ",")} ${h(f, "frontmatter")}${h(p, ",")} ${h(f, "toc")}${h(p, ",")} ${h(f, "syntect")} ${h(p, "}")} ${h(k, "from")} ${h(s, '"@unifast/node"')}${h(p, ";")}`,
      "",
      `${h(k, "const")} result = ${h(f, "compile")}${h(p, "(")}markdown${h(p, ",")} ${h(p, "{")}`,
      `  plugins${h(p, ":")} ${h(p, "[")}`,
      `    ${h(f, "frontmatter")}${h(p, "()")}${h(p, ",")}`,
      `    ${h(f, "gfm")}${h(p, "()")}${h(p, ",")}`,
      `    ${h(f, "toc")}${h(p, "(")}${h(p, "{")} maxDepth${h(p, ":")} ${h(n, "3")} ${h(p, "}")}${h(p, ")")}${h(p, ",")}`,
      `    ${h(f, "syntect")}${h(p, "()")}${h(p, ",")}`,
      `  ${h(p, "]")}${h(p, ",")}`,
      `${h(p, "}")}${h(p, ")")}${h(p, ";")}`,
    ]),
  },
  {
    id: "frontmatter",
    label: "Frontmatter",
    html: wrapLines([
      `${h(k, "import")} ${h(p, "{")} ${h(f, "compile")}${h(p, ",")} ${h(f, "frontmatter")} ${h(p, "}")} ${h(k, "from")} ${h(s, '"@unifast/node"')}${h(p, ";")}`,
      "",
      `${h(k, "const")} md = ${h(s, "`---")}`,
      `${h(s, "title: My Post")}`,
      `${h(s, "date: 2025-01-15")}`,
      `${h(s, "tags: [unifast, markdown]")}`,
      `${h(s, "---")}`,
      `${h(s, "# Hello`")}${h(p, ";")}`,
      "",
      `${h(k, "const")} result = ${h(f, "compile")}${h(p, "(")}md${h(p, ",")} ${h(p, "{")} plugins${h(p, ":")} ${h(p, "[")}${h(f, "frontmatter")}${h(p, "()")}${h(p, "]")} ${h(p, "}")}${h(p, ")")}${h(p, ";")}`,
      `result.frontmatter ${h(c, '// { title: "My Post", date: "2025-01-15", ... }')}`,
    ]),
  },
  {
    id: "react",
    label: "React",
    html: wrapLines([
      `${h(k, "import")} ${h(p, "{")} ${h(f, "compile")} ${h(p, "}")} ${h(k, "from")} ${h(s, '"@unifast/node"')}${h(p, ";")}`,
      `${h(k, "import")} ${h(p, "{")} ${h(f, "hastToReact")} ${h(p, "}")} ${h(k, "from")} ${h(s, '"@unifast/react"')}${h(p, ";")}`,
      "",
      `${h(k, "const")} result = ${h(f, "compile")}${h(p, "(")}source${h(p, ",")} ${h(p, "{")} outputKind${h(p, ":")} ${h(s, '"hast"')} ${h(p, "}")}${h(p, ")")}${h(p, ";")}`,
      "",
      `${h(k, "function")} ${h(f, "Post")}${h(p, "()")} ${h(p, "{")}`,
      `  ${h(k, "return")} ${h(f, "hastToReact")}${h(p, "(")}result.hast${h(p, ",")} ${h(p, "{")}`,
      `    components${h(p, ":")} ${h(p, "{")}`,
      `      code${h(p, ":")} CodeBlock${h(p, ",")}`,
      `      a${h(p, ":")} ${h(p, "(")}props${h(p, ")")} ${h(k, "=&gt;")} ${h(p, "&lt;")}a target=${h(s, '"_blank"')} ${h(p, "{")}...props${h(p, "}")} ${h(p, "/&gt;")}${h(p, ",")}`,
      `    ${h(p, "}")}${h(p, ",")}`,
      `  ${h(p, "}")}${h(p, ")")}${h(p, ";")}`,
      `${h(p, "}")}`,
    ]),
  },
  {
    id: "mdx",
    label: "MDX",
    html: wrapLines([
      `${h(k, "import")} ${h(p, "{")} ${h(f, "compile")} ${h(p, "}")} ${h(k, "from")} ${h(s, '"@unifast/node"')}${h(p, ";")}`,
      `${h(k, "import")} ${h(p, "{")} ${h(f, "compileToReact")} ${h(p, "}")} ${h(k, "from")} ${h(s, '"@unifast/react"')}${h(p, ";")}`,
      "",
      `${h(k, "const")} mdx = ${h(s, "`")}`,
      `${h(s, "# Interactive Docs")}`,
      `${h(s, "")}`,
      `${h(s, '&lt;Alert type="warning"&gt;A live component!&lt;/Alert&gt;')}`,
      `${h(s, "")}`,
      `${h(s, "Some **mixed** content.")}`,
      `${h(s, "`")}${h(p, ";")}`,
      "",
      `${h(k, "const")} result = ${h(f, "compile")}${h(p, "(")}mdx${h(p, ",")} ${h(p, "{")} inputKind${h(p, ":")} ${h(s, '"mdx"')} ${h(p, "}")}${h(p, ")")}${h(p, ";")}`,
      `${h(k, "const")} Content = ${h(f, "compileToReact")}${h(p, "(")}result${h(p, ")")}${h(p, ";")}`,
    ]),
  },
  {
    id: "highlight",
    label: "Syntax Highlighting",
    html: wrapLines([
      `${h(k, "import")} ${h(p, "{")} ${h(f, "compile")}${h(p, ",")} ${h(f, "syntect")}${h(p, ",")} ${h(f, "treeSitter")} ${h(p, "}")} ${h(k, "from")} ${h(s, '"@unifast/node"')}${h(p, ";")}`,
      "",
      `${h(c, "// syntect: Rust-native, fast startup")}`,
      `${h(k, "const")} a = ${h(f, "compile")}${h(p, "(")}md${h(p, ",")} ${h(p, "{")} plugins${h(p, ":")} ${h(p, "[")}${h(f, "syntect")}${h(p, "()")}${h(p, "]")} ${h(p, "}")}${h(p, ")")}${h(p, ";")}`,
      "",
      `${h(c, "// tree-sitter: precise grammars, broad language support")}`,
      `${h(k, "const")} b = ${h(f, "compile")}${h(p, "(")}md${h(p, ",")} ${h(p, "{")} plugins${h(p, ":")} ${h(p, "[")}${h(f, "treeSitter")}${h(p, "()")}${h(p, "]")} ${h(p, "}")}${h(p, ")")}${h(p, ";")}`,
    ]),
  },
  {
    id: "gfm",
    label: "GFM",
    html: wrapLines([
      `${h(k, "import")} ${h(p, "{")} ${h(f, "compile")}${h(p, ",")} ${h(f, "gfm")} ${h(p, "}")} ${h(k, "from")} ${h(s, '"@unifast/node"')}${h(p, ";")}`,
      "",
      `${h(k, "const")} md = ${h(s, "`| Feature | Status |")}`,
      `${h(s, "|---------|--------|")}`,
      `${h(s, "| Tables  | Yes    |")}`,
      `${h(s, "")}`,
      `${h(s, "- [x] Task complete")}`,
      `${h(s, "- [ ] Task pending")}`,
      `${h(s, "")}`,
      `${h(s, "~~strikethrough~~`")}${h(p, ";")}`,
      "",
      `${h(k, "const")} result = ${h(f, "compile")}${h(p, "(")}md${h(p, ",")} ${h(p, "{")} plugins${h(p, ":")} ${h(p, "[")}${h(f, "gfm")}${h(p, "()")}${h(p, "]")} ${h(p, "}")}${h(p, ")")}${h(p, ";")}`,
    ]),
  },
];

const FEATURES = [
  {
    emoji: "\u{1F980}",
    title: "Rust-Powered",
    description: "Core compiler written in Rust with zero-copy parsing and arena allocation.",
  },
  {
    emoji: "\u{1F50B}",
    title: "Batteries Included",
    description:
      "20+ built-in plugins: GFM, math, emoji, syntax highlighting, frontmatter, and more.",
  },
  {
    emoji: "\u{1F308}",
    title: "Dual Highlight Engines",
    description: "Tree-sitter and syntect built in. Up to 8x faster than rehype-highlight.",
  },
  {
    emoji: "\u{269B}\u{FE0F}",
    title: "MDX Support",
    description: "First-class MDX compilation with JSX and expression support.",
  },
] as const;

function BenchmarkBars({
  rows,
}: {
  rows: ReadonlyArray<{ label: string; unifast: number; unified: number }>;
}) {
  const maxTime = Math.max(...rows.map((r) => r.unified));
  return (
    <div className={styles.benchmarks}>
      {rows.map((b) => (
        <div key={b.label} className={styles.benchRow}>
          <span className={styles.benchLabel}>{b.label}</span>
          <div className={styles.benchBars}>
            <div className={styles.benchBarGroup}>
              <div
                className={styles.benchBarUnifastTrack}
                style={{ width: `${(b.unifast / maxTime) * 100}%` }}
              >
                <div className={styles.benchBarUnifastFill} />
              </div>
              <span className={styles.benchValue}>{b.unifast}ms</span>
            </div>
            <div className={styles.benchBarGroup}>
              <div
                className={styles.benchBarUnifiedTrack}
                style={{ width: `${(b.unified / maxTime) * 100}%` }}
              >
                <div className={styles.benchBarUnifiedFill} />
              </div>
              <span className={styles.benchValue}>{b.unified}ms</span>
            </div>
          </div>
          <span className={styles.benchMultiplier}>{Math.round(b.unified / b.unifast)}x</span>
        </div>
      ))}
      <div className={styles.benchLegend}>
        <span className={styles.legendUnifastDot} />
        <span>unifast</span>
        <span className={styles.legendUnifiedDot} />
        <span>unified + remark + rehype</span>
      </div>
    </div>
  );
}

function BenchmarkSection() {
  return (
    <section className={styles.section}>
      <div className={styles.sectionInner}>
        <h2 className={styles.sectionTitle}>Benchmark</h2>
        <p className={styles.sectionDescription}>
          Markdown to HTML compilation time (ms). Lower is better.
        </p>
        <Tabs.Root defaultValue={BENCH_TABS[0].id}>
          <Tabs.List className={styles.benchTabs}>
            {BENCH_TABS.map((t) => (
              <Tabs.Tab key={t.id} value={t.id} className={styles.benchTab}>
                {t.label}
              </Tabs.Tab>
            ))}
          </Tabs.List>
          {BENCH_TABS.map((t) => (
            <Tabs.Panel key={t.id} value={t.id}>
              <BenchmarkBars rows={t.rows} />
            </Tabs.Panel>
          ))}
        </Tabs.Root>
      </div>
    </section>
  );
}

function ShowcaseSection() {
  return (
    <section className={styles.section}>
      <div className={styles.sectionInner}>
        <h2 className={styles.sectionTitle}>The APIs you need. Built in.</h2>
        <Tabs.Root defaultValue={SHOWCASES[0].id}>
          <Tabs.List className={styles.showcasePills}>
            {SHOWCASES.map((sc) => (
              <Tabs.Tab key={sc.id} value={sc.id} className={styles.showcasePill}>
                {sc.label}
              </Tabs.Tab>
            ))}
          </Tabs.List>
          {SHOWCASES.map((sc) => (
            <Tabs.Panel key={sc.id} value={sc.id} className={styles.showcaseCode}>
              <pre>
                <code dangerouslySetInnerHTML={{ __html: sc.html }} />
              </pre>
              <CopyButton text={stripHtml(sc.html)} />
            </Tabs.Panel>
          ))}
        </Tabs.Root>
      </div>
    </section>
  );
}

export function LandingPage({ locale }: { locale: LocaleCode }) {
  useTranslation(locale);

  return (
    <I18nContext.Provider value={{ locale }}>
      <div className={styles.page}>
        <div className={styles.headerBar}>
          <header className={styles.header}>
            <a href={localePath("/", locale)} className={styles.logo}>
              unifast
            </a>
            <div className={styles.headerActions}>
              <SearchDialog />
              <span className={styles.desktopOnly}>
                <LanguageSwitcher />
              </span>
              <span className={styles.desktopOnly}>
                <ThemeToggle />
              </span>
              <span className={styles.desktopOnly}>
                <a
                  href="https://github.com/kenzo-pj/unifast"
                  target="_blank"
                  rel="noopener noreferrer"
                  className={styles.githubLink}
                  aria-label="GitHub"
                >
                  <GitHubIcon size={20} />
                </a>
              </span>
              <MobileMenu />
            </div>
          </header>
        </div>

        <section className={styles.hero}>
          <h1 className={styles.title}>
            Blazing <span className={styles.accent}>fast</span> Markdown compiler
          </h1>
          <p className={styles.subtitle}>
            High-performance Markdown / MDX compiler built with Rust.
            <br />
            Up to <strong>25x faster</strong> than unified.
          </p>
          <div className={styles.actions}>
            <a
              href={localePath("/docs/introduction/what-is-unifast/", locale)}
              className={styles.primaryBtn}
            >
              Get Started
            </a>
            <a
              href="https://github.com/kenzo-pj/unifast"
              target="_blank"
              rel="noopener noreferrer"
              className={styles.secondaryBtn}
            >
              <GitHubIcon size={16} />
              GitHub
            </a>
          </div>
          <div className={styles.install}>
            <PackageInstall package="@unifast/node" />
          </div>
        </section>

        <BenchmarkSection />

        <ShowcaseSection />

        <section className={styles.section}>
          <div className={styles.sectionInner}>
            <h2 className={styles.sectionTitle}>Features</h2>
            <div className={styles.features}>
              {FEATURES.map((feat) => (
                <div key={feat.title} className={styles.featureCard}>
                  <h3 className={styles.featureTitle}>
                    <span className={styles.featureEmoji}>{feat.emoji}</span>
                    {feat.title}
                  </h3>
                  <p className={styles.featureDescription}>{feat.description}</p>
                </div>
              ))}
            </div>
          </div>
        </section>

        {sponsors.length > 0 && (
          <section className={styles.section}>
            <div className={styles.sectionInner}>
              <h2 className={styles.sectionTitle}>Sponsors</h2>
              <div className={styles.sponsors}>
                {sponsors.map((sp) => (
                  <a
                    key={sp.login}
                    href={sp.url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className={styles.sponsorLink}
                    title={sp.login}
                  >
                    <Avatar.Root className={styles.sponsorAvatar}>
                      <Avatar.Image src={sp.avatarUrl} alt={sp.login} width={48} height={48} />
                      <Avatar.Fallback>{sp.login.slice(0, 2).toUpperCase()}</Avatar.Fallback>
                    </Avatar.Root>
                  </a>
                ))}
              </div>
            </div>
          </section>
        )}

        <section className={styles.cta}>
          <div className={styles.ctaInner}>
            <h2 className={styles.ctaTitle}>Ready to compile?</h2>
            <p className={styles.ctaDescription}>
              Explore the documentation or dive into the API reference.
            </p>
            <div className={styles.ctaCards}>
              <a
                href={localePath("/docs/introduction/what-is-unifast/", locale)}
                className={styles.ctaCard}
              >
                <span className={styles.ctaCardText}>
                  <span className={styles.ctaCardTitle}>Documentation</span>
                  <span className={styles.ctaCardDescription}>
                    Learn how to install and use unifast in your project.
                  </span>
                </span>
                <ArrowRight01Icon size={16} className={styles.ctaCardArrow} />
              </a>
              <a
                href={localePath("/docs/packages/node/overview/", locale)}
                className={styles.ctaCard}
              >
                <span className={styles.ctaCardText}>
                  <span className={styles.ctaCardTitle}>API Reference</span>
                  <span className={styles.ctaCardDescription}>
                    Explore @unifast/node APIs, plugins, and options.
                  </span>
                </span>
                <ArrowRight01Icon size={16} className={styles.ctaCardArrow} />
              </a>
            </div>
          </div>
        </section>

        <Footer />
      </div>
    </I18nContext.Provider>
  );
}
