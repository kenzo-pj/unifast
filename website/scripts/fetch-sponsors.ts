import fs from "node:fs";
import path from "node:path";
import url from "node:url";

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));
const outPath = path.resolve(__dirname, "../src/data/sponsors.json");

const GITHUB_LOGIN = "Kenzo-Wada";

interface Sponsor {
  login: string;
  avatarUrl: string;
  url: string;
}

async function fetchSponsors(): Promise<Sponsor[]> {
  const token = process.env.GH_SPONSORS_TOKEN;
  if (!token) {
    console.log("[fetch-sponsors] GH_SPONSORS_TOKEN not set, skipping.");
    return [];
  }

  const query = `
    query {
      user(login: "${GITHUB_LOGIN}") {
        sponsorshipsAsMaintainer(first: 100, activeOnly: true) {
          nodes {
            sponsorEntity {
              ... on User {
                login
                avatarUrl
                url
              }
              ... on Organization {
                login
                avatarUrl
                url
              }
            }
          }
        }
      }
    }
  `;

  const res = await fetch("https://api.github.com/graphql", {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ query }),
  });

  if (!res.ok) {
    console.error(`[fetch-sponsors] GitHub API error: ${res.status}`);
    return [];
  }

  const json = (await res.json()) as {
    data?: {
      user?: {
        sponsorshipsAsMaintainer?: {
          nodes?: Array<{ sponsorEntity?: Sponsor }>;
        };
      };
    };
  };

  const nodes = json.data?.user?.sponsorshipsAsMaintainer?.nodes ?? [];
  return nodes
    .map((n) => n.sponsorEntity)
    .filter((e): e is Sponsor => e != null && e.login != null);
}

const sponsors = await fetchSponsors();
fs.writeFileSync(outPath, JSON.stringify(sponsors, null, 2) + "\n");
console.log(`[fetch-sponsors] Wrote ${sponsors.length} sponsors to ${outPath}`);
