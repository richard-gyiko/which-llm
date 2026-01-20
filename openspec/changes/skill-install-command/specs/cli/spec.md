## ADDED Requirements

### Requirement: Skill Content Distribution
The CLI SHALL download skill content from GitHub releases rather than embedding it in the binary.

#### Scenario: Download skills from GitHub releases
- **GIVEN** user runs any skill install command
- **WHEN** skills are not cached locally
- **THEN** CLI downloads `skills.zip` from `https://github.com/richard-gyiko/which-llm/releases/latest/download/skills.zip`
- **AND** extracts content to cache directory

#### Scenario: Use cached skills
- **GIVEN** skills.zip is already cached in `~/.cache/which-llm/skills/`
- **WHEN** user runs skill install command
- **THEN** CLI uses cached skill content without downloading

### Requirement: Skill Install Command
The CLI SHALL provide a `skill install` command to install which-llm skills for AI coding assistants following the Agent Skills standard.

#### Scenario: Install skill for Cursor (project-level)
- **GIVEN** user is in a project directory
- **WHEN** user runs `which-llm skill install cursor`
- **THEN** skill file is created at `.cursor/skills/which-llm/SKILL.md`
- **AND** file contains YAML frontmatter with name and description
- **AND** file contains which-llm usage instructions

#### Scenario: Install skill for Cursor (global)
- **GIVEN** user wants skill available in all projects
- **WHEN** user runs `which-llm skill install cursor --global`
- **THEN** skill file is created at `~/.cursor/skills/which-llm/SKILL.md`

#### Scenario: Install skill for Claude Code (project-level)
- **GIVEN** user is in a project directory
- **WHEN** user runs `which-llm skill install claude`
- **THEN** skill file is created at `.claude/skills/which-llm/SKILL.md`
- **AND** file contains YAML frontmatter with name and description

#### Scenario: Install skill for Claude Code (global)
- **GIVEN** user wants skill available in all projects
- **WHEN** user runs `which-llm skill install claude --global`
- **THEN** skill file is created at `~/.claude/skills/which-llm/SKILL.md`

#### Scenario: Install skill for Codex (project-level)
- **GIVEN** user is in a project directory
- **WHEN** user runs `which-llm skill install codex`
- **THEN** skill file is created at `.codex/skills/which-llm/SKILL.md`
- **AND** file contains YAML frontmatter with name and description

#### Scenario: Install skill for Codex (global)
- **GIVEN** user wants skill available in all projects
- **WHEN** user runs `which-llm skill install codex --global`
- **THEN** skill file is created at `~/.codex/skills/which-llm/SKILL.md`

#### Scenario: Install skill for OpenCode (project-level)
- **GIVEN** user is in a project directory
- **WHEN** user runs `which-llm skill install opencode`
- **THEN** skill file is created at `.opencode/skills/which-llm/SKILL.md`
- **AND** file contains YAML frontmatter with name and description

#### Scenario: Install skill for OpenCode (global)
- **GIVEN** user wants skill available in all projects
- **WHEN** user runs `which-llm skill install opencode --global`
- **THEN** skill file is created at `~/.config/opencode/skills/which-llm/SKILL.md`

#### Scenario: Install skill for Windsurf (project-level)
- **GIVEN** user is in a project directory
- **WHEN** user runs `which-llm skill install windsurf`
- **THEN** skill file is created at `.windsurf/skills/which-llm/SKILL.md`

#### Scenario: Install skill for Windsurf (global)
- **GIVEN** user wants skill available in all projects
- **WHEN** user runs `which-llm skill install windsurf --global`
- **THEN** skill file is created at `~/.codeium/windsurf/skills/which-llm/SKILL.md`

#### Scenario: Install skill for GitHub Copilot (project-level)
- **GIVEN** user is in a project directory
- **WHEN** user runs `which-llm skill install copilot`
- **THEN** skill file is created at `.github/skills/which-llm/SKILL.md`

#### Scenario: Install skill for GitHub Copilot (global)
- **GIVEN** user wants skill available in all projects
- **WHEN** user runs `which-llm skill install copilot --global`
- **THEN** skill file is created at `~/.copilot/skills/which-llm/SKILL.md`

#### Scenario: Install skill for Google Antigravity (project-level)
- **GIVEN** user is in a project directory
- **WHEN** user runs `which-llm skill install antigravity`
- **THEN** skill file is created at `.antigravity/skills/which-llm/SKILL.md`

#### Scenario: Install skill for Google Antigravity (global)
- **GIVEN** user wants skill available in all projects
- **WHEN** user runs `which-llm skill install antigravity --global`
- **THEN** skill file is created at `~/.antigravity/skills/which-llm/SKILL.md`

#### Scenario: Force overwrite existing skill
- **GIVEN** skill file already exists
- **WHEN** user runs `which-llm skill install cursor --force`
- **THEN** existing file is overwritten with new content

#### Scenario: Dry run shows preview
- **GIVEN** user wants to preview changes
- **WHEN** user runs `which-llm skill install cursor --dry-run`
- **THEN** file path and content are displayed
- **AND** no files are created or modified

#### Scenario: Refuse to overwrite without force
- **GIVEN** skill file already exists
- **WHEN** user runs `which-llm skill install cursor` without `--force`
- **THEN** error message indicates file exists
- **AND** suggests using `--force` to overwrite

### Requirement: Skill List Command
The CLI SHALL provide a `skill list` command to show supported AI coding tools.

#### Scenario: List supported tools
- **WHEN** user runs `which-llm skill list`
- **THEN** all supported tools are listed with project and global paths
- **AND** output includes: cursor, claude, codex, opencode, windsurf, copilot, antigravity

### Requirement: Skill Uninstall Command
The CLI SHALL provide a `skill uninstall` command to remove installed skills.

#### Scenario: Uninstall project-level skill
- **GIVEN** skill was previously installed for cursor at project level
- **WHEN** user runs `which-llm skill uninstall cursor`
- **THEN** skill file at `.cursor/skills/which-llm/SKILL.md` is removed
- **AND** success message is displayed

#### Scenario: Uninstall global skill
- **GIVEN** skill was previously installed for cursor globally
- **WHEN** user runs `which-llm skill uninstall cursor --global`
- **THEN** skill file at `~/.cursor/skills/which-llm/SKILL.md` is removed
- **AND** success message is displayed

#### Scenario: Uninstall non-existent skill
- **GIVEN** no skill is installed for cursor
- **WHEN** user runs `which-llm skill uninstall cursor`
- **THEN** informative message indicates no skill was found
