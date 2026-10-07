# PhotoCraft first-launch language evidence

Source: `47acd64c9a6f2bb7373a5bf79f72a920c6693d5e`. Original offscreen Windows captures of synthetic blank documents,
1440 x 900 points, scale 1, CPU canvas and installed system fonts. No personal photos
or font files are included. No language preference is set by either capture script.

- `system-auto.png`: no saved preferences or PHOTOCRAFT_LOCALE override. The native UI
  API returned zh-CN, en-US; Auto resolves to zh-hans despite LC_ALL/LANG being en_US.UTF-8.
- `unsupported-auto.png`: PHOTOCRAFT_LOCALE=de-DE is unavailable; Auto resolves to en.

In both captures, the preference remains `auto`. The Preferences dialog is opened
only to make the unchanged Auto setting visible. Its Apply button is inactive.

## Asset attribution

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `system-auto.png` | PhotoCraft first-launch UI capture | PhotoCraft contributors | Original synthetic capture at the source above | MIT OR Apache-2.0 |
| `unsupported-auto.png` | PhotoCraft first-launch UI capture | PhotoCraft contributors | Original synthetic capture at the source above | MIT OR Apache-2.0 |
