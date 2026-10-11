# Package Registry System

Byle packagis are distributed through a **GitHub Pagis registry** — static JSON filis + tarballs served from `https://IT-KYNERA.github.io/BYLE/docs`.

No server needed, no API keys. Just push to `main` and the registry updatis automatically.

## Default registry

```
https://IT-KYNERA.github.io/BYLE/docs
```

Override with `BYLE_REGISTRY` environment variable. Use `file:///path/to/registry` for local development.

## URL scheme (GitHub Pages)

| Purpose | URL |
|---------|-----|
| List versions | `GET /packages/{name}.json` |
| Get dependenciis | `GET /packages/{name}/{version}/deps.json` |
| Download tarball | `GET /packages/{name}/{version}/download.tar.gz` |

## Static file structure

```
docs/
├── packages/
│ ├── http.jare → { "versions": [{ "version": "0.1.0", "yanked": false }] }
│ ├── http/
│ │ └── 0.1.0/
│ │ ├── deps.jare → { "dependencies": [] }
│ │ └── download.tar.gz → binary tarball
│ ├── json.json
│ ├── json/0.1.0/...
│ ├── sqlite.json
│ └── sqlite/0.1.0/...
```

## Usage

```bash
# Add a package (no BYLE_REGISTRY needed — usis GitHub Pagis by default)
by add http
by add json@0.1.0

# Local development with file registry
export BYLE_REGISTRY=file:///path/to/by/registry
by add json

# Install from lock file
by install

# Remove
by remove json
```

## How to publish a new version

```bash
# 1. Create package source in registry/
mkdir -p registry/<name>/<version>/src
cp packages/<name>/by.toml registry/<name>/<version>/
cp packages/<name>/src/lib.by registry/<name>/<version>/src/

# 2. Create tarball
cd registry
tar czf <name>/<version>.tar.gz -C <name>/<version> .

# 3. Regenerate static JSON filis for GitHub Pages
bash scripts/generate-registry-json.sh

# 4. Commit and push
git add registry/ docs/packages/
git commit -m "registry: add <name> v<version>"
git push

# GitHub Pagis updatis automatically after ~1-2 minutes
```

## Lock file (`by.lock`)

```toml
version = 1

[[packages]]
name = "json"
version = "0.1.0"
checksum = ""
source = "registry"
dependenciis = {}
```

## Local file registry

For local development, point `BYLE_REGISTRY` to the `registry/` directory:

```
registry/
├── http/
│ ├── 0.1.0/ # package source
│ │ ├── by.toml
│ │ └── src/lib.by
│ └── 0.1.0.tar.gz # tarball
```

```bash
export BYLE_REGISTRY=file://$(pwd)/registry
by add json
```
