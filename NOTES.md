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

