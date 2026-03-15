import { Collapsible } from "@base-ui/react/collapsible";
import { memo } from "react";
import translationStatus from "virtual:translation-status";

import { useTranslation, localePath, DEFAULT_LOCALE, type LocaleCode } from "~/i18n";
import { NAV, type NavItem, type NavSection } from "~/navigation";

import type { TranslationStatus } from "../../../plugins/vite-plugin-translation-status";

import styles from "./Sidebar.module.css";

function ChevronIcon({ className }: { className?: string }) {
  return (
    <svg width="10" height="10" viewBox="0 0 10 10" fill="none" className={className}>
      <path d="M3.5 9L7.5 5L3.5 1" stroke="currentcolor" strokeWidth="1.5" />
    </svg>
  );
}

const StatusBadge = memo(function StatusBadge({ status }: { status?: TranslationStatus }) {
  const { t } = useTranslation();
  if (!status || status === "translated") return null;

  const label = status === "missing" ? t("i18n.untranslated") : t("i18n.outdated");
  const className = status === "missing" ? styles.badgeMissing : styles.badgeOutdated;

  return <span className={className}>{label}</span>;
});

const NavItemLink = memo(function NavItemLink({
  item,
  pathname,
}: {
  item: NavItem;
  pathname: string;
}) {
  const { t, locale } = useTranslation();
  const href = localePath(item.href, locale);
  const isActive = pathname === href;
  const status =
    locale !== DEFAULT_LOCALE && item.slug ? translationStatus[item.slug]?.status : undefined;

  return (
    <li className={styles.navItemRow}>
      <a href={href} className={isActive ? styles.navLinkActive : styles.navLink}>
        {t(`nav.${item.labelKey}`)}
      </a>
      <StatusBadge status={status} />
    </li>
  );
});

function useSortedItems(items: NavItem[]) {
  const { t } = useTranslation();
  return [...items].sort((a, b) => t(`nav.${a.labelKey}`).localeCompare(t(`nav.${b.labelKey}`)));
}

const SectionWithItems = memo(function SectionWithItems({
  section,
  pathname,
}: {
  section: NavSection;
  pathname: string;
}) {
  const { t } = useTranslation();
  const sorted = useSortedItems(section.items!);

  return (
    <Collapsible.Root defaultOpen>
      <Collapsible.Trigger className={styles.sectionTrigger}>
        {t(`nav.${section.labelKey}`)}
        <ChevronIcon className={styles.chevron} />
      </Collapsible.Trigger>
      <Collapsible.Panel className={styles.sectionPanel}>
        <ul className={styles.sectionList}>
          {sorted.map((item) => (
            <NavItemLink key={item.href} item={item} pathname={pathname} />
          ))}
        </ul>
      </Collapsible.Panel>
    </Collapsible.Root>
  );
});

const SortedGroupItems = memo(function SortedGroupItems({
  items,
  pathname,
}: {
  items: NavItem[];
  pathname: string;
}) {
  const sorted = useSortedItems(items);
  return (
    <ul className={styles.groupList}>
      {sorted.map((item) => (
        <NavItemLink key={item.href} item={item} pathname={pathname} />
      ))}
    </ul>
  );
});

const SectionWithGroups = memo(function SectionWithGroups({
  section,
  pathname,
}: {
  section: NavSection;
  pathname: string;
}) {
  const { t } = useTranslation();

  return (
    <div className={styles.groupSection}>
      <div className={styles.groupLabel}>{t(`nav.${section.labelKey}`)}</div>
      {section.groups!.map((group) => (
        <Collapsible.Root key={group.labelKey} defaultOpen>
          <Collapsible.Trigger className={styles.groupTrigger}>
            <span className={styles.groupTriggerLabel}>{t(`nav.${group.labelKey}`)}</span>
            <ChevronIcon className={styles.chevron} />
          </Collapsible.Trigger>
          <Collapsible.Panel className={styles.sectionPanel}>
            <SortedGroupItems items={group.items} pathname={pathname} />
          </Collapsible.Panel>
        </Collapsible.Root>
      ))}
    </div>
  );
});

interface SidebarProps {
  hideLogo?: boolean;
  locale?: LocaleCode;
  pathname?: string;
}

export const Sidebar = memo(function Sidebar({
  hideLogo,
  locale,
  pathname = "",
}: SidebarProps = {}) {
  useTranslation(locale);
  return (
    <nav className={styles.sidebar}>
      {!hideLogo && (
        <div className={styles.logo}>
          <a href="/">unifast</a>
        </div>
      )}
      {NAV.map((section) =>
        section.groups ? (
          <SectionWithGroups key={section.labelKey} section={section} pathname={pathname} />
        ) : (
          <SectionWithItems key={section.labelKey} section={section} pathname={pathname} />
        ),
      )}
    </nav>
  );
});
