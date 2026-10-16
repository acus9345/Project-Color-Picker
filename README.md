## what is it

its a useless color picker that incudes 14 color modes(its only useful when u dont wanna open ur browser everytime)
its the ai redesigned version of my original color picker ofc its much better for me now cuz ai is much better at this
(the original version of my code is in master branch)
- **modes included** - RGBA,HEX,HSL,HSB,CMYK,HWB,OKLCH,LAB,GRAY,YUV,YPBPR,XYZ,XYYMUNSELL

## how to use

well for window x64 just open the target/debug/too1.exe
for linux use   cargo clean + cargo run
for mac user read the macos file

## controls
(idk yea this part is ai idk why claude added this)
- **drag** the color plane, the hue / alpha bars or any channel slider; **scroll** over a slider to nudge it one step
- **Mode** opens the menu with all color modes
- **click the value box** to type a value (Enter applies, Esc cancels); `#hex` works in every mode
- **copy buttons** copy the values / the hex, **Ctrl+C** copies the values, **Ctrl+V** pastes a color
- **Pick** (or **P**) grabs a color from anywhere on screen
- **swatches**: click an empty one to save, click a saved one to use it, right-click to replace, middle-click to clear
- the **pin** in the title bar keeps the window on top
- the last color, mode and pin are remembered (`state.txt` next to the exe, swatches in `palette.txt`)
