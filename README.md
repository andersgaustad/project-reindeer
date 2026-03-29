# Project Reindeer
**Project Reindeer** is a simple game/prototype/proof-of-concept meant to visualize my solution for [Day 16 (Reindeer Maze) of Advent of Code 2024](https://adventofcode.com/2024/day/16).

What started out as a simple idea of seeing how the reindeer would jump around tiles to navigate the maze quickly outgrew its scope as I thought of new features to add (I assure you, dear reader, that joypad support with configurable rebinding was absolutely essential for this project) and discovered fun and thematically fitting game assets online.


## How to Play

### Dependencies (Playing)
None.

**Supported Platforms:**
- Linux (x86_64)
- Windows
- MacOS (**Experimental**)


### Playing

1. Go to [the Project Reindeer itch.io page](https://andersgaustad.itch.io/project-reindeer).
2. Download the version that fits your operating system, and extract the `.zip` to a folder of your choice.
    - If on Mac, contents should not be extracted in Downloads as [Gatekeeper breaks Godot's internal paths due to security reasons](https://docs.godotengine.org/en/latest/tutorials/export/running_on_macos.html#doc-running-on-macos). To resolve this, extract or move the project to e.g., the `/Applications` folder.
3. Run the game by running (e.g., double-clicking) the `ProjectReindeer` application/binary/executable. 

First, create your own maze or load the default one by pressing `Load Maze` or `Load Default`.
If the maze is valid, you should be spawned into a forest clearing with the maze in the middle.
Use WASD to move, and Spacebar and Left Control to ascend/descend.
Pause the game with Escape.
See Controls in the in-game menu for all controls.



## How to Access Project

### Dependencies (Accessing Project)

| Dependency  | Version |
| --- | ------- |
| [Cargo/rustc](https://rust-lang.org/learn/get-started/)       | 1.94.1+ |
| [git-lfs](https://git-lfs.com/) | 3.6.1+ |
| [Godot](https://godotengine.org/) | 4.5 |

> ^ Project Reindeer is made with [Godot 4.5](https://godotengine.org/download/archive/), though newer version of [Godot](https://godotengine.org/) might also work fine.
Earlier versions of rustc and git-lfs might also work, but this is not guaranteed.


### Building

1. Download the project (either by `git clone` or downloading a release).
2. Navigate to the root of this project (i.e., the location of this README.md).
3. Build the project using `cargo build`.
    - This is needed in order to register all Rust-defined classes.
4. Open the project by opening the Godot editor and selecting [project.godot](godot/project-reindeer/project.godot).


## Acknowledgements

This project is written almost exclusively in Rust, and is thus highly dependant on the excellent work on Rust bindings for Godot by the [godot-rust team](https://github.com/godot-rust/gdext).

All assets, art, music, fonts, and sound effects are created by volunteers that have chosen to release their creation under an Open Source License. Huge thanks to the contributers on sites like https://opengameart.org/, https://sketchfab.com/feed, https://itch.io/, and a special thanks to Kenney of https://kenney.nl/ whose 3D models serve as the foundation of this project.

All contributers as well as the Licenses used are listed [here](godot/project-reindeer/about/credits/CREDITS.md).
