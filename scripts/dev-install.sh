#!/bin/bash
set -eu

export PATH="/opt/homebrew/opt/rustup/bin:/opt/homebrew/opt/llvm@18/bin:$PATH"
export LLVM_SYS_181_PREFIX=$(brew --prefix llvm@18)

echo "→ Building by compiler..."
cargo build --release --bin by --locked

echo "→ Building runtime..."
cargo build --release -p byc_runtime --locked

echo "→ Installing to ~/.by/..."
mkdir -p ~/.by/bin ~/.by/lib
cp target/release/by ~/.by/bin/by
cp target/release/libbyc_runtime.a ~/.by/lib/libbyc_runtime.a
chmod +x ~/.by/bin/by

if [ "$(uname -s)" = "Darwin" ]; then
    xattr -d com.apple.quarantine ~/.by/bin/by 2>/dev/null || true
    codesign -f -s - ~/.by/bin/by
fi

echo "→ Verifying..."
~/.by/bin/by --version

echo ""
echo "✅ Installed. Try:"
echo "   ~/.by/bin/by run examples/hello.by"
echo "   export PATH=\"\$HOME/.by/bin:\$PATH\" && by run examples/hello.by"
