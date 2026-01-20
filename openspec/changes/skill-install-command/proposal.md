# Change: Add `skill install` command for AI coding tools

## Why

Users want to integrate `which-llm` into their AI coding assistants (Cursor, Claude Code, Codex, OpenCode, etc.) but currently have to manually create skill files. A `which-llm skill install <tool>` command would automate this setup, improving onboarding and adoption.

All major AI coding tools now support the [Agent Skills](https://agentskills.io) open standard - a portable, version-controlled format for teaching agents specialized tasks.

## What Changes

Add a new `skill` subcommand with `install`, `uninstall`, and `list` subcommands:

```bash
# Install skill for a specific tool (project-level by default)
which-llm skill install cursor
which-llm skill install claude
which-llm skill install codex
which-llm skill install opencode
which-llm skill install antigravity

# Install globally (user-level, available in all projects)
which-llm skill install cursor --global

# List supported tools
which-llm skill list

# Remove installed skill
which-llm skill uninstall cursor
```

## Architecture

### Skill Distribution via GitHub Releases

Skills are **not embedded** in the CLI binary. Instead, they are distributed via GitHub releases:

1. The `which-llm` repo (`richard-gyiko/which-llm`) contains a `skills/` directory with skill content
2. GitHub Actions workflow zips the `skills/` directory on each release
3. Release artifacts include `skills.zip` containing the latest skill files
4. CLI downloads from: `https://github.com/richard-gyiko/which-llm/releases/latest/download/skills.zip`

**Benefits:**
- Skill content can be updated without releasing a new CLI version
- Single source of truth for skill content
- Skills are versioned with releases
- Downloaded skills are cached locally

### Skills Repository Structure

Skills follow the [Agent Skills specification](https://agentskills.io/specification) directory structure. Each skill is a self-contained directory that can include multiple files:

```
which-llm/                          # Main repo (richard-gyiko/which-llm)
├── skills/
│   └── which-llm/                  # Skill directory (named after the skill)
│       ├── SKILL.md                # Required: Main instructions (<500 lines)
│       └── references/             # Optional: Additional documentation
│           └── EXAMPLES.md         # Detailed usage examples
└── .github/workflows/
    └── release.yml                 # Zips skills/ and attaches to releases
```

**Agent Skills Directory Structure:**
- `SKILL.md` - Required main instructions file (keep under 500 lines)
- `references/` - Optional directory for detailed docs, API references, etc.
- `scripts/` - Optional directory for executable helper scripts
- `assets/` - Optional directory for templates, images, data files

### Install Flow

```
which-llm skill install cursor
        │
        ▼
┌─────────────────────────────┐
│ Check cache for skills.zip  │
│ (~/.cache/which-llm/)       │
└─────────────────────────────┘
        │ (cache miss or stale)
        ▼
┌─────────────────────────────┐
│ Download skills.zip from    │
│ GitHub releases (latest)    │
└─────────────────────────────┘
        │
        ▼
┌─────────────────────────────┐
│ Extract which-llm/ skill    │
│ directory from zip          │
└─────────────────────────────┘
        │
        ▼
┌─────────────────────────────┐
│ Copy entire skill directory │
│ to tool-specific path       │
│ .cursor/skills/which-llm/   │
└─────────────────────────────┘
```

The install command copies the **entire skill directory** (including `SKILL.md`, `references/`, and any other subdirectories) to the target location.

### Supported Tools

All tools follow the Agent Skills standard. The skill directory (containing `SKILL.md` and optional subdirectories) is copied to the appropriate location.

| Tool | Project Path | Global Path |
|------|--------------|-------------|
| **cursor** | `.cursor/skills/which-llm/` | `~/.cursor/skills/which-llm/` |
| **claude** | `.claude/skills/which-llm/` | `~/.claude/skills/which-llm/` |
| **codex** | `.codex/skills/which-llm/` | `~/.codex/skills/which-llm/` |
| **opencode** | `.opencode/skills/which-llm/` | `~/.config/opencode/skills/which-llm/` |
| **windsurf** | `.windsurf/skills/which-llm/` | `~/.codeium/windsurf/skills/which-llm/` |
| **copilot** | `.github/skills/which-llm/` | `~/.copilot/skills/which-llm/` |
| **antigravity** | `.antigravity/skills/which-llm/` | `~/.antigravity/skills/which-llm/` |

### Flags

- `--global` - Install to user-level directory (available in all projects)
- `--force` - Overwrite existing skill directory
- `--dry-run` - Show what would be written without making changes

## Impact

### New Files (CLI repo: `which-llm-cli`)
- `src/commands/skill.rs` - Skill command implementation (download, extract, install directory)

### Modified Files (CLI repo)
- `src/cli/mod.rs` - Add `Skill` command variant
- `src/commands/mod.rs` - Export skill module
- `src/main.rs` - Handle skill commands

## Non-Goals

- MCP server registration (complex, requires separate JSON configs)
- Auto-detection of which tools are installed
- Installing specific skill versions (always uses latest for now)

## Future Considerations

- `which-llm skill update` to refresh cached skills and reinstall
- `--version` flag to install skills from specific release
- Offline mode using cached skills when network unavailable
