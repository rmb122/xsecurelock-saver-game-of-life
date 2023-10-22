# xsecurelock-saver-game-of-life

A [xsecurelock](https://github.com/google/xsecurelock) screensaver module that can run [game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).  

<video src='https://user-images.githubusercontent.com/33388702/175051232-42a6ca75-e945-4aac-a9cc-64e88b6bf635.mp4'></video>

## Usage

Clone this repository and run `cargo build --release` to get compiled bianry.  
Add `export XSECURELOCK_SAVER="${PATH_OF_COMPILED_BINARY}"` to your lock script.  

```
#!/bin/sh
# omit other settings ... 

export XSECURELOCK_SAVER="${PATH_OF_COMPILED_BINARY}"
xsecurelock
```
 
## Settings

The module also have some options can be passed by environment variables.

| env name                     | default value | meaning                                                                                                                                                                                                |
| ---------------------------- | ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| CGOL_CELL_SIZE               | 5             | How many pixel per cell have. Smaller size allows more cells to be displayed on the screen.                                                                                                            |
| CGOL_INIT_ALIVE_PROBABILITY  | 0.2           | Initialize cells by random, this value will be the alive probability of cell.                                                                                                                          |
| CGOL_INIT_RLE_FILE           |               | Use [RLE](https://conwaylife.com/wiki/Run_Length_Encoded) file to initialize cells, this setting will override `CGOL_INIT_ALIVE_PROBABILITY`.                                                          |
| CGOL_MUTATION_ROUND_INTERVAL | 0             | How many rounds between mutations, if it is equals zero, the mutation will never be triggered. The mutation will randomly flip the cell's status, letting the game won't be stuck in a repeating loop. |
| CGOL_MUTATION_PROBABILITY    | 0.001         | The probability of cell mutation.                                                                                                                                                                      |
| CGOL_ROUND_SLEEP_TIME        | 0.1           | Sleep time between rounds.                                                                                                                                                                             |


## RLE

The default position of pattern is the left-top corner. You can use `base_x` and `base_y` to adjust the position of pattern.  
e.g. `base_x = 10, base_y = 20`, will move pattern 10 cell to the right and move pattern 20 cell to the bottom. 