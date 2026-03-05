import { useState, useRef, useCallback, useEffect } from "react";

interface SearchResult {
  url: string;
  title: string;
  excerpt: string;
}

let pagefind: any = null;

async function loadPagefind() {
  if (pagefind) return pagefind;
  try {
    pagefind = await import(/* @vite-ignore */ `${import.meta.env.BASE_URL}pagefind/pagefind.js`);
    await pagefind.init();
    return pagefind;
  } catch {
    return null;
  }
}

export function useSearch() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [loading, setLoading] = useState(false);
  const timerRef = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);

  const search = useCallback((term: string) => {
    setQuery(term);

    if (timerRef.current) clearTimeout(timerRef.current);

    if (!term.trim()) {
      setResults([]);
      setLoading(false);
      return;
    }

    setLoading(true);
    timerRef.current = setTimeout(async () => {
      const pf = await loadPagefind();
      if (!pf) {
        setResults([]);
        setLoading(false);
        return;
      }

      const response = await pf.search(term);
      const items: SearchResult[] = [];
      const base = import.meta.env.BASE_URL.replace(/\/+$/, "");

      for (const result of response.results.slice(0, 8)) {
        const data = await result.data();
        const url = base && data.url.startsWith(base) ? data.url.slice(base.length) : data.url;
        items.push({
          url,
          title: data.meta?.title || url,
          excerpt: data.excerpt,
        });
      }

      setResults(items);
      setLoading(false);
    }, 200);
  }, []);

  const reset = useCallback(() => {
    setQuery("");
    setResults([]);
    setLoading(false);
    if (timerRef.current) clearTimeout(timerRef.current);
  }, []);

  useEffect(() => {
    return () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    };
  }, []);

  return { query, results, loading, search, reset };
}
