### 11/15/2022
I created a test file with a single, invisible platform hovering over the Home Run Contest (HRC) stage `GrHr.dat`
I didn't have Contango (my Windows tower computer) set up, so I spent the rest of the day setting it up with a dev environment. Side note: dev on Windows has gotten significantly better, if you're willing to take the time to set up the environment. The "Windows Terminal" in particular is very helpful.

### 11/16/2022
I used Melee Modding Wizard (MMW) to **Test External Stage File** with `GrHr.dat`.
The game boots into the Character Select Screen (CSS), so I had to navigate over to the HRC in order to view it.
The invisible platform was there, demonstrating a pipeline for building and testing stages!

I built MMW (v0.9.4) as an executable.
I can call `mmw.exe` from the command line (CLI) and in scripts to automate some of the manual steps.
Unfortunately, I got stuck at an innocuous error when I try to use `mmw.exe test -b BOOT_FILE`

```bash
loading source + preProc.txt for Asset Test
Found assembly in supposedly pre-processed code for "Asset Test"; code change at 801B148C
```

After some experimentation and digging around in the source code,
I realized that `mmw.exe` needs to have a disc loaded to do AssetTest.
Unfortunately, there is no command-line argument for `--discPath` when executing `mmw.exe test`,
so I modified the script to properly load a disc from `--discPath` and populate `globalData.disc`.
Now it boots fine and I have a working script to launch the stage for dev!

### 11/20/2022
I did some research into the other tools available in the space and watched several video tutorials for stage and character modding. Some things I learned:
* mexTool is very good for constructing a full-game modpack, and worth investigating - [ ]urther
* Blender can be used to edit models (import as DAE, then export as FBX) which can be much better than using HSDRAW
* HSDRAW is very cumbersome to use because a lot of the stuff in the file-hierarchy are unintelligible. Key points of interest are `map_head > ModelGroups` and `coll_data` since those contain the model and collision info for the stage.

I successfully made a Battlefield mod where the two side platforms' models and collision were removed.
Then I was able to boot the game using the AssetTest script I wrote the other day and indeed the platforms were gone!

### 11/21/2022
I want to be able to make stages programmatically. But first, I want to learn more about making stages by hand.
So today I'm going to try making several different basic stage mods, each one teaching a new technique:
- [x] remove platforms
- [ ] edit platform shape
- [ ] edit platform material
- [ ] edit platform animation
- [ ] add a custom platform (model, coll, mat, & anim)
- [ ] move spawn platform in VS
- [ ] move spawn point in TT and RTTF
- [ ] remove a target in TT
- [ ] move a target in TT
- [ ] add a target in TT
- [ ] move finish line in RTTF
- [ ] move finish portal in RTTF
- [ ] add an item
- [ ] add an enemy

Some of these are harder than others...

