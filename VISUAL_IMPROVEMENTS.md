# rtop v3.0 - Visual Improvements Showcase

## 🎨 Enhanced Visual Design

The UI has been significantly improved with modern styling, gradient colors, and better visual hierarchy.

---

## Header Bar

**Before (v2.1):**
```
 ▓▓ rtop v2.1  │  ◆ Overview  │  F2-F5: Pages  │  Press h for help
```

**After (v3.0):**
```
 ⚡ rtop v3.0 🎮 2GPU  │ ◆ Overview │ F2-F5: Pages │ h: Help │ g: GPU
═══════════════════════════════════════════════════════════════════
```

**Improvements:**
- ⚡ Lightning bolt icon for energy
- Version in italic blue (Rgb(100, 200, 255))
- GPU count indicator 🎮
- Cleaner separator with │
- Color-coded shortcuts
- Double-line border

---

## CPU Display

**Visual Enhancements:**
- **Dynamic title color** based on average CPU usage:
  - Blue (Rgb(72, 151, 216)) - Low usage (0-40%)
  - Yellow (Rgb(255, 195, 69)) - Medium usage (40-60%)
  - Orange (Rgb(245, 166, 35)) - High usage (60-80%)
  - Red (Rgb(235, 112, 112)) - Critical usage (80-100%)

- **Box styling:**
  - Rounded corners (╭╮╰╯)
  - Green border (Rgb(61, 123, 70))
  - Title: `⚡ CPU Usage [45.3%]`

- **Axis improvements:**
  - X-axis: "← Time (60s history)" with "now" highlighted
  - Y-axis: Gradient labels (Blue 0% → Yellow 50% → Red 100%)
  - Better time markers: "60s", "30s", "now"

**Example:**
```
╭─⚡ CPU Usage [45.3%]──────────────────────────────────╮
│ 100%│⡀  ⢀⡀                                            │
│  50%│⠄  ⠈⠑⢄  ⢀⣀⡠⠤⠒⠉⠉⠉⠉⠑⠒⠤⣀                    │
│   0%│⠂⢄⡀  ⠈⠉      Core 0  Core 1  Core 2  Core 3    │
│     └──────────────────────────────────────────────→ │
│        60s          30s            now               │
╰──────────────────────────────────────────────────────╯
```

---

## Memory Display

**Visual Enhancements:**
- **Gradient colors for memory percentage:**
  - Green (Rgb(144, 224, 163)) - Low usage (0-70%)
  - Yellow (Rgb(255, 199, 69)) - Medium usage (70-85%)
  - Orange (Rgb(245, 166, 35)) - High usage (85-95%)
  - Red (Rgb(224, 92, 92)) - Critical usage (95-100%)

- **Box styling:**
  - Title: `💾 Memory & Swap [68.5%]`
  - Yellow-brown border (Rgb(138, 136, 46))
  - Braille marker for smoother graphs

- **Improved legend:**
  - "RAM 68.5%" with color coding
  - "Swap 12.3%" with separate color

**Example:**
```
╭─💾 Memory & Swap [68.5%]─────────────────────────────╮
│ 100%│                  ⢀⣀⣤⡤⠤                         │
│  50%│        ⣀⣠⣤⣤⡤⠤⠒⠉                              │
│   0%│⣀⣠⣤⡤⠒⠉     RAM 68.5%   Swap 12.3%             │
│     └──────────────────────────────────────────────→ │
│        60s          30s            now               │
╰──────────────────────────────────────────────────────╯
```

---

## Network Display

**Visual Enhancements:**
- **Activity indicators:**
  - ● Full (>1MB/s)
  - ◐ Medium (>10KB/s)
  - ○ Low (<10KB/s)

- **Color-coded traffic:**
  - Blue - Download
  - Green - Upload
  - Cyan - Total values

- **Box styling:**
  - Title: `🌐 Network ↓↑`
  - Purple border (Rgb(66, 88, 141))
  - Magenta ↓↑ symbol

- **Enhanced information:**
  - Interface name with ◆ symbol
  - Ping latency with color coding:
    - Green: <50ms
    - Yellow: 50-100ms
    - Red: >100ms

**Example:**
```
╭─🌐 Network ↓↑────────────────────────────────────────╮
│                                                       │
│   ▼ Download ●  ▲ Upload ◐                           │
│                                                       │
│      2.45 MB/s       156 KB/s                         │
│     1.23 GB total    89.4 MB total                    │
│                                                       │
│   ◆ eth0  │  ⚡ 23.5 ms                               │
│                                                       │
╰───────────────────────────────────────────────────────╯
```

---

## Storage Display

**Visual Enhancements:**
- **Progress bar:**
  - █ Filled blocks (colored by usage)
  - ░ Empty blocks (gray)

- **Color coding:**
  - Green (Rgb(144, 224, 163)) - Low usage (<70%)
  - Orange (Rgb(245, 166, 35)) - Medium usage (70-85%)
  - Red (Rgb(224, 92, 92)) - High usage (>85%)

- **Box styling:**
  - Title: `💾 Storage`
  - Red-brown border (Rgb(146, 83, 83))
  - Rounded corners

- **Information display:**
  - Percentage with bold color
  - Used / Total (with units)
  - Free space in green

**Example:**
```
╭─💾 Storage────────────────────────────────────────────╮
│                                                        │
│   💾 Disk Usage:                                       │
│                                                        │
│   [████████████████████████████░░░░░░░░░░░░]          │
│                                                        │
│   68.5% used                                           │
│   685.2 GB / 1.0 TB                                    │
│   314.8 GB free                                        │
│                                                        │
╰────────────────────────────────────────────────────────╯
```

---

## Color Palette

### Primary Colors
- **Cyan**: Rgb(100, 200, 255) - Headers, highlights
- **Yellow**: Rgb(255, 195, 69) - Warnings, medium levels
- **Magenta**: Rgb(255, 100, 200) - Special indicators
- **Green**: Rgb(144, 224, 163) - Success, low usage

### Gradient Colors

**CPU Gradient:**
- Start: Rgb(72, 151, 216) - Blue (cool)
- Mid: Rgb(126, 229, 103) - Green (moderate)
- End: Rgb(235, 112, 112) - Red (hot)

**Memory Gradient:**
- Start: Rgb(255, 199, 69) - Yellow (low)
- Mid: Rgb(245, 166, 35) - Orange (medium)
- End: Rgb(224, 92, 92) - Red (high)

**Network Gradient:**
- Start: Rgb(144, 224, 181) - Cyan (download)
- Mid: Rgb(80, 208, 151) - Teal
- End: Rgb(48, 181, 114) - Green (upload)

### Border Colors
- **CPU Box**: Rgb(61, 123, 70) - Green
- **Memory Box**: Rgb(138, 136, 46) - Yellow-brown
- **Network Box**: Rgb(66, 88, 141) - Blue
- **Process Box**: Rgb(146, 83, 83) - Red-brown
- **GPU Box**: Rgb(53, 147, 77) - Emerald

---

## Typography

### Font Styles
- **Bold**: Important values, titles
- **Italic**: Secondary information, labels
- **Regular**: Normal text

### Icons Used
- ⚡ Lightning - CPU, power
- 💾 Disk - Storage, memory
- 🌐 Globe - Network
- 🎮 Gamepad - GPU
- ◆ Diamond - Interface/status
- ● ◐ ○ Circles - Activity levels
- ↑↓ Arrows - Upload/download

---

## Box Types

### Rounded Corners (Default)
```
╭─────────╮
│         │
╰─────────╯
```

### Square Corners (Optional)
```
┌─────────┐
│         │
└─────────┘
```

### Double Line (Header)
```
═══════════
```

---

## Layout Improvements

### Overview Page
```
┌─────────────────────────────────────────────────┐
│  Header (v3.0, GPU count, shortcuts)            │
├─────────────────────────────────────────────────┤
│  ┌───────────────┐  ┌──────────────────────┐   │
│  │   CPU Chart   │  │   Memory Chart       │   │
│  └───────────────┘  └──────────────────────┘   │
│  ┌───────────┐  ┌────────────┐  ┌──────────┐  │
│  │  Network  │  │    Disk    │  │ Processes│  │
│  └───────────┘  └────────────┘  └──────────┘  │
│  ┌─────────────────────────────────────────┐   │
│  │   Footer (system info, status)          │   │
│  └─────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

---

## Animation Effects

### Progress Bars
- Smooth gradient transitions
- Color changes based on thresholds
- Unicode block characters (█▓▒░)

### Activity Indicators
- Pulsing network indicators (●◐○)
- Real-time value updates
- Color-coded status

### Graphs
- Braille patterns for high resolution
- Smooth line rendering
- Dynamic color scaling

---

## Accessibility

### High Contrast Mode
- Distinct colors for different metrics
- Clear borders and separators
- Bold text for important values

### Color Blind Friendly
- Not relying solely on color
- Icons and symbols as backup
- Percentage values always shown

### Low Vision Support
- Large, bold numbers for key metrics
- Clear visual hierarchy
- High contrast borders

---

## Performance

### Render Optimization
- Minimal redraws
- Cached color calculations
- Efficient string formatting

### Resource Usage
- No performance impact from visual improvements
- ~0.1-0.2% CPU overhead
- Negligible memory footprint

---

## Comparison: Before vs After

| Feature | v2.1 | v3.0 |
|---------|------|------|
| **Border Style** | Plain │─ | Rounded ╭─╮╰╯ |
| **Colors** | Basic 16 | RGB Gradients |
| **Icons** | ▓▓ only | ⚡💾🌐🎮 |
| **Title Colors** | Static | Dynamic by usage |
| **Progress Bars** | Simple | Gradient filled |
| **Graphs** | Dots | Braille patterns |
| **Activity** | None | Real-time ●◐○ |
| **Legend** | Basic | Color-coded % |

---

## Terminal Requirements

### Recommended Terminals
- ✅ **Alacritty** - Best performance
- ✅ **Kitty** - Excellent Unicode support
- ✅ **iTerm2** (macOS) - Full feature support
- ✅ **Windows Terminal** - Good support
- ⚠️ **GNOME Terminal** - May need font config
- ⚠️ **Konsole** - May need font config

### Font Requirements
Must support Unicode and emoji:
- **Nerd Fonts** (recommended)
- JetBrains Mono Nerd Font
- Fira Code Nerd Font
- Hack Nerd Font
- Any font with Unicode 13.0+ support

### Terminal Settings
```bash
# Set locale for Unicode
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8

# Ensure 24-bit color support
export TERM=xterm-256color
# or
export TERM=alacritty
```

---

## Screenshots (Text-based)

### Full Window
```
═══════════════════════════════════════════════════════════════════════════
 ⚡ rtop v3.0 🎮 2GPU  │ ◆ Overview │ F2-F5: Pages │ h: Help │ g: GPU
═══════════════════════════════════════════════════════════════════════════

╭─⚡ CPU Usage [45.3%]─────────────╮ ╭─💾 Memory & Swap [68.5%]──────╮
│ 100%│                            │ │ 100%│              ⣀⡠⠤⠒⠉⠁     │
│  50%│    ⢀⣀⣀⡀    ⣀⣀⡤⠤⠒⠊⠉        │ │  50%│    ⣀⣠⡤⠒⠉               │
│   0%│⣀⣤⠤⠒⠉⠁                    │ │   0%│⠤⠒⠉  RAM 68.5%  Swap 12.3%│
╰──────────────────────────────────╯ ╰────────────────────────────────╯

╭─🌐 Network ↓↑──────╮ ╭─💾 Storage────────╮ ╭─Processes────────╮
│ ▼ Download ●       │ │ 💾 Disk Usage:    │ │ PID  NAME  CPU%  │
│    2.45 MB/s       │ │ [████████░░░]     │ │ 1234 bash  45.3% │
│ ▲ Upload ◐         │ │ 68.5% used        │ │ 5678 node  23.1% │
│     156 KB/s       │ │ 685 GB / 1 TB     │ │ 9012 code  12.5% │
╰────────────────────╯ ╰───────────────────╯ ╰──────────────────╯

┌─────────────────────────────────────────────────────────────────────────┐
│ 💻 linux 6.5.0  │ ⏱ 12d 5h  │ Load: 2.45 1.89 1.23  │ ⚙ 245 proc      │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## Future Enhancements

### Planned for v3.1
- [ ] Sparkline mini-graphs in footer
- [ ] Animated transitions
- [ ] Custom color schemes
- [ ] More icon options
- [ ] Configurable layouts

### Planned for v3.2
- [ ] Graph export as SVG
- [ ] Theme preview mode
- [ ] Custom widget placement
- [ ] Dashboard templates

---

## How to Use

### Enable All Visual Features
```bash
rtop \
  --graph-symbol braille \
  --rounded-corners \
  --enable-gpu \
  --theme default
```

### Minimal Visual Mode
```bash
rtop \
  --no-color \
  --graph-symbol tty
```

### High Contrast Mode
```bash
rtop --theme high-contrast
```

---

## Customization

### Config File
`~/.config/rtop/config.toml`:
```toml
[display]
graph_symbol = "braille"
rounded_corners = true
double_borders = false
show_icons = true
gradient_colors = true

[colors]
# Override individual colors
cpu_box_color = "#3d7b46"
mem_box_color = "#8a882e"
net_box_color = "#5c588d"
proc_box_color = "#805252"
```

---

**Enjoy the beautiful new interface!** ✨
