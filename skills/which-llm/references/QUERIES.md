# Common Query Patterns

Ready-to-use `aa query` commands for common model selection scenarios.

## By Use Case

### Smartest Models (Regardless of Cost)
```bash
aa query "SELECT name, creator, intelligence, coding, price 
          FROM llms 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

### Cheap Models for High-Volume Tasks
```bash
aa query "SELECT name, creator, intelligence, price, tps 
          FROM llms 
          WHERE intelligence >= 30 
          ORDER BY price 
          LIMIT 10"
```

### Fast Models for Real-Time Chat
```bash
aa query "SELECT name, creator, intelligence, tps, latency, price 
          FROM llms 
          WHERE tps > 100 AND latency < 0.5 
          ORDER BY tps DESC"
```

### Best Coding Models Under Budget
```bash
aa query "SELECT name, creator, coding, intelligence, price 
          FROM llms 
          WHERE coding IS NOT NULL AND price < 10 
          ORDER BY coding DESC"
```

### Agentic/Orchestrator Models
```bash
aa query "SELECT name, creator, intelligence, coding, price 
          FROM llms 
          WHERE intelligence >= 55 AND coding >= 45 
          ORDER BY intelligence DESC 
          LIMIT 10"
```

### Budget Worker Models (for pipelines)
```bash
aa query "SELECT name, creator, intelligence, price 
          FROM llms 
          WHERE intelligence >= 30 
          ORDER BY price 
          LIMIT 10"
```

## By Constraint

### With Budget Constraint
```bash
aa query "SELECT name, creator, intelligence, coding, price 
          FROM llms 
          WHERE price < 5 
          ORDER BY intelligence DESC"
```

### With Speed Constraint
```bash
aa query "SELECT name, creator, intelligence, tps, price 
          FROM llms 
          WHERE tps > 150 
          ORDER BY intelligence DESC"
```

### With Latency Constraint
```bash
aa query "SELECT name, creator, intelligence, latency, price 
          FROM llms 
          WHERE latency < 0.3 
          ORDER BY intelligence DESC"
```

## By Creator

### Compare Specific Creators
```bash
aa query "SELECT name, creator, intelligence, coding, price, tps 
          FROM llms 
          WHERE creator IN ('OpenAI', 'Anthropic', 'Google') 
          ORDER BY intelligence DESC"
```

### Single Creator Deep Dive
```bash
aa query "SELECT name, intelligence, coding, math, price, tps 
          FROM llms 
          WHERE creator = 'OpenAI' 
          ORDER BY intelligence DESC"
```

## Aggregations

### Average Intelligence by Creator
```bash
aa query "SELECT creator, COUNT(*) as models, 
                 ROUND(AVG(intelligence), 1) as avg_intel,
                 ROUND(MIN(price), 2) as min_price
          FROM llms 
          WHERE intelligence IS NOT NULL 
          GROUP BY creator 
          ORDER BY avg_intel DESC"
```

### Price Tiers
```bash
aa query "SELECT 
            CASE 
              WHEN price < 1 THEN 'Budget (<$1)'
              WHEN price < 5 THEN 'Mid ($1-5)'
              WHEN price < 20 THEN 'Premium ($5-20)'
              ELSE 'Enterprise ($20+)'
            END as tier,
            COUNT(*) as models,
            ROUND(AVG(intelligence), 1) as avg_intel
          FROM llms
          WHERE price IS NOT NULL
          GROUP BY tier
          ORDER BY avg_intel DESC"
```

## Skill-Type Specific Queries

### Transformational Tasks (summarize, extract, reformat)
```bash
aa query "SELECT name, creator, intelligence, price, tps 
          FROM llms 
          WHERE intelligence >= 30 
          ORDER BY price 
          LIMIT 10"
```

### Analytical Tasks (compare, analyze, justify)
```bash
aa query "SELECT name, creator, intelligence, price 
          FROM llms 
          WHERE intelligence >= 45 
          ORDER BY price 
          LIMIT 10"
```

### Tool-Using Tasks (API calls, code execution)
```bash
aa query "SELECT name, creator, intelligence, coding, price 
          FROM llms 
          WHERE intelligence >= 40 AND coding >= 40 
          ORDER BY price 
          LIMIT 10"
```

### Agentic Tasks (planning, orchestration)
```bash
aa query "SELECT name, creator, intelligence, coding, price 
          FROM llms 
          WHERE intelligence >= 55 AND coding >= 45 
          ORDER BY price 
          LIMIT 10"
```

## Utilities

### List All Available Columns
```bash
aa query --tables
```

### Refresh Data
```bash
aa llms --refresh
```

### Export to JSON (for further processing)
```bash
aa llms --json > models.json
```
