# Clawd Runner - Game Design Document

A Chrome dino-style endless runner for Claude Code's compaction wait screen. Built in Rust with ratatui/crossterm for inline terminal rendering.

---

## Philosophy

**"Maximum output from minimum input."**

The Chrome dino team [deliberately kept the motion rigid](https://blog.google/products-and-products/products/chrome/chrome-dino/), reminiscent of vintage games. They rejected ideas like cute kicks or roars, settling on "the basics of any good runner game: run, duck and jump." That restraint is the template here.

The game exists to make waiting pleasant, not to demand attention. It should feel good to play but never punish you for looking away.

---

## Current State

### What's Built

| Component | Status | Notes |
|-----------|--------|-------|
| Core loop | Done | ~30fps, terminal rendering |
| Player physics | Done | Gravity, jump velocity, ground detection |
| Obstacles | Done | Small, Tall, Double, Flying variants |
| Collision | Done | AABB with forgiving hitboxes |
| Scoring | Done | +1/frame + 10 bonus per obstacle cleared |
| Run animation | Done | 2-frame cycle, feet in/out |
| Jump animation | Done | Feet tuck in air |
| Landing squash | Done | 3-frame feet-tucked on landing |
| Duck mechanic | Done | Toggle duck, 2-frame waddle animation |
| Flying obstacles | Done | Appear at 1500+ score, duck to avoid |
| Milestone flash | Done | Score flashes at 100, 500, 1000, etc. |
| Obstacle curve | Done | Gradual introduction by score |
| Hook integration | Done | PreCompact launches game in new Terminal |

### The Clawd Sprite

**Running** (2 frames, feet alternate in/out):
```
▗█▀█▀█▖    ▗█▀█▀█▖
 █▅█▅█      █▅█▅█
  ▀ ▀       ▀   ▀
```

**Jumping** (feet tucked):
```
▗█▀█▀█▖
 █▅█▅█

```

**Landing** (3 frames, feet tucked):
```
▗█▀█▀█▖
 █▅█▅█
```

**Ducking** (2 frames, feet alternate):
```
▗█▀█▀█▖    ▗█▀█▀█▖
 ▀▔▔▔▀      ▔▀▔▀▔
```

7 chars wide. Claude's brand salmon (#D97757). Eyes are gaps at top of ▅ blocks.

### Current Obstacles

```
Small:    Tall:     Double:    Flying:
  █         █        █   █       \█/
 ███       ███      ███ ███       █
  █         █        █   █
            █
```

Light gray (#B4B4AA). Flying obstacles positioned above ground—duck to avoid.

### Obstacle Introduction Curve

| Score | Available Types |
|-------|-----------------|
| 0-299 | Small only |
| 300-599 | Small, Tall |
| 600-899 | Small, Tall, Double |
| 900+ | Small, Tall, Double, Flying |

---

## Controls

### Current
- **Space / Up / W / K**: Jump
- **Down / S / J**: Toggle duck
- **Q / Escape**: Quit

### Design Notes

The Chrome dino works because [it's frictionless UX](https://norbertsflow.com/reviews/why-dino-chrome-became-more-than-just-a-no-internet-game/)—zero instructions needed. Our controls follow that:

1. **Two actions**: Jump and duck. Duck added because flying obstacles demand it.
2. **Multiple keys**: Space is intuitive, but vi users expect K/J. Cover the bases.
3. **Toggle duck**: Key release events are unreliable cross-platform (require kitty protocol), so duck is press-to-toggle rather than hold.
4. **Jump exits duck**: Jumping from ducked state works naturally.

---

## Juiciness in Terminal Constraints

Traditional game juice relies on particles, screen shake, squash-and-stretch. Terminal rendering limits us to:

- Character-based "pixels"
- Limited color (though we have true color support)
- No sub-cell positioning
- No transparency/blending

But [juice is about feel, not fidelity](https://sefaertunc.medium.com/game-design-series-ii-game-juice-92f6702d4991). Here's what we *can* do:

### Currently Implemented
- **Run cycle animation**: Feet alternate in/out every 8 frames
- **Duck waddle**: Feet alternate while ducking
- **Jump pose change**: Feet tuck when airborne
- **Landing squash**: 3 frames with feet tucked on landing
- **Scrolling ground**: Creates sense of motion
- **Milestone flash**: Score flashes yellow/salmon at 100, 500, 1000, etc.
- **Collision flash**: `collision_flash` field exists (visual TBD)

### Low-Hanging Fruit (Remaining)

| Effect | Effort | Impact | How |
|--------|--------|--------|-----|
| Score pop | Low | Medium | Flash score brighter when +10 bonus hits |
| Jump anticipation | Low | Medium | 1-frame crouch before launch |
| Speed lines | Medium | Medium | Add `>` or `-` chars behind player at high speed |
| Day/night cycle | Medium | Medium | Chrome dino does this—color scheme swap |
| Collision visual | Low | Medium | Actually render the flash on collision |

### Squash and Stretch (Terminal Edition)

From [juicing principles](https://www.gameanalytics.com/blog/squeezing-more-juice-out-of-your-game-design/): squash-and-stretch conveys weight and energy.

In terminal:
- **Squash**: Wider, shorter sprite (land impact)
- **Stretch**: Taller, narrower sprite (peak of jump)

Example squash frame:
```
▗█▀▀▀▀█▖
█ ▀▀▀▀ █
████████
```

This is subtle but sells the physics.

### What NOT to Do

Per [game feel research](https://medium.com/swlh/what-makes-for-good-visual-game-juice-e63cb8ba2068):
- Don't add juice that doesn't match the game's tone (we're chill, not chaotic)
- Don't juice secondary actions more than primary (jumping > everything else)
- Screen shake would be overkill for this context—user is waiting, not raging

---

## Difficulty Curve

### Current Implementation
- Speed starts at 1.0, increments by 0.0005 per frame
- Caps at 2.5x
- Obstacle spawn is random (needs review)

### Chrome Dino's Approach

[Speed increases gradually](https://www.oreateai.com/blog/indepth-analysis-gameplay-and-hacking-techniques-for-the-hidden-dinosaur-game-in-google-chrome/7a767d646b53f4ec3479ff2917fff22e), and "the challenge lies in adapting to the increasing speed." The curve is smooth enough that you don't notice it happening.

### Recommendations

1. **Slower initial ramp**: Current 0.0005/frame feels right, but cap could go higher (3.0?)
2. **Obstacle density curve**: Spawn rate should increase with speed, not independently
3. **Pattern introduction**: Early game = only Small obstacles. Tall after 500 points. Double after 1000.
4. **Breather moments**: Occasional long gaps even at high speed (prevents exhaustion)

---

## Obstacle Design

### Current Types

| Type | Visual Size | Hitbox | Jump Clearance |
|------|-------------|--------|----------------|
| Small | 3x3 | 1x2 | Easy |
| Tall | 3x4 | 1x3 | Medium |
| Double | 7x3 | 5x2 | Hard (width) |

### Hitbox Philosophy

Hitboxes are intentionally smaller than visuals. [Forgiving collision](https://blog.google/products-and-products/products/chrome/chrome-dino/) makes near-misses feel like skill, not luck. Current implementation: 1 char inset on all sides.

### Potential New Obstacles

| Type | Description | Requires |
|------|-------------|----------|
| Gap | Missing ground (auto-death?) | Death state |
| Flying | Pterodactyl equivalent | Duck mechanic |
| Moving | Obstacle that bounces or shifts | More complex physics |

**Opinion**: Stick with ground obstacles. Flying requires duck, moving adds complexity. The game is for idle waiting—don't make it require full attention.

---

## Score & Progression

### Current
- +1 per frame survived
- +10 bonus when obstacle fully passed

### Observations
- No persistence (score resets each session)
- No milestones or achievements
- No high score tracking

### Recommendations

1. **Visual milestones**: Flash at 100, 500, 1000, etc. Chrome dino does a satisfying chirp.
2. **Session high score**: Track in memory, show "NEW HIGH" on beat
3. **Persistent high score**: Write to `~/.claude/clawd_high_score` (optional, maybe overkill)

---

## Audio (Terminal Constraints)

Terminal can technically emit audio via:
- System bell (`\x07`)
- `osascript` to trigger system sounds (macOS)
- External process

**Opinion**: Skip audio. It's a background distraction game. Sound would be jarring in a coding context. The original Chrome dino works fine muted.

---

## Exit Conditions

### Current
- Manual quit (Q/Escape)
- Timeout (default 300s)
- Transcript file changes (compaction complete)
- Terminal too small

### No Death State

The spec says no death state—collision flashes but game continues. This is right for the use case. Death would require:
- Game over screen
- Restart prompt
- Attention at the wrong moment

The game should just... keep going. Worst case, you miss some obstacles. Best case, you have a zen moment during compaction.

---

## Implementation Priorities

### Completed
- [x] Landing squash animation (3-frame feet tuck)
- [x] Milestone flash at 100/500/1000/etc
- [x] Obstacle pattern introduction curve
- [x] Duck mechanic + flying obstacles

### Next Steps: Polish
- [ ] Score pop on bonus (+10 flash)
- [ ] Collision visual (render the flash)
- [ ] Jump anticipation frame (1-frame crouch before launch)
- [ ] Session high score tracking

### Future: Feel
- [ ] Stretch at jump apex
- [ ] Speed lines at high velocity
- [ ] Day/night palette swap

### Maybe Someday
- [ ] Persistent high score (`~/.claude/clawd_high_score`)
- [ ] More obstacle variety
- [ ] Sound effects (probably not—context is coding)

---

## References

- [Google's Chrome Dino origin story](https://blog.google/products-and-products/products/chrome/chrome-dino/)
- [Game Juice principles](https://sefaertunc.medium.com/game-design-series-ii-game-juice-92f6702d4991)
- [Squeezing more juice out of your game](https://www.gameanalytics.com/blog/squeezing-more-juice-out-of-your-game-design/)
- [Visual game juice deep dive](https://medium.com/swlh/what-makes-for-good-visual-game-juice-e63cb8ba2068)
- [Why Dino Chrome endures](https://norbertsflow.com/reviews/why-dino-chrome-became-more-than-just-a-no-internet-game/)

---

*Document created January 2026. Updated January 10, 2026 after duck/flying/juice pass.*
