#!/bin/bash
# Builds "Project Color Picker.app" into dist/ (runs on Apple Silicon and Intel Macs).
# Run on a Mac with Rust installed:   bash macos/build-app.sh
set -euo pipefail
cd "$(dirname "$0")/.."

NAME="Project Color Picker"
APP="dist/$NAME.app"
VERSION="1.0.1"

rustup target add aarch64-apple-darwin x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"

# one binary for both kinds of Mac
lipo -create -output "$APP/Contents/MacOS/tool1" \
    target/aarch64-apple-darwin/release/tool1 \
    target/x86_64-apple-darwin/release/tool1

# app icon from assets/icon.png
ICONSET="$(mktemp -d)/AppIcon.iconset"
mkdir -p "$ICONSET"
for s in 16 32 128 256 512; do
    sips -z $s $s assets/icon.png --out "$ICONSET/icon_${s}x${s}.png" >/dev/null
    sips -z $((s * 2)) $((s * 2)) assets/icon.png --out "$ICONSET/icon_${s}x${s}@2x.png" >/dev/null
done
iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/AppIcon.icns"

cat > "$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>                <string>$NAME</string>
    <key>CFBundleDisplayName</key>         <string>$NAME</string>
    <key>CFBundleIdentifier</key>          <string>io.github.acus9345.projectcolorpicker</string>
    <key>CFBundleExecutable</key>          <string>tool1</string>
    <key>CFBundleIconFile</key>            <string>AppIcon</string>
    <key>CFBundlePackageType</key>         <string>APPL</string>
    <key>CFBundleShortVersionString</key>  <string>$VERSION</string>
    <key>CFBundleVersion</key>             <string>$VERSION</string>
    <key>LSMinimumSystemVersion</key>      <string>10.15</string>
    <key>NSHighResolutionCapable</key>     <true/>
</dict>
</plist>
PLIST

# ad-hoc signature so macOS will start it (it's still unsigned for distribution:
# the first time, right-click the app and choose Open)
codesign --force --deep --sign - "$APP"

echo "Built: $APP"
