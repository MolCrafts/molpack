---
title: molpack
description: Packmol-grade molecular packing in pure Rust, with Python bindings.
hide:
  - navigation
  - toc
hero:
  kicker: molpack Manual
  title: molpack
  description: Build non-overlapping initial configurations for molecular dynamics. Give molpack molecular templates, per-type counts, and geometric restraints, and it returns coordinates that satisfy every constraint — a faithful port of Packmol's GENCAN engine in pure Rust.
  install:
    label: Install
    command: pip install molcrafts-molpack molcrafts-molrs
  badges:
    - img: https://github.com/MolCrafts/molpack/actions/workflows/ci.yml/badge.svg
      href: https://github.com/MolCrafts/molpack/actions/workflows/ci.yml
      alt: CI status
    - img: https://img.shields.io/github/stars/MolCrafts/molpack?style=flat&color=c2410c
      href: https://github.com/MolCrafts/molpack
      alt: GitHub stars
    - img: https://img.shields.io/badge/license-BSD--3--Clause-blue.svg
      href: https://github.com/MolCrafts/molpack/blob/master/LICENSE
      alt: License BSD-3-Clause
  actions:
    - label: Get started
      href: getting_started/
      style: primary
    - label: Guide
      href: concepts/
    - label: Architecture
      href: architecture/
---

<h1 class="molcrafts-sr-only">molpack</h1>

<div class="molcrafts-manual-home" markdown>

<section class="molcrafts-manual-section molcrafts-manual-section--compact" markdown>

<div class="molcrafts-manual-section__header" markdown>

<span class="molcrafts-manual-eyebrow">At a glance</span>

## Templates and counts in, packed coordinates out

molpack takes a set of molecular templates, a copy count for each, and the geometric restraints each type must satisfy. It then places every copy so that nothing overlaps and every constraint holds:

</div>

```python
import molrs
from molpack import InsideBoxRestraint, Molpack, Target

frame = molrs.read_pdb("water.pdb")             # one molecule template

water = (
    Target(frame, count=1000)                   # 1000 copies
    .with_name("water")
    .with_restraint(InsideBoxRestraint([0, 0, 0], [40, 40, 40]))
)

packed = Molpack().with_seed(42).pack([water], max_loops=200)
```

The same engine drives a Packmol-compatible CLI and a native Rust API, so a mixture defined once can be packed from a shell script, a Rust program, or Python. The [Getting Started](getting_started.md) chapter walks through a full mixture end to end; the [Examples](examples.md) collect the five canonical workloads.

</section>

<section class="molcrafts-manual-section molcrafts-manual-section--compact" markdown>

<div class="molcrafts-manual-section__header" markdown>

<span class="molcrafts-manual-eyebrow">Find your page</span>

## The manual in six chapters

</div>

<nav class="molcrafts-manual-index" aria-label="Manual chapters">
  <a href="install/">
    <span>01</span>
    <strong>Install</strong>
    <em>The CLI binary, the Rust crate, and the Python wheel — with prerequisites and building from source.</em>
  </a>
  <a href="getting_started/">
    <span>02</span>
    <strong>Getting Started</strong>
    <em>Your first pack end to end: templates, counts, restraints, and written coordinates.</em>
  </a>
  <a href="concepts/">
    <span>03</span>
    <strong>Concepts</strong>
    <em>Targets, restraints, regions, handlers, and relaxers — the pieces every pack is built from.</em>
  </a>
  <a href="python/">
    <span>04</span>
    <strong>Python</strong>
    <em>The molrs-backed binding: installation, guide, examples, and the full API reference.</em>
  </a>
  <a href="architecture/">
    <span>05</span>
    <strong>Architecture</strong>
    <em>The module map, data flow, the packing loop, and the hot path that keeps it fast.</em>
  </a>
  <a href="extending/">
    <span>06</span>
    <strong>Extending</strong>
    <em>Write your own restraint, region, handler, or relaxer against the engine's traits.</em>
  </a>
</nav>

</section>

</div>
