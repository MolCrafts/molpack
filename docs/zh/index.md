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

## 分子模板、数量、约束 — 然后运行一次装填

molpack 在每个入口下都围绕同一个固定装填任务：给出分子模板、副本数量、几何约束，
以及可选的固定放置或周期边界；引擎返回一个满足距离和区域要求的装填构型。

</div>

<div class="molpack-system-panel">
<div class="molpack-system-panel__header">
<span>一个引擎 · 四个入口</span>
<strong>脚本用于可复现任务 · API 用于流水线 · handler 用于观测</strong>
</div>
<div class="molpack-system-flow">
<div>
<span>01 · Packmol 脚本</span>
<a href="../cli/"><strong>运行 `.inp`：structure、number、inside、fixed、pbc</strong></a>
</div>
<div>
<span>02 · Python API</span>
<a href="../python/"><strong>从 frame 构造 target，在 notebook 中装填</strong></a>
</div>
<div>
<span>03 · Rust API</span>
<a href="../rust/"><strong>在 crate 里使用 Target 与 Molpack builder</strong></a>
</div>
<div>
<span>04 · 自定义 handler</span>
<a href="../rust/handlers-relaxers/"><strong>观测 step、导出轨迹，或提前停止</strong></a>
</div>
</div>
</div>

[快速开始](../getting_started/) 会装填第一个水盒子。有 `.inp` 时用
[Packmol 脚本](../cli/)，在 notebook 与流水线里用 [Python](../python/)，嵌入
原生应用时用 [Rust](../rust/)。

</section>

<section class="molcrafts-manual-section molpack-results-section" markdown>

<div class="molcrafts-manual-section__header" markdown>

<span class="molcrafts-manual-eyebrow">结果示意</span>

## 引擎能装出什么

受限溶质、界面分布、多组分盒子 — 同一套约束模型贯穿所有入口。

</div>

<div class="molpack-result-gallery">
<figure class="molpack-result-figure">
<img src="../assets/images/paper-confinement-sphere.png" alt="球形约束内的溶剂装填" loading="lazy" />
<figcaption>球形约束 — 活动溶剂被限制在固定溶质周围的几何区域内。</figcaption>
</figure>
<figure class="molpack-result-figure">
<img src="../assets/images/paper-compatibility-distributions.png" alt="界面密度分布匹配" loading="lazy" />
<figcaption>相容性分布 — 集体 profile 约束将界面密度曲线匹配到目标分布。</figcaption>
</figure>
<figure class="molpack-result-figure molpack-result-figure--wide">
<img src="../assets/images/paper-mt-scaling.png" alt="多线程装填墙钟时间缩放" loading="lazy" />
<figcaption>并行评估 — 开启 rayon 后墙钟时间随体系规模的变化。</figcaption>
</figure>
</div>

</section>

<section class="molcrafts-manual-section" markdown>

<div class="molcrafts-manual-section__header" markdown>

<span class="molcrafts-manual-eyebrow">实际使用</span>

## 同一个装填模型，四个入口

所有入口都会落到同一个 target / count / restraint 模型。按工作流语言选择入口。

</div>

<div class="molcrafts-workflow-list molpack-workflow-list" markdown>

<article markdown>

<div class="molcrafts-workflow-list__meta">01 · Packmol 脚本</div>

### [运行 `.inp` 任务](../cli/)

CLI 适合已经写成 Packmol 风格输入、需要纳入版本控制的可复现任务。

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

加载或构造 frame，创建不可变 target，再把结果交给已有的写出或分析代码。

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

适合原生应用、服务、测试，以及引擎本身的新功能。

```rust
let frame = Molpack::new()
    .with_seed(42)
    .pack(&[water], 200)?;
```

</article>

<article markdown>

<div class="molcrafts-workflow-list__meta">04 · 自定义 handler</div>

### [观测或停止一次运行](../rust/handlers-relaxers/)

Handler 接收装填循环中的结构化事件，便于记录诊断、导出中间 frame 或请求提前停止。

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

**Tutorial** 讲通用装填模型；**Packmol 脚本**、**Python**、**Rust** 覆盖公开界面；
**Handler** 讲观测与提前停止。

!!! note "中文内容"
    中文首页为导航入口；详细章节目前以英文正文为主。语言切换在页眉下拉菜单。

</div>

<div class="molcrafts-doc-map molpack-doc-map">
<section>
<h3><a href="../install/">安装</a></h3>
<p>CLI、crates.io 与 PyPI — 选一个入口并验证可导入 / 可运行。</p>
</section>
<section>
<h3><a href="../getting_started/">快速开始</a></h3>
<p>在 40&nbsp;Å 立方体中装填 100 个水分子，并读取收敛诊断。</p>
</section>
<section>
<h3><a href="../cli/">Packmol 脚本</a></h3>
<p>兼容 `.inp`、文件格式、路径解析与 CLI 示例。</p>
</section>
<section>
<h3><a href="../python/">Python API</a></h3>
<p>Targets、restraints、packer、PBC、示例与 API 参考。</p>
</section>
<section>
<h3><a href="../rust/">Rust API</a></h3>
<p>Builder、约束作用域、周期边界、handlers、relaxers。</p>
</section>
</div>

</section>

</div>
