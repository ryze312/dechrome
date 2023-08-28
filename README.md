# Dechrome

Dechrome is a tool written in Rust for batch removing Chromium-based browsers and installing Firefox as a replacement.

> [!WARNING]
> The script is experimental and wasn't thoroughly tested on all Windows systems.

> [!NOTE]
> Executing as administrator is preferred in order to remove system-wide installations. Make sure to terminate msedge.exe processes through task manager before launching, as the script might fail to delete certain files locked by Edge

# Reasoning

Chromium-based browsers hold around 74.05% market share across all devices, 78.58% on desktop, 90.45%* on Windows, according to [GlobalStats statcounter](https://gs.statcounter.com/browser-market-share/desktop/worldwide/#monthly-202307-202307-bar) as of July 2023.
> *Calculated by excluding Safari market share from browser market share on desktop, may be inaccurate

This causes multiple issues:
1. Increased attack surface
2. Illusion of choice
3. Single entity control

### Increased attack surface
Having a single product spread across millions of machines imposes a severe risk in case of discovered vulnerability as the area of attack could be worldwide and the patches could take a long time to propagate, giving opportunity for attackers to take advantage of the situation.

### Illusion of choice
Users are given illusion of choice, no matter what they pick they are likely to end up using Google's product, either [Blink](https://www.chromium.org/blink) or Chromium, since most browsers are based on them and are not advertised as such.

### Single entity control
While Chromium project is open-source, ultimately Google is in full control of the changes being made to it. Giving away control of the web client to Google, entity controlling most of the web space, gives it the ability to shift and control market to it's will, both client-side and server-side. This provokes monopoly and hurts competition in the long run. See recent [Web Environment Integrity proposal](https://github.com/RupertBenWiser/Web-Environment-Integrity/blob/main/explainer.md).

# Implemented uninstallers
- Google Chrome
- Google Chrome Canary
- Microsoft Edge
- Brave
- Vivaldi
- Opera
- Opera GX
- Yandex Browser

# Contributing
All issues and pull requests are welcome! Feel free to open an issue if you've got an idea or a problem. You can open a pull request if you are able to implement it yourself.

---
<p align="center">
<sub><strong>
    Made with ponies and love!
    <br/>
    GNU GPL © Ryze 2023
</strong></sub>
</p>
