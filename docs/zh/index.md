---
title: molpack（中文）
description: 纯 Rust 实现的 Packmol 级分子装填，提供命令行、Python API 和 Rust API。
hide:
  - navigation
  - toc
hero:
  kicker: molpack 手册
  title: molpack
  description: 纯 Rust 实现的 Packmol 兼容分子装填引擎 — `.inp` 脚本、Python 包与原生 Rust crate 共用同一套装填模型。
  install:
    label: 安装
    methods:
      - label: CLI
        command: cargo install molcrafts-molpack --features cli
      - label: Rust 库
        command: cargo add molcrafts-molpack
      - label: Python
        command: pip install molcrafts-molpack
  badges:
    - img: https://img.shields.io/crates/v/molcrafts-molpack?color=c2410c&label=crates.io
      href: https://crates.io/crates/molcrafts-molpack
      alt: crates.io 版本
    - img: https://img.shields.io/pypi/v/molcrafts-molpack?color=d97706&label=PyPI
      href: https://pypi.org/project/molcrafts-molpack/
      alt: PyPI 版本
    - img: https://github.com/MolCrafts/molpack/actions/workflows/ci.yml/badge.svg
      href: https://github.com/MolCrafts/molpack/actions/workflows/ci.yml
      alt: CI 状态
    - img: https://img.shields.io/badge/license-BSD--3--Clause-blue.svg
      href: https://github.com/MolCrafts/molpack/blob/master/LICENSE
      alt: BSD-3-Clause 许可证
  actions:
    - label: 快速开始
      href: getting_started/
      style: primary
    - label: Packmol 脚本
      href: cli/
    - label: Python API
      href: python/
    - label: Rust API
      href: rust/
---

<h1 class="molcrafts-sr-only">molpack</h1>

<div class="molcrafts-manual-home" markdown>

<section class="molcrafts-manual-section molcrafts-manual-section--compact" markdown>

<div class="molcrafts-manual-section__header" markdown>

<span class="molcrafts-manual-eyebrow">从这里开始</span>

## 找到对应页面

本页是手册索引，不是营销页。所有入口共用同一模型：分子模板、副本数量、几何约束，
然后运行一次装填。

!!! note "翻译进度"
    中文站与英文站平行（`docs/zh/` ↔ `docs/en/`）。导航与首页为中文；
    部分章节正文仍暂用英文，点导航会留在 `/zh/` 下，不会跳回英文站。

</div>

<nav class="molcrafts-manual-index" aria-label="手册入口">
  <a href="../install/">
    <span>01</span>
    <strong>安装</strong>
    <em>CLI、crates.io 或 PyPI — 选一个入口并验证可运行。</em>
  </a>
  <a href="../getting_started/">
    <span>02</span>
    <strong>快速开始</strong>
    <em>在 40&nbsp;Å 立方体中装填 100 个水分子，并读取收敛诊断。</em>
  </a>
  <a href="../cli/">
    <span>03</span>
    <strong>Packmol 脚本</strong>
    <em>运行 `.inp`：structure、number、inside、fixed、pbc。</em>
  </a>
  <a href="../python/">
    <span>04</span>
    <strong>Python API</strong>
    <em>从 frame 构造 target，在 notebook 与流水线中装填。</em>
  </a>
  <a href="../rust/">
    <span>05</span>
    <strong>Rust API</strong>
    <em>在 crate 里使用 Target 与 Molpack builder。</em>
  </a>
  <a href="../rust/handlers-relaxers/">
    <span>06</span>
    <strong>Handlers</strong>
    <em>观测 step、导出轨迹，或提前停止。</em>
  </a>
</nav>

</section>

<section id="workflows" class="molcrafts-manual-section" markdown>

<div class="molcrafts-manual-section__header" markdown>

<span class="molcrafts-manual-eyebrow">工作流</span>

## 同一个装填模型，四个入口

所有入口都会落到 target / count / restraint。按现有工作流语言选择入口。

</div>

<div class="molcrafts-workflow-list" markdown>

<article markdown>

<div class="molcrafts-workflow-list__meta">路径 01 · 脚本</div>

### 运行 `.inp` 任务

适合已写成 Packmol 风格、需要纳入版本控制的可复现任务。见 [CLI](cli/)。

```text
structure water.pdb
  number 1000
  inside box 0. 0. 0. 40. 40. 40.
end structure
```

</article>

<article markdown>

<div class="molcrafts-workflow-list__meta">路径 02 · Python</div>

### 在 notebook 或流水线中装填

加载或构造 frame，创建不可变 target，再把结果交给写出或分析代码。见 [Python](python/)。

```python
water = Target(frame, 100).with_restraint(
    InsideBoxRestraint([0, 0, 0], [40, 40, 40])
)
packed = Molpack().with_seed(42).pack([water])
```

</article>

<article markdown>

<div class="molcrafts-workflow-list__meta">路径 03 · Rust</div>

### 把引擎嵌入 crate

适合原生应用、服务与测试。见 [Rust](rust/)。

```rust
let frame = Molpack::new()
    .with_seed(42)
    .pack(&[water], 200)?;
```

</article>

<article markdown>

<div class="molcrafts-workflow-list__meta">路径 04 · Handler</div>

### 观测或停止一次运行

Handler 接收装填循环中的结构化事件。见 [Handlers](rust/handlers-relaxers/)。

```rust
impl Handler for WatchFdist {
    fn on_step(&mut self, info: &StepInfo, _sys: &PackContext) {
        eprintln!("fdist={}", info.fdist);
    }
}
```

</article>

</div>

</section>

<section class="molcrafts-manual-section" markdown>

<div class="molcrafts-manual-section__header" markdown>

<span class="molcrafts-manual-eyebrow">地图</span>

## 手册结构

</div>

<div class="molcrafts-doc-map">
  <section>
    <h3>Tutorial</h3>
    <p>安装、快速开始、概念、示例与 Packmol 对等说明。</p>
  </section>
  <section>
    <h3>Packmol 脚本</h3>
    <p>`.inp` 概览、脚本说明、格式与 CLI 示例。</p>
  </section>
  <section>
    <h3>Python</h3>
    <p>安装、targets、restraints、packer、PBC 与 API 参考。</p>
  </section>
  <section>
    <h3>Rust</h3>
    <p>Builder、约束、周期边界、handlers 与 relaxers。</p>
  </section>
  <section>
    <h3>Development</h3>
    <p>贡献流程、架构与扩展点。</p>
  </section>
</div>

</section>

<section class="molcrafts-manual-section molcrafts-manual-section--stack" markdown>

<div class="molcrafts-manual-section__header" markdown>

<span class="molcrafts-manual-eyebrow">入口</span>

## 跳转到表面

</div>

<div class="molcrafts-manual-grid molcrafts-manual-grid--cols-3">
  <a href="../cli/">
    <strong>Packmol 脚本</strong>
    <em>可复现的 `.inp` 任务。</em>
  </a>
  <a href="../python/">
    <strong>Python</strong>
    <em>Notebook 与流水线装填。</em>
  </a>
  <a href="../rust/">
    <strong>Rust</strong>
    <em>原生 builder 与 handler。</em>
  </a>
</div>

</section>

</div>
