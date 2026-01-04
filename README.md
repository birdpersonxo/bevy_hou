# bevy_hou
<p align="center">
  <img src="assets/houdini-icon.png" alt="Houdini Icon" width="33" style="display:inline-block; vertical-align:middle;" />
  <img src="assets/bevy-logo.png" alt="Bevy Icon" width="33" style="display:inline-block; vertical-align:middle; margin-left:8px;" />
</p>

`bevy_hou` is a Bevy module for importing and using level-layout data authored in Houdini. The current focus of the project is simple **2D platforming geometry**, represented as rectangles. Custom geometry, usd and gltf coming soon.


This project is **early-stage and experimental**.

## Features
- [x] Load Simple Rectabl Geometry
- [ ] Load custom geometry shapes
- [ ] Metadata per geometry (tags, layers, gameplay properties)
- [ ] Automatic collider generation based on tags
- [ ] Better integration with Bevy ECS workflows
- [ ] Better file format that replaces JSON

## Installation
` cargo add bevy_hou`

## Basic Usage (High-Level)

1. Use Houdini with the [houdini_bevy](https://github.com/birdpersonxo/houdini_bevy) tools to draw 2D platform rectangles
2. Export the data
3. Load the exported data into Bevy using `bevy_hou`
4. Use `HouRect` data for gameplay, collisions, or level logic


## Examples
- [Load HouRect As Mesh](examples/load_hourect.rs)
