# Design Patterns in Clawd Runner

A quick tour of the patterns and techniques used in this codebase. Written for future contributors (or future selves) who want to understand the "why" behind the structure.

---

## State Machine for Player

`PlayerState` is an enum:

```rust
pub enum PlayerState {
    Running,
    Jumping,
    Ducking,
    Landing(u8), // frames remaining
}
```

Transitions happen in `update_player()`. This avoids boolean soup (`is_jumping && !is_ducking && ...`) and makes illegal states unrepresentable. The `Landing(u8)` variant carries its own timer—no separate field needed.

---

## The Game Loop

Classic structure: **input → update → render**, repeated at ~30fps.

```
loop {
    poll_input()      // non-blocking, short timeout
    game.tick()       // physics, collisions, scoring
    game.maybe_spawn_obstacle()
    terminal.draw()   // render current state
    sleep(remaining_frame_time)
}
```

Order matters. We check collisions *after* moving obstacles so positions are current. We render *after* all state updates so visuals match logic.

---

## Separation by Concern

```
src/
├── game/
│   ├── physics.rs   # movement, gravity, collision detection
│   ├── spawner.rs   # when and what to spawn
│   └── state.rs     # data structures (Player, Obstacle, GameState)
├── render/
│   ├── scene.rs     # main game widget
│   ├── sprites.rs   # character art and colors
│   └── ground.rs    # scrolling ground line
├── input/
│   └── events.rs    # keyboard handling
└── watcher/
    └── transcript.rs # file change detection
```

Render code reads state but never mutates it. Physics code doesn't know about terminals. This makes each piece testable and replaceable.

---

## Forgiving Hitboxes

Visual sprites are larger than collision boxes:

```
Small obstacle: 3x3 visual, 1x2 hitbox
Player: 7x3 visual, 5x2 hitbox (1 char inset)
```

Near-misses feel like skill, not luck. Players think "I barely made it!" instead of "that didn't touch me!" This is standard in platformers—Chrome dino does the same.

---

## Progressive Unlocks

Score thresholds gate obstacle types:

```rust
const TALL_THRESHOLD: u32 = 300;
const DOUBLE_THRESHOLD: u32 = 600;
const FLYING_THRESHOLD: u32 = 900;
```

Early game is learnable (just small obstacles). Variety increases as you prove competence. Keeps the difficulty curve smooth without explicit "levels."

---

## Frame Counters for Animation

No timers, no delta time—just frame counting:

```rust
let sprite = if self.game.frame_count % 16 < 8 {
    ClaudeSprite::FLOATING_1
} else {
    ClaudeSprite::FLOATING_2
};
```

At 30fps, this toggles every ~267ms. Deterministic, simple, no floating-point drift. Flash effects use the same pattern (`collision_flash > 0`, decrement each frame).

---

## Builder-ish Configuration

```rust
let game = GameState::new().with_terminal_width(size.width);
```

Optional configuration without constructor parameter bloat. Rust's ownership makes this clean—`with_terminal_width` takes `mut self` and returns `Self`.

---

## The Widget Trait

ratatui's core abstraction. `GameScene` implements `Widget`:

```rust
impl Widget for GameScene<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // draw ground, obstacles, player, status bar
    }
}
```

Caller just does `frame.render_widget(GameScene::new(&game), area)`. All rendering logic stays encapsulated. Easy to add new widgets (debug overlay, pause screen) without touching the main loop.

---

## Collision Tracking per Obstacle

Each obstacle tracks its own state:

```rust
pub struct Obstacle {
    pub x: f32,
    pub obstacle_type: ObstacleType,
    pub passed: bool,    // cleared the player
    pub collided: bool,  // player hit this one
}
```

This prevents weird interactions like getting +10 bonus for an obstacle you crashed into. State lives with the entity it describes.

---

## File Watching for Exit

```rust
pub fn file_changed(&self) -> std::io::Result<bool> {
    let current_size = metadata.len();
    Ok(current_size != self.initial_size || current_mtime > self.initial_mtime)
}
```

Polls file metadata each frame (cheap). Detects compaction completion by size *change* (not just increase, since compaction shrinks the transcript). No filesystem watchers, no async—just simple polling.

---

## What's NOT Here

- **ECS**: Overkill for ~10 entities. Plain structs and vecs are fine.
- **Delta time**: Fixed timestep is simpler and deterministic.
- **Dependency injection**: Single entry point, no need.
- **Event buses**: Direct function calls are traceable.

The right amount of architecture is the minimum that keeps the code clear.
