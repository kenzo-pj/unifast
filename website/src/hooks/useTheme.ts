import { useState, useEffect, useCallback } from "react";

type ThemeMode = "light" | "dark" | "system";
type ResolvedTheme = "light" | "dark";

function getSystemTheme(): ResolvedTheme {
  return globalThis.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

function applyTheme(resolved: ResolvedTheme) {
  if (resolved === "dark") {
    document.documentElement.dataset.theme = "dark";
  } else {
    delete document.documentElement.dataset.theme;
  }
}

export function useTheme() {
  const [mode, setMode] = useState<ThemeMode>("system");
  const [resolved, setResolved] = useState<ResolvedTheme>("light");

  useEffect(() => {
    const stored = localStorage.getItem("theme");
    const initialMode: ThemeMode = stored === "light" || stored === "dark" ? stored : "system";
    setMode(initialMode);
    const initialResolved = initialMode === "system" ? getSystemTheme() : initialMode;
    setResolved(initialResolved);
    applyTheme(initialResolved);
  }, []);

  useEffect(() => {
    if (mode !== "system") return;
    const mq = globalThis.matchMedia("(prefers-color-scheme: dark)");
    const handler = (e: MediaQueryListEvent) => {
      const newResolved = e.matches ? "dark" : "light";
      setResolved(newResolved);
      applyTheme(newResolved);
    };
    mq.addEventListener("change", handler);
    return () => mq.removeEventListener("change", handler);
  }, [mode]);

  const cycle = useCallback(() => {
    setMode((prev) => {
      const next: ThemeMode = prev === "system" ? "light" : prev === "light" ? "dark" : "system";
      if (next === "system") {
        localStorage.removeItem("theme");
      } else {
        localStorage.setItem("theme", next);
      }
      const newResolved = next === "system" ? getSystemTheme() : next;
      setResolved(newResolved);
      applyTheme(newResolved);
      return next;
    });
  }, []);

  return { mode, resolved, cycle };
}
