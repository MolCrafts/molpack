# molpack documentation sources

Multi-language setup matches the **official Zensical language guide**:

https://zensical.org/docs/setup/language/

## What the official docs require

1. **`[project.theme] language = "…"`** — one canonical UI language per project
   (HTML only allows a single language per document).
2. **`[project.extra] alternate = […]`** — header language selector with
   `name` / `link` / `lang` pointing at each language’s published root.
3. **Content trees** — separate sources per language, built as separate sites:

```
docs/
├── en/          # English  → zensical.toml      → site/en  (language = en)
└── zh/          # 简体中文 → zensical.zh.toml   → site/zh  (language = zh)
    ├── assets → ../en/assets
    └── img    → ../en/img
```

There is **no** single-build “folder language mode” in Zensical. Chinese is not
a subdirectory of the English site; it is its own project with `language = "zh"`.

## Config (same shape as the official example)

English (`zensical.toml`):

```toml
[project]
site_url = "https://molcrafts.github.io/molpack/en/"
docs_dir = "docs/en"
site_dir = "site/en"

[project.theme]
language = "en"

[project.extra]
alternate = [
  { name = "English", link = "https://molcrafts.github.io/molpack/en/", lang = "en" },
  { name = "简体中文", link = "https://molcrafts.github.io/molpack/zh/", lang = "zh" },
]
```

Chinese (`zensical.zh.toml`): same `alternate`, but `language = "zh"`,
`docs_dir = "docs/zh"`, `site_dir = "site/zh"`,
`site_url = "https://molcrafts.github.io/molpack/zh/"`.

> **Why full URLs in `alternate.link`?**
> The guide says absolute links may include a host. If you only set `link = "/en/"`,
> Zensical prepends **only the domain** of `site_url`, so a project site under
> `/molpack/` would jump to `https://molcrafts.github.io/en/` (wrong). Full URLs
> keep the `/molpack/en/` and `/molpack/zh/` prefixes.

## Build & deploy

```bash
zensical build -f zensical.toml      # → site/en
zensical build -f zensical.zh.toml   # → site/zh
```

Publish so those directories are served at:

| Build output | Public URL |
|---|---|
| `site/en/` | `…/molpack/en/` |
| `site/zh/` | `…/molpack/zh/` |

Optional root landing: copy `docs/root-redirect.html` to the deploy root as
`index.html` so `/molpack/` redirects to `/molpack/en/`.

Cloudflare / CI must run **both** builds; a single `zensical build` only
produces one language.
