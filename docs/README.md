# molpack documentation

Multi-language follows the **official Zensical language setup** — one config,
one build, language selector via `alternate`:

https://zensical.org/docs/setup/language/

## Layout

```
docs/
├── assets/          # shared static files
├── en/              # English pages  →  /en/…
└── zh/              # 简体中文 pages →  /zh/…
```

## `zensical.toml` (only config)

```toml
[project]
docs_dir = "docs"
site_dir = "site"

[project.theme]
language = "en"

[project.extra]
alternate = [
  { name = "English", link = "/en/", lang = "en" },
  { name = "简体中文", link = "/zh/", lang = "zh" },
]
```

Nav points at `en/…` and `zh/…` so both URL trees are published in a single
`zensical build`. The header language selector jumps between `/en/` and `/zh/`.

## Build

```bash
zensical build
# optional: cp docs/root-redirect.html site/index.html
```
