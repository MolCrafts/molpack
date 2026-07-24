# molpack documentation sources

Layout follows [Zensical language alternate](https://zensical.org/docs/setup/language/)
and the MolCrafts dual-tree pattern (same as molexp):

```
docs/
├── en/                 # English sources  → zensical.toml  → site/en
├── zh/                 # Chinese sources  → zensical.zh.toml → site/zh
│   ├── assets → ../en/assets   # shared (symlink)
│   └── img    → ../en/img
└── root-redirect.html  # optional deploy root → /en/
```

## Build

From the repo root (with the `doc` dependency group installed):

```bash
zensical build -f zensical.toml      # site/en
zensical build -f zensical.zh.toml   # site/zh
# Optional combined deploy root:
#   cp docs/root-redirect.html site/index.html
#   and publish site/en + site/zh beside it.
```

Language switcher links are absolute under `site_url`: `/en/` and `/zh/`.
Clicking 简体中文 stays inside the Chinese tree; nav targets never jump to
English paths.
