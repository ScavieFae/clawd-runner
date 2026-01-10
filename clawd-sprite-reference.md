# Clawd Sprite Reference

Based on the official Claude Code mascot. Clawd is a chunky, friendly character rendered in pixel art style.

## Original Pixel Analysis

From the reference image, Clawd consists of:
- **Body**: Salmon/peach colored rectangular body (#D97757 or similar)
- **Eyes**: Two dark squares, positioned in upper portion of body
- **Ears/Antennae**: Two small rectangles protruding from top corners
- **Feet**: Three small rectangles at bottom (two outer feet, one middle element or shadow)
- **Background**: Dark gray (#2D2D2A)

The character appears to be roughly 13x11 pixels (excluding background), very chunky and minimal.

## Color Palette

```
Primary body:    #D97757 (salmon/peach/terracotta)
Eyes/Dark:       #2D2D2A (near black)
Background:      #2D2D2A (dark gray)
```

ANSI 256-color approximations:
- Body: Color 173 (d7875f) or 174 (d78787) 
- Eyes: Color 235 (262626) or 236 (303030)

Truecolor (24-bit) is preferred if terminal supports it.

---

## Terminal Block Character Sprites

Using Unicode block drawing characters:
- `█` (U+2588) Full block
- `▄` (U+2584) Lower half block  
- `▀` (U+2580) Upper half block
- `▌` (U+258C) Left half block
- `▐` (U+2590) Right half block
- `░` (U+2591) Light shade
- `▖▗▘▝` Quarter blocks

### Standing Pose (Idle)

**ASCII Grid (13 wide x 9 tall conceptually):**

```
    ██  ██         <- ears (2px each, gap in middle)
  ██████████       <- top of head
  ██████████       <- upper face
  ██▀▀██▀▀██       <- eyes row (eyes are dark gaps)
  ██████████       <- lower face  
  ██████████       <- body
    ██  ██         <- feet
```

**Compact 5-line version using half-blocks:**

```
 ▄█▄▄▄▄█▄
 █▀▀██▀▀█
 ████████
 ▀▄████▄▀
   █  █
```

**Even more compact 4-line:**

```
▄█▀▀▀▀█▄
█ ▀▀▀▀ █
█▄████▄█
  ▀  ▀
```

**Simplest 4-line (game-ready):**

```
 ▟█▀▀█▙
 █ ▀▀ █
 ▜████▛
  █  █
```

---

### Recommended Game Sprite (7 chars wide, 3 lines tall)

The final design—compact, expressive, with eyes created by gaps at the top of ▅ blocks:

```
▗█▀█▀█▖
 █▅█▅█
  ▀ ▀
```

Quarter-block ears (▗ ▖), eyes as negative space in the ▅ blocks, feet centered under the eyes.

---

## Animation Frames

### Running Frame 1 (feet together)
```
▗█▀█▀█▖
 █▅█▅█
  ▀ ▀
```

### Running Frame 2 (feet apart)
```
▗█▀█▀█▖
 █▅█▅█
 ▀   ▀
```

### Jumping (feet tucked)
```
▗█▀█▀█▖
 █▅█▅█

```

---

## Implementation Notes

### Rendering with ANSI Colors

```rust
// Rust example with crossterm
use crossterm::style::{Color, SetForegroundColor, SetBackgroundColor};

const CLAWD_COLOR: Color = Color::Rgb { r: 217, g: 119, b: 87 }; // #D97757
const EYE_COLOR: Color = Color::Rgb { r: 45, g: 45, b: 42 };     // #2D2D2A

fn render_clawd(frame: usize) {
    print!("{}", SetForegroundColor(CLAWD_COLOR));
    // ... render sprite lines
    print!("{}", SetForegroundColor(Color::Reset));
}
```

### Character Width Considerations

Some Unicode block characters may render as different widths in different terminals/fonts. Test with:
- iTerm2 / Terminal.app (macOS)
- Windows Terminal
- GNOME Terminal / Konsole (Linux)
- Alacritty / Kitty

If half-blocks cause alignment issues, fall back to full blocks only:

**Full-block fallback sprite:**
```
 ██  ██
████████
██ ▀▀ ██
████████
 ██  ██
```

### Size Recommendations

For a game in ~6-8 terminal rows:
- Sprite height: 4-5 lines (leaves room for ground + status)
- Sprite width: 6-8 characters
- Ground line: 1 line of `═` or `─`
- Status bar: 1 line at bottom

---

## Obstacle Sprites

### Small Cactus/Spike (3 wide, 3 tall)
```
 ▄
▄█▄
 █
```

### Tall Cactus (3 wide, 4 tall)
```
 ▄
▄█▄
 █
 █
```

### Rock/Block (4 wide, 2 tall)
```
▄██▄
████
```

Use same dark color as eyes (#2D2D2A) for obstacles to create contrast.

---

## Final Recommended Sprite Set

**For the game, use this minimal set:**

### CLAWD_RUN_1 (7x3) - feet together
```
▗█▀█▀█▖
 █▅█▅█
  ▀ ▀
```

### CLAWD_RUN_2 (7x3) - feet apart
```
▗█▀█▀█▖
 █▅█▅█
 ▀   ▀
```

### CLAWD_JUMP (7x3)
```
▗█▀█▀█▖
 █▅█▅█

```

### OBSTACLE_SMALL (3x3)
```
 █
███
 █
```

### OBSTACLE_TALL (3x4)
```
 █
███
 █
 █
```

---

## Alternative: Higher-Fidelity Sprite

If terminal supports it and you want more detail (8 wide, 5 tall):

```
 ██    ██ 
 ████████ 
 █ ▀▀▀▀ █ 
 ████████ 
  ██  ██  
```

This more closely matches the reference image proportions with:
- Visible ear nubs at top corners
- Clear eye holes
- Chunky body
- Two distinct feet

---

## Quick Copy-Paste Test

Run this in your terminal to preview:

```bash
echo -e "\033[38;2;217;119;87m"
echo "▗█▀█▀█▖"
echo " █▅█▅█"
echo "  ▀ ▀"
echo -e "\033[0m"
```

If colors work, you'll see Clawd in salmon/peach!
