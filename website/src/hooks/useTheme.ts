import { useState, useEffect, useCallback } from "react";

type ThemeMode = "light" | "dark" | "system";
type ResolvedTheme = "light" | "dark";

function getSystemTheme(): ResolvedTheme {
  if (globalThis.window === undefined) return "light";
  return globalThis.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

function getStoredMode(): ThemeMode {
  if (globalThis.window === undefined) return "system";
  const stored = localStorage.getItem("theme");
  if (stored === "light" || stored === "dark") return stored;
  return "system";
}

function applyTheme(resolved: ResolvedTheme) {
  if (resolved === "dark") {
    document.documentElement.dataset.theme = "dark";
  } else {
    delete document.documentElement.dataset.theme;
  }
}

export function useTheme() {
  const [mode, setMode] = useState<ThemeMode>(getStoredMode);
  const [resolved, setResolved] = useState<ResolvedTheme>(() =>
    mode === "system" ? getSystemTheme() : mode,
  );

  useEffect(() => {
    const newResolved = mode === "system" ? getSystemTheme() : mode;
    setResolved(newResolved);
    applyTheme(newResolved);
  }, [mode]);

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
      return next;
    });
  }, []);

  return { mode, resolved, cycle };
}
