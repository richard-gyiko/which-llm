---
name: which-llm
description: Select optimal LLM(s) for a task based on skill requirements, budget, and latency constraints. Uses the `aa` CLI to query Artificial Analysis benchmark data. Use when deciding which model to use, comparing models, or designing multi-model architectures.
license: MIT
compatibility: Requires `aa` CLI installed and configured with API key
metadata:
  author: artificial-analysis-cli
  version: "1.1"
  category: "ai"
allowed-tools: Bash(aa:*) Read
---

# Skill: which-llm

Select the right LLM(s) for a task using real benchmark data.

## When to Use

- User needs to pick a model for a specific task
- User wants to compare models by capability/price/speed
- User is designing a multi-agent system and needs model recommendations

## Prerequisites

This skill requires the `aa` CLI. Check if it's installed:

```bash
aa --version
```

If not installed, see [references/INSTALL.md](references/INSTALL.md) for installation instructions.

## Quick Start

1. **Classify the task** into a skill type
2. **Derive minimum requirements** from the skill type
3. **Query with `aa`** to find matching models
4. **Recommend 2-3 options** with tradeoffs

## Skill Type → Requirements

> **Note:** These thresholds are calibrated for Intelligence Index v4.0 (Jan 2026), where current SOTA models score ~50. See [references/BENCHMARKS.md](references/BENCHMARKS.md) for score interpretation.

| Skill Type | Examples | Min Intelligence | Min Coding |
|------------|----------|------------------|------------|
| **Transformational** | summarize, extract, reformat | 20 | - |
| **Analytical** | compare, analyze, justify | 38 | - |
| **Tool-using** | API calls, DB queries, code execution | 35 | 35 |
| **Agentic** | plan, decompose, orchestrate, self-critique | 48 | 42 |

## Core Queries

```bash
# Find models meeting requirements, sorted by price
aa query "SELECT name, creator, intelligence, coding, price, tps 
          FROM llms 
          WHERE intelligence >= 38 
          ORDER BY price 
          LIMIT 10"

# Speed-critical (real-time chat)
aa query "SELECT name, intelligence, tps, latency, price 
          FROM llms 
          WHERE tps > 100 AND latency < 0.5 
          ORDER BY tps DESC"

# Best coding models under budget
aa query "SELECT name, coding, intelligence, price 
          FROM llms 
          WHERE coding >= 35 AND price < 10 
          ORDER BY coding DESC"
```

## Output Format

```markdown
## Task Classification
- **Skill Type:** [type]
- **Reasoning:** [why]

## Recommendations
### Option 1: [Model] (Best Value)
- intelligence X, coding Y, $Z/M tokens
- Why: [fit explanation]

### Option 2: [Model] (Higher Capability)  
- intelligence X, coding Y, $Z/M tokens
- Why: [tradeoff explanation]

## Query Used
[the aa query command]
```

## Important Disclaimer

These recommendations are **indicative starting points**, not definitive answers. Benchmark scores measure general capabilities but may not reflect performance on your specific task.

**Always validate** by testing candidate models on representative examples from your actual use case before committing to a model choice.

## References

For detailed information, see:
- [references/INSTALL.md](references/INSTALL.md) - Installing and configuring the `aa` CLI
- [references/BENCHMARKS.md](references/BENCHMARKS.md) - What the scores mean
- [references/QUERIES.md](references/QUERIES.md) - Common query patterns
- [references/MULTI-MODEL.md](references/MULTI-MODEL.md) - Multi-model architecture guidance
