import { createFileRoute } from "@tanstack/react-router";
import { LandingPage } from "~/components/LandingPage";
import { NotFound } from "~/components/NotFound";
import { SUPPORTED_LOCALES, type LocaleCode } from "~/i18n";

export const Route = createFileRoute("/$locale/")({
  component: LocaleLandingPage,
});

function LocaleLandingPage() {
  const { locale } = Route.useParams();

  if (!SUPPORTED_LOCALES.includes(locale as LocaleCode)) {
    return <NotFound />;
  }

  return <LandingPage />;
}
