# Multi-Model Architecture Guide

When and how to use different models for different roles in a system.

## When to Use Multi-Model

Consider multi-model architecture when:

- Task has both "thinking" and "doing" components
- High volume workload where cost matters
- Pipeline with distinct stages (extract → process → format)
- User explicitly mentions agents or orchestration

## The Pattern

> **Note:** Thresholds calibrated for Intelligence Index v4.0 (Jan 2026), where current SOTA is ~50.

```
┌─────────────────────────────────────────────────────┐
│                    ORCHESTRATOR                      │
│              (intelligence 48+, coding 42+)          │
│         Planning, decomposition, decisions           │
└─────────────────────┬───────────────────────────────┘
                      │
        ┌─────────────┼─────────────┐
        ▼             ▼             ▼
   ┌─────────┐  ┌─────────┐  ┌─────────┐
   │ Extract │  │ Execute │  │ Format  │
   │  (20+)  │  │  (35+)  │  │  (15+)  │
   └─────────┘  └─────────┘  └─────────┘
     Cheap        Medium       Cheapest
```

## Role Requirements

| Role | Min Intelligence | Min Coding | Priority |
|------|------------------|------------|----------|
| **Orchestrator** | 48 | 42 | Capability over cost |
| **Reasoning Worker** | 38 | - | Balance |
| **Tool Worker** | 35 | 35 | Reliability |
| **Extraction Worker** | 20 | - | Cost |
| **Formatting Worker** | 15 | - | Cost |

## Finding Models by Role

### Orchestrator (Don't Skimp)
```bash
aa query "SELECT name, creator, intelligence, coding, price 
          FROM llms 
          WHERE intelligence >= 48 AND coding >= 42 
          ORDER BY intelligence DESC 
          LIMIT 5"
```

### Reasoning Workers
```bash
aa query "SELECT name, creator, intelligence, price 
          FROM llms 
          WHERE intelligence >= 38 
          ORDER BY price 
          LIMIT 5"
```

### Tool/Code Workers
```bash
aa query "SELECT name, creator, intelligence, coding, price 
          FROM llms 
          WHERE intelligence >= 35 AND coding >= 35 
          ORDER BY price 
          LIMIT 5"
```

### Cheap Extraction/Formatting Workers
```bash
aa query "SELECT name, creator, intelligence, price, tps 
          FROM llms 
          WHERE intelligence >= 15 
          ORDER BY price 
          LIMIT 5"
```

## Cost Optimization Example

**Scenario:** Document processing pipeline
- Extract entities from 10,000 documents
- Analyze extracted data for insights
- Generate summary report

**Single Model Approach:**
- Use GPT-4 class model for everything
- Cost: ~$50/M tokens × high volume = expensive

**Multi-Model Approach:**
| Stage | Model Tier | Est. Price | Volume |
|-------|------------|------------|--------|
| Extract | Budget ($0.50/M) | Low | 10,000 docs |
| Analyze | Mid ($5/M) | Medium | Aggregated |
| Report | Mid ($5/M) | Medium | 1 report |

**Savings:** Often 5-10x cheaper for high-volume pipelines

## Architecture Patterns

### Router Pattern
Use a cheap/fast model to classify incoming requests, route to appropriate specialist:

```
Input → Router (cheap, fast) → Specialist A (expensive)
                            → Specialist B (medium)
                            → Specialist C (cheap)
```

### Cascade Pattern
Start with cheap model, escalate to expensive only when needed:

```
Input → Cheap Model → [confident?] → Output
                   → [uncertain?] → Expensive Model → Output
```

### Verifier Pattern
Use a second model to check the first model's work:

```
Input → Generator (medium) → Output → Verifier (cheap) → [OK?] → Final
                                                      → [Bad?] → Retry
```

## Key Insight

> Intelligence scores matter most for **orchestrator/planning** roles.
> They matter least for **well-scoped execution** roles.

If you decompose your system into narrow skills:
- Each skill requires far less intelligence
- You can use cheaper models for most work
- Failures become local and diagnosable
- Total cost drops significantly
