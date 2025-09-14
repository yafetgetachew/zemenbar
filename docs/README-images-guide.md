# How to Add Images to README

Here are different ways to add screenshots and images to your README.md file:

## Method 1: Local Images in Repository (Recommended)

Store images in a `docs/images/` directory and reference them like this:

```markdown
## Screenshots

![Menu Bar View](docs/images/menubar-screenshot.png)
*ZemenBar showing Ethiopian date in the macOS menu bar*

![Settings Panel](docs/images/settings-panel.png)
*Compact settings panel with language and numeral options*

![App Icon](docs/images/app-icon.png)
*ZemenBar app icon*
```

## Method 2: Using HTML for More Control

For better control over image sizing and alignment:

```markdown
## Screenshots

<div align="center">
  <img src="docs/images/menubar-screenshot.png" alt="Menu Bar View" width="600">
  <p><em>ZemenBar showing Ethiopian date in the macOS menu bar</em></p>
</div>

<div align="center">
  <img src="docs/images/settings-panel.png" alt="Settings Panel" width="400">
  <p><em>Compact settings panel with language and numeral options</em></p>
</div>
```

## Method 3: Side-by-Side Images

```markdown
## Screenshots

| Menu Bar | Settings Panel |
|----------|----------------|
| ![Menu Bar](docs/images/menubar-screenshot.png) | ![Settings](docs/images/settings-panel.png) |
| Ethiopian date in menu bar | Language and numeral options |
```

## Method 4: Using GitHub Issues/Releases (Alternative)

You can also upload images to a GitHub issue or release and use those URLs:

```markdown
![Screenshot](https://github.com/user/repo/assets/12345/image-url.png)
```

## Best Practices

1. **Image Size**: Keep images under 1MB for faster loading
2. **Format**: Use PNG for screenshots, JPG for photos
3. **Alt Text**: Always include descriptive alt text
4. **Organization**: Store images in `docs/images/` or `assets/images/`
5. **Naming**: Use descriptive filenames like `menubar-screenshot.png`

## Taking Screenshots

### macOS Screenshots:
- **Full screen**: `Cmd + Shift + 3`
- **Selection**: `Cmd + Shift + 4`
- **Window**: `Cmd + Shift + 4`, then `Space`, then click window
- **Menu bar area**: `Cmd + Shift + 4`, then select the menu bar area

### Optimizing Screenshots:
```bash
# Resize image (requires ImageMagick)
magick input.png -resize 800x600 output.png

# Compress PNG
pngquant input.png --output output.png

# Convert to WebP for smaller size
magick input.png output.webp
```

## Example README Structure

```markdown
# ZemenBar

Brief description...

## Screenshots

![Demo](docs/images/demo.gif)
*ZemenBar in action*

### Menu Bar Integration
![Menu Bar](docs/images/menubar.png)

### Settings Panel
![Settings](docs/images/settings.png)

## Features

- Feature 1
- Feature 2

## Installation

...
```
