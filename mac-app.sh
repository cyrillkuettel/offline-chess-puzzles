#!/bin/bash

#PROJECT_DIR="$(pwd)"

# CONFIGURATION
APP_NAME="Offline Chess Puzzles"
ZIP_PATH=./offline-chess-puzzles-release
EXECUTABLE_PATH=./target/release/offline-chess-puzzles
ICON_IMAGE=./icon.png
ICONSET_DIR=./target/offline-chess-puzzles.iconset
ICNS_FILE_NAME=offline-chess-puzzles.icns
ICNS_PATH=./target/$ICNS_FILE_NAME
export APP_BUNDLE=$ZIP_PATH/$APP_NAME.app
MACOS_DIR=$APP_BUNDLE/Contents/MacOS
RESOURCES_DIR=$APP_BUNDLE/Contents/Resources
PLIST_PATH=$APP_BUNDLE/Contents/Info.plist

# 0. Build the app with macbundle configuration
echo "Building app with macbundle configuration..."
RUSTFLAGS="--cfg macbundle" cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

# 1. Generate .icns from PNG
mkdir -p "$ICONSET_DIR"
sips -z 16 16     "$ICON_IMAGE" --out "$ICONSET_DIR/icon_16x16.png"
sips -z 32 32     "$ICON_IMAGE" --out "$ICONSET_DIR/icon_16x16@2x.png"
sips -z 32 32     "$ICON_IMAGE" --out "$ICONSET_DIR/icon_32x32.png"
sips -z 64 64     "$ICON_IMAGE" --out "$ICONSET_DIR/icon_32x32@2x.png"
sips -z 128 128   "$ICON_IMAGE" --out "$ICONSET_DIR/icon_128x128.png"
sips -z 256 256   "$ICON_IMAGE" --out "$ICONSET_DIR/icon_128x128@2x.png"
sips -z 256 256   "$ICON_IMAGE" --out "$ICONSET_DIR/icon_256x256.png"
sips -z 512 512   "$ICON_IMAGE" --out "$ICONSET_DIR/icon_256x256@2x.png"
sips -z 512 512   "$ICON_IMAGE" --out "$ICONSET_DIR/icon_512x512.png"
cp "$ICON_IMAGE" "$ICONSET_DIR/icon_512x512@2x.png"
iconutil -c icns "$ICONSET_DIR" -o "$ICNS_PATH"

# 2. Create app bundle structure
mkdir -p $ZIP_PATH
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

cp -R pieces "$RESOURCES_DIR/"
cp -R puzzles "$ZIP_PATH/"
cp -R translations "$RESOURCES_DIR/"
cp .env "$ZIP_PATH/"
cp ocp.db "$ZIP_PATH/"
cp *.ogg "$RESOURCES_DIR/"
cp settings.json "$ZIP_PATH/"
# Fix the puzzle_db_location path in settings.json for macbundle
sed -i '' 's|"puzzle_db_location": "puzzles/|"puzzle_db_location": "../../../puzzles/|g' "$ZIP_PATH/settings.json"
cp LICENSE "$ZIP_PATH/"
cp README.md "$ZIP_PATH/"
cp $EXECUTABLE_PATH "$MACOS_DIR/offline-chess-puzzles-bin"

# 3. Copy the icon
cp "$ICNS_PATH" "$RESOURCES_DIR"

# 4. Create launcher script that sets working directory to MacOS
cat > "$MACOS_DIR/offline-chess-puzzles" <<'EOF'
#!/bin/bash
# Get the directory containing this script (MacOS directory)
MACOS_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
# Change to MacOS directory (where all relative paths in the app expect to be)
cd "$MACOS_DIR"
# Run the binary
exec "$MACOS_DIR/offline-chess-puzzles-bin"
EOF

chmod +x "$MACOS_DIR/offline-chess-puzzles"

# 5. Create Info.plist
cat > "$PLIST_PATH" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
 "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>offline-chess-puzzles</string>
  <key>CFBundleIdentifier</key>
  <string>brianch.offlinechesspuzzles</string>
  <key>CFBundleName</key>
  <string>Offline Chess Puzzles</string>
  <key>CFBundleDisplayName</key>
  <string>Offline Chess Puzzles</string>
  <key>CFBundleVersion</key>
  <string>1.0</string>
  <key>CFBundleShortVersionString</key>
  <string>1.0</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleIconFile</key>
  <string>$ICNS_FILE_NAME</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>LSMinimumSystemVersion</key>
  <string>10.13</string>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>LSApplicationCategoryType</key>
  <string>public.app-category.games</string>
  <key>NSHumanReadableCopyright</key>
  <string>Copyright © 2024</string>
</dict>
</plist>
EOF

# 6. Refresh the app bundle so Spotlight recognizes it
touch "$APP_BUNDLE"
tree

echo "✅ App bundle created at: $APP_BUNDLE"
echo "🎨 Icon added from: $ICON_IMAGE"
echo "📦 You can now find and launch it via Spotlight."
