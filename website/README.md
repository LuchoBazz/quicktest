# Website

This website is built using [Docusaurus](https://docusaurus.io/), a modern static website generator.

## Quick Start

Requires [Bun](https://bun.sh/) (`>= 1.0.0`). Install it via:

```bash
curl -fsSL https://bun.sh/install | bash
```

Or with `asdf`/`mise` — the repo's `.tool-versions` pins the exact version:

```bash
asdf install
# or
mise install
```

### Installation

```bash
bun install
```

### Local Development

```bash
bun run start
```

Starts a local development server and opens a browser window. Most changes are reflected live without restarting.

### Build

```bash
bun run build
```

Generates static content into the `build` directory.

### Deployment

Using SSH:

```bash
USE_SSH=true bun run deploy
```

Not using SSH:

```bash
GIT_USER=<Your GitHub username> bun run deploy
```

If you are using GitHub Pages for hosting, this command builds the website and pushes to the `gh-pages` branch.
