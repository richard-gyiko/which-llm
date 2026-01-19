# Installing the `aa` CLI

This skill requires the `aa` CLI to be installed and configured.

## Quick Check

```bash
aa --version
```

If this works, you're ready. If not, follow the installation steps below.

## Installation

### macOS / Linux (Homebrew)

```bash
brew tap richard-gyiko/tap
brew install aa
```

### Windows (Scoop)

```powershell
scoop bucket add richard-gyiko https://github.com/richard-gyiko/scoop-bucket
scoop install aa
```

### Manual Download

Download from [GitHub Releases](https://github.com/richard-gyiko/artificial-analysis-cli/releases):

```bash
# macOS (Apple Silicon)
curl -LO https://github.com/richard-gyiko/artificial-analysis-cli/releases/latest/download/aa-aarch64-apple-darwin.tar.gz
tar -xzf aa-aarch64-apple-darwin.tar.gz
sudo mv aa /usr/local/bin/

# macOS (Intel)
curl -LO https://github.com/richard-gyiko/artificial-analysis-cli/releases/latest/download/aa-x86_64-apple-darwin.tar.gz
tar -xzf aa-x86_64-apple-darwin.tar.gz
sudo mv aa /usr/local/bin/

# Linux
curl -LO https://github.com/richard-gyiko/artificial-analysis-cli/releases/latest/download/aa-x86_64-unknown-linux-gnu.tar.gz
tar -xzf aa-x86_64-unknown-linux-gnu.tar.gz
sudo mv aa /usr/local/bin/
```

### From Source (requires Rust)

```bash
git clone https://github.com/richard-gyiko/artificial-analysis-cli.git
cd artificial-analysis-cli
cargo install --path .
```

## Setup

1. Create an account at [Artificial Analysis](https://artificialanalysis.ai/login)
2. Generate an API key
3. Configure the CLI:

```bash
aa profile create default --api-key YOUR_API_KEY
```

Or set the environment variable:

```bash
export AA_API_KEY=YOUR_API_KEY
```

## Verify Installation

```bash
# Should show available models
aa llms --quiet | head -20
```

If you see a table of models, you're all set!
