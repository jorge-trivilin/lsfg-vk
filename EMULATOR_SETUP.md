# Emulator Setup Guide for lsfg-vk

This guide explains how to configure lsfg-vk to work with emulators like PCSX2, Dolphin, and others.

## Overview

Starting from this version, lsfg-vk includes enhanced support for emulators:

1. **Automatic Detection**: Common emulators (PCSX2, Dolphin, Retroarch, etc.) now appear in the process picker even before they load Vulkan
2. **Flexible Matching**: Support for wildcard and partial matching of process names
3. **Better Compatibility**: Handles emulator variants and different binary names

## Supported Emulators

The following emulators are automatically detected:
- **PCSX2** (all variants: pcsx2-qt, pcsx2, PCSX2)
- **Dolphin** (dolphin-emu, dolphin-emu-qt, dolphin-emu-wx)
- **Retroarch**
- **RPCS3**
- **DuckStation** (duckstation-qt, duckstation)

## Setup Instructions

### Method 1: Using the Process Picker

1. Launch **lsfg-vk-ui** from your application menu
2. Click the "+" button to add a new profile
3. Click the process picker button (magnifying glass icon)
4. Your emulator should now appear in the list, even if it hasn't launched a game yet
5. Select your emulator from the list
6. Configure your preferred settings (multiplier, flow scale, etc.)
7. Save the configuration

### Method 2: Manual Configuration

If your emulator doesn't appear automatically, you can manually add it:

1. Launch **lsfg-vk-ui**
2. Add a new profile
3. In the "Process Name" field, enter the name of your emulator binary

#### Finding Your Emulator's Process Name

To find your emulator's process name:

```bash
# Launch your emulator, then run:
ps aux | grep -i "pcsx2\|dolphin\|retroarch"
```

Look for the process name in the output (usually in the last column).

### Using Wildcards

You can use wildcards in the process name for better compatibility:

- `pcsx2*` - Matches pcsx2-qt, pcsx2-wx, etc.
- `dolphin-*` - Matches dolphin-emu, dolphin-emu-qt, etc.
- `*emulator*` - Matches any process with "emulator" in its name

### Example Configurations

#### PCSX2
- **Process Name**: `pcsx2` or `pcsx2-qt` or `pcsx2*`
- **Recommended Settings**:
  - Multiplier: 3 or 4
  - Flow Scale: 0.7-1.0
  - Performance Mode: Enabled

#### Dolphin
- **Process Name**: `dolphin-emu` or `dolphin-*`
- **Recommended Settings**:
  - Multiplier: 3 or 4
  - Flow Scale: 0.7-1.0
  - Performance Mode: Enabled

#### Retroarch
- **Process Name**: `retroarch`
- **Note**: Make sure your core is configured to use Vulkan video driver
- **Recommended Settings**:
  - Multiplier: 3 or 4
  - Flow Scale: 0.7-1.0
  - Performance Mode: Enabled

## Troubleshooting

### Emulator doesn't show in process picker

1. Make sure your emulator is running
2. Check if your emulator is in the supported list
3. Try manually adding it using Method 2

### Frame generation not working

1. **Verify Vulkan is enabled**: Ensure your emulator is configured to use Vulkan renderer
   - PCSX2: Settings → Graphics → Renderer → Vulkan
   - Dolphin: Graphics → Backend → Vulkan
   - Retroarch: Settings → Video → Output → vulkan

2. **Check process name**: Make sure the process name in your configuration matches the actual running process
   ```bash
   # While your game is running:
   ps aux | grep -i <your-emulator-name>
   ```

3. **Try wildcard matching**: If the exact name doesn't work, try using wildcards (e.g., `pcsx2*`)

4. **Check logs**: Look for lsfg-vk messages in your terminal or logs:
   ```bash
   # Run your emulator from terminal to see lsfg-vk output
   LSFG_PROCESS=<emulator-name> <your-emulator-command>
   ```

### Games launch but screen is blank

This can happen if the emulator spawns child processes for rendering. Try:

1. Use a more general process name pattern with wildcards
2. Add multiple profiles for different emulator variants
3. Use the environment variable override:
   ```bash
   LSFG_PROCESS=<emulator-name> <your-emulator-command>
   ```

## Advanced Configuration

### Using Environment Variables

You can override the process name detection:

```bash
# Force lsfg-vk to activate for any Vulkan process
LSFG_PROCESS=pcsx2-qt /usr/bin/pcsx2-qt
```

### Multiple Emulator Profiles

You can create separate profiles for different emulators or even different games within the same emulator:

1. Create a profile for `pcsx2` with your general settings
2. Create another profile for `pcsx2-god-of-war` for game-specific optimizations

The most specific match will be used.

## Tips for Best Performance

1. **Start with conservative settings**: Begin with multiplier 2 or 3 and increase gradually
2. **Adjust flow scale**: If you see artifacts, reduce flow scale (0.7-0.9 works well)
3. **Enable performance mode**: This is usually beneficial for emulators
4. **Monitor frame time**: Use emulator's built-in FPS counter to verify performance

## Reporting Issues

If you encounter problems with emulator support:

1. Check that your emulator is using Vulkan renderer
2. Verify the process name using `ps aux`
3. Try with wildcard matching
4. Report the issue with:
   - Emulator name and version
   - Process name (from `ps aux`)
   - lsfg-vk configuration
   - Any error messages

## Contributing

If your emulator isn't on the supported list, you can:

1. Request it to be added by opening an issue
2. Submit a pull request adding it to `KNOWN_EMULATORS` in `ui/src/utils.rs`
