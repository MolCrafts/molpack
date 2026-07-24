---
title: molpack（中文）
description: 纯 Rust 实现的 Packmol 级分子装填，提供命令行、Python API 和 Rust API。
hide:
  - navigation
  - toc
hero:
  kicker: molpack 手册
  title: molpack
  description: 纯 Rust 实现的 Packmol 兼容分子装填引擎，可通过 `.inp` 脚本、Python 包和 Rust crate 使用。
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
    - img: https://github.com/MolCrafts/molpack/actions/workflows/ci.yml/badge.svg
      href: https://github.com/MolCrafts/molpack/actions/workflows/ci.yml
      alt: CI 状态
    - img: https://img.shields.io/github/stars/MolCrafts/molpack?style=flat&color=c2410c
      href: https://github.com/MolCrafts/molpack
      alt: GitHub stars
    - img: https://img.shields.io/badge/license-BSD--3--Clause-blue.svg
      href: https://github.com/MolCrafts/molpack/blob/master/LICENSE
      alt: BSD-3-Clause 许可证
  actions:
    - label: 快速开始
      href: ../getting_started/
      style: primary
    - label: Packmol 脚本
      href: ../cli/
    - label: Python API
      href: ../python/
    - label: Rust API
      href: ../rust/
---

<h1 class="molcrafts-sr-only">molpack</h1>

<div class="molcrafts-manual-home molpack-home" markdown>

<section class="molcrafts-manual-section molpack-system-section" markdown>

<div class="molcrafts-manual-section__header" markdown>

<span class="molcrafts-manual-eyebrow">核心模型</span>

## 分子模板、数量、约束，然后运行一次装填

molpack 在每个入口下都围绕同一个固定装填任务组织：给出分子模板、副本数量、几何约束，
以及可选的固定放置或周期边界；引擎返回一个满足距离和区域要求的装填构型。

</div>

<div class="molpack-system-panel">
<div class="molpack-system-panel__header">
<span>一个引擎 · 四个入口</span>
<strong>脚本用于可复现任务，API 用于流水线，handler 用于观测运行过程</strong>
</div>
<div class="molpack-system-flow">
<div>
<span>01 · Packmol 脚本</span>
<a href="../cli/"><strong>运行 `.inp` 文件，支持熟悉的 structure、number、inside、fixed 和 pbc 关键字</strong></a>
</div>
<div>
<span>02 · Python API</span>
<a href="../python/"><strong>从 frame 构造 target，在 notebook 中装填，并把结果留在内存里</strong></a>
</div>
<div>
<span>03 · Rust API</span>
<a href="../rust/"><strong>在 crate 里直接使用 Target 和 Molpack builder</strong></a>
</div>
<div>
<span>04 · 自定义 handler</span>
<a href="../rust/handlers-relaxers/"><strong>观测 step、导出轨迹，或从自己的 handler 停止运行</strong></a>
</div>
</div>
</div>

[快速开始](../getting_started/) 会装填第一个水盒子。有 `.inp` 输入文件时用
[Packmol 脚本](../cli/)，在 notebook 和流水线里用 [Python](../python/)，需要嵌入
原生应用时用 [Rust](../rust/)。

</section>

<section class="molcrafts-manual-section" markdown>

<div class="molcrafts-manual-section__header" markdown>

<span class="molcrafts-manual-eyebrow">实际使用</span>

## 同一个装填模型，四个入口

所有入口都会落到同一个 target / count / restraint 模型。按你现有工作流的语言选择入口。

</div>

<div class="molcrafts-workflow-list molpack-workflow-list" markdown>

<article markdown>

<div class="molcrafts-workflow-list__meta">01 · Packmol 脚本</div>

### [运行 `.inp` 任务](../cli/)

CLI 适合已经写成 Packmol 风格输入文件、需要纳入版本控制的可复现装填任务。

```text
structure water.pdb
  number 1000
  inside box 0. 0. 0. 40. 40. 40.
end structure
```

</article>

<article markdown>

<div class="molcrafts-workflow-list__meta">02 · Python API</div>

### [在 notebook 或流水线中装填](../python/)

加载或构造一个 frame，创建不可变 target，然后把装填后的 frame 交给你已有的写文件或分析代码。

```python
water = Target(frame, 100).with_restraint(
    InsideBoxRestraint([0, 0, 0], [40, 40, 40])
)
packed = Molpack().with_seed(42).pack([water])
```

</article>

<article markdown>

<div class="molcrafts-workflow-list__meta">03 · Rust API</div>

### [把引擎嵌入 crate](../rust/)

Rust builder API 适合原生应用、服务、测试，以及 molpack 引擎本身的新功能。

```rust
let frame = Molpack::new()
    .with_seed(42)
    .pack(&[water], 200)?;
```

</article>

<article markdown>

<div class="molcrafts-workflow-list__meta">04 · 自定义 handler</div>

### [观测或停止一次运行](../rust/handlers-relaxers/)

Handler 从装填循环接收结构化事件，因此你可以记录诊断信息、导出中间 frame，或请求提前停止。

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

<span class="molcrafts-manual-eyebrow">页面索引</span>

## 手册

手册按入口组织。**Tutorial** 讲通用装填模型。**Packmol 脚本**、**Python API**
和 **Rust API** 覆盖公开使用界面。**自定义 handler** 讲观测与提前停止钩子。

</div>

<div class="molcrafts-doc-map molpack-doc-map">
<section>
<h3><a href="../getting_started/">Tutorial</a></h3>
<p>安装 molpack，跑 quickstart，理解 target/restraint，并对比 canonical workloads。</p>
</section>
<section>
<h3><a href="../cli/">Packmol 脚本</a></h3>
<p>Packmol 兼容 `.inp` 脚本、支持的文件格式、路径解析和 CLI 示例。</p>
</section>
<section>
<h3><a href="../python/">Python API</a></h3>
<p>Notebook 和流水线用法：targets、restraints、packer options、examples 和 API reference。</p>
</section>
<section>
<h3><a href="../rust/">Rust API</a></h3>
<p>原生 builder、restraint 作用域、周期边界、handlers、relaxers 和示例程序。</p>
</section>
<section>
<h3><a href="../rust/handlers-relaxers/">自定义 Handlers</a></h3>
<p>进度观测、轨迹导出、自定义诊断、early stop 和 in-loop relaxers。</p>
</section>
</div>

</section>

</div>
