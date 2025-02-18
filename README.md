# Hypraway

**Hypraway** is a tool to automatically lock your screen when you leave, and it integrates seamlessly with Hyprland and Wayland compositors.

## Installation

You can install **Hypraway** from the AUR (Arch User Repository) using `yay` or any other AUR helper:

```bash
yay -S hypraway
```

Alternatively, you can manually clone the AUR repository and build it:

```bash
git clone https://aur.archlinux.org/packages/hypraway.git
cd hypraway
makepkg -si
```

## Dependencies

- `glibc`
- `hyprland`
- `hyprlock`
- `swayidle`
- `cargo` (for building the package)

## Configuration

Once installed, you need to add **Hypraway** to your Hyprland configuration. You can do this by adding the following line to your `hyprland.conf`:

```ini
exec-once = hypraway
```

This will start **Hypraway** automatically when your session starts.

## Usage

Once running, **Hypraway** will monitor your system and automatically lock the screen or hibernate the machine based on the defined timeout levels. You can modify the configuration file located at:

```
~/.config/hypr/hypraway.conf
```

### Example:

You can customize your **Hypraway** settings for different power modes (AC, battery) and timeouts. The default configuration has 3 levels for both AC and battery:

- Level 1: Timeout of 600 seconds, and it will display a notification.
- Level 2: Timeout of 1200 seconds, and it will lock the screen using `hyprlock`.
- Level 3: Timeout of 0 seconds, and it will hibernate the system.

Feel free to adjust the timeouts and commands to suit your needs.

## Links

- [AUR Package](https://aur.archlinux.org/packages/hypraway)
- [GitHub Repository](https://github.com/canmi21/hypraway)
