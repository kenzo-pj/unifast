import { Link } from "@tanstack/react-router";

import { Button } from "~/components/Button";
import { useTranslation } from "~/i18n";

import styles from "./NotFound.module.css";

export function NotFound() {
  const { t } = useTranslation();

  return (
    <div className={styles.container}>
      <div className={styles.code}>404</div>
      <p className={styles.message}>{t("notFound.message")}</p>
      <Button render={<Link to="/" />}>{t("notFound.backHome")}</Button>
    </div>
  );
}
