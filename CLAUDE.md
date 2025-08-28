# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
This is a Back Health Timer Application (허리디스크 예방 프로젝트) hosted on GitHub Pages. It reminds users to take regular stretching breaks to prevent back problems.

## Technology Stack
- Pure HTML/CSS/JavaScript (no frameworks or build tools)
- Audio files: MP3 format for alerts
- Browser LocalStorage for user preferences
- GitHub Pages hosting (automatic deployment from main branch)

## Development Commands
This is a static website with no build process:
- **Run locally**: Open `index.html` directly in a browser or use a local server (e.g., `python -m http.server`)
- **Deploy**: Push to main branch (GitHub Pages auto-deploys)
- **Test**: Manual testing in browser (no test framework)

## Code Architecture
Single-file application (`index.html`) containing:
- **HTML**: Timer interface with interval selection, volume control, and start/stop buttons
- **CSS**: Inline styles for simple, centered layout
- **JavaScript**: Timer logic with three main components:
  1. **Main Timer**: Counts down to work interval (30min or 1hour)
  2. **Break Timer**: 1-minute break after each interval
  3. **Display Timer**: Updates countdown display every second

Key JavaScript patterns:
- LocalStorage keys: `selectedInterval` (30/60), `alarmVolume` (0-1)
- Audio preloading for `finish.mp3`, dynamic loading for interval alerts
- Timer cleanup on stop to prevent memory leaks
- Async audio playback with error handling

## Audio Files
- `1hour.mp3`: Plays after 1-hour work interval
- `30min.mp3`: Plays after 30-minute work interval  
- `finish.mp3`: Plays when break ends (preloaded)

## Important Considerations
- Korean language interface and documentation
- No external dependencies or CDNs
- Mobile-responsive viewport meta tag included
- Audio playback may be blocked by browser autoplay policies