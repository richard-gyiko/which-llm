# Tasks: skill install command

## Phase 1: Core Infrastructure (which-llm-cli repo)
- [ ] Create `src/commands/skill.rs` with tool definitions
- [ ] Define `Tool` enum with supported tools (cursor, claude, codex, opencode, windsurf, copilot, antigravity)
- [ ] Define tool path configurations (project and global directory paths)
- [ ] Implement skills.zip download from GitHub releases
- [ ] Implement caching in `~/.cache/which-llm/skills/`
- [ ] Implement skill directory extraction from zip

## Phase 2: CLI Integration
- [ ] Add `Skill` command to `src/cli/mod.rs` with subcommands
- [ ] Add `SkillCommands` enum (Install, Uninstall, List)
- [ ] Add flags: `--global`, `--force`, `--dry-run`
- [ ] Export skill module in `src/commands/mod.rs`

## Phase 3: Install Command
- [ ] Implement `skill install <tool>` command
- [ ] Check cache for existing skills.zip (with TTL or version check)
- [ ] Download skills.zip if not cached or stale
- [ ] Extract entire `which-llm/` skill directory from zip
- [ ] Handle project-level installation (default)
- [ ] Handle global-level installation (`--global`)
- [ ] Recursively copy skill directory (SKILL.md, references/, etc.)
- [ ] Handle existing directory detection (error without `--force`)
- [ ] Support `--dry-run` output (list all files that would be copied)

## Phase 4: Supporting Commands
- [ ] Implement `skill list` - show supported tools with directory paths
- [ ] Implement `skill uninstall <tool>` - remove installed skill directory
- [ ] Handle `--global` flag for uninstall

## Phase 5: Tool-Specific Paths
- [ ] Cursor: `.cursor/skills/which-llm/` / `~/.cursor/skills/which-llm/`
- [ ] Claude Code: `.claude/skills/which-llm/` / `~/.claude/skills/which-llm/`
- [ ] Codex: `.codex/skills/which-llm/` / `~/.codex/skills/which-llm/`
- [ ] OpenCode: `.opencode/skills/which-llm/` / `~/.config/opencode/skills/which-llm/`
- [ ] Windsurf: `.windsurf/skills/which-llm/` / `~/.codeium/windsurf/skills/which-llm/`
- [ ] Copilot: `.github/skills/which-llm/` / `~/.copilot/skills/which-llm/`
- [ ] Antigravity: `.antigravity/skills/which-llm/` / `~/.antigravity/skills/which-llm/`

## Phase 6: Testing & Documentation
- [ ] Add unit tests for path resolution
- [ ] Add unit tests for download/cache logic
- [ ] Add unit tests for directory copy logic
- [ ] Add integration tests for install/uninstall
- [ ] Update README with skill command documentation
- [ ] Add examples for each supported tool
