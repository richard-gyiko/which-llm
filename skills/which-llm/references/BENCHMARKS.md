# Benchmark Reference

Detailed explanation of Artificial Analysis metrics and what scores mean.

> **Note:** These benchmarks measure general capabilities across standardized tests. They are useful for narrowing down candidates but do not guarantee performance on your specific task. Always validate with your own evaluations.

## Available Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `intelligence` | DOUBLE | Composite score (0-100) across reasoning, knowledge, coding, agents |
| `coding` | DOUBLE | Code generation and terminal-based task completion |
| `math` | DOUBLE | Mathematical reasoning (competition level) |
| `mmlu_pro` | DOUBLE | MMLU-Pro benchmark score |
| `gpqa` | DOUBLE | Graduate-level science Q&A |
| `price` | DOUBLE | Blended cost per 1M tokens (3:1 input:output ratio) |
| `input_price` | DOUBLE | Cost per 1M input tokens |
| `output_price` | DOUBLE | Cost per 1M output tokens |
| `tps` | DOUBLE | Tokens per second (output generation speed) |
| `latency` | DOUBLE | Time to first token in seconds |

## Intelligence Index (v4.0)

The intelligence score is a weighted composite of 10 evaluations across 4 categories:

| Category (25% each) | What it measures |
|---------------------|------------------|
| **Agents** | Real-world task completion (GDPval-AA), agentic workflows (τ²-Bench) |
| **Coding** | Terminal tasks (Terminal-Bench Hard), scientific code (SciCode) |
| **General** | Long-context reasoning, knowledge/hallucination, instruction following |
| **Scientific Reasoning** | Graduate-level science (GPQA, HLE), physics (CritPt) |

**Scale Calibration (Jan 2026):** The v4.0 index was recalibrated to create headroom for future improvements. Current SOTA models (GPT-5.2, Claude Opus 4.5) score ~50. Legacy models from 2024 (e.g., Llama 3.1 405B) score ~15. This is intentional—the scale measures real-world economic work capability, not academic test performance.

**Confidence interval:** ±1% (so 45 vs 46 is effectively equivalent)

## Intelligence Score Interpretation

> **Note:** These tiers are calibrated for Intelligence Index v4.0 (Jan 2026). Current SOTA is ~50.

| Score | Tier | Capabilities |
|-------|------|--------------|
| **48+** | Frontier | Complex autonomous agents, research-level problems, PhD-level reasoning |
| **40-48** | Very Strong | Professional coding, complex analysis, multi-step agentic tasks |
| **30-40** | Strong | Most business tasks, standard coding, document Q&A, analysis |
| **20-30** | Capable | Simple coding, basic Q&A, content generation, straightforward tasks |
| **10-20** | Basic | Chat, simple queries, drafting text, basic assistance |
| **<10** | Limited | Very simple tasks only |

## Coding Score Interpretation

| Score | Capability |
|-------|------------|
| **45+** | Exceptional - complex systems, debugging, architecture |
| **35-45** | Strong - professional software development |
| **25-35** | Capable - standard coding tasks, scripts |
| **<25** | Limited - simple code only |

## Math Score Interpretation

| Score | Capability |
|-------|------------|
| **70+** | Competition-level (AIME, IMO problems) |
| **50-70** | University-level mathematics |
| **30-50** | High school / AP level |
| **<30** | Basic arithmetic, simple algebra |

## Price Considerations

- `price` is blended assuming 3:1 input:output ratio
- For **output-heavy tasks** (generation), check `output_price` directly
- For **input-heavy tasks** (analysis of large docs), check `input_price`

```bash
# Compare input vs output pricing
aa query "SELECT name, input_price, output_price, price 
          FROM llms 
          WHERE price < 5 
          ORDER BY output_price"
```

## Speed vs Latency

- `tps` (tokens per second) - How fast the model generates output
- `latency` (TTFT) - Time to first token, critical for interactive applications

| Use Case | Priority |
|----------|----------|
| Batch processing | High TPS, latency doesn't matter |
| Real-time chat | Both TPS and low latency matter |
| Streaming UI | Low latency most important |

## Data Freshness

- Data is cached locally by the `aa` CLI
- Use `aa llms --refresh` to fetch fresh data
- Use `aa cache status` to check cache age
