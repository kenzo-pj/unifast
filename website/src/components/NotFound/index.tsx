import { Button } from "~/components/Button";
import { useTranslation, type LocaleCode } from "~/i18n";

import styles from "./NotFound.module.css";

export function NotFound({ locale }: { locale?: LocaleCode } = {}) {
  const { t } = useTranslation(locale);

  return (
    <div className={styles.container}>
      <div className={styles.code}>404</div>
      <p className={styles.message}>{t("notFound.message")}</p>
      <Button render={<a href="/" aria-label={t("notFound.backHome")} />}>
        {t("notFound.backHome")}
      </Button>
    </div>
  );
}
