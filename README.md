# Cardwire Tray

<img width="846" height="341" alt="image" src="https://github.com/user-attachments/assets/4b083b1d-dea3-473b-b3ac-d0c9def6408a" />

### An universal tray applet for [Cardwire](https://github.com/OpenGamingCollective/cardwire) GPU manager.

This applet implements all current Cardwire features including mode switching between integrated, hybrid, and manual mode. It also supports manual GPU blocking while in manual mode. Hovering over the tray icon shows all info about your GPUs such as their name, power state, and block status.

## Install

I release Flatpak bundles for this applet using GitHub Releases. If you don't have Flatpak, build and compile using make.

Dependencies: `rust`, `cargo`

```bash
make
sudo make install
```
