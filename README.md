# xsecurelock-saver-game-of-life

A screensaver module that can run game of life for [xsecurelock](https://github.com/google/xsecurelock).

# Usage

clone this repository and run `cargo build --release` to get compiled bianry.  
add `export XSECURELOCK_SAVER="${PATH_OF_COMPILED_BINARY}"` to your script.  

```
#!/bin/sh
# omit other settings ... 

export XSECURELOCK_SAVER="${PATH_OF_COMPILED_BINARY}"
xsecurelock
```
 
# Settings

The module also have some options can be passed by environment variables.

| env name | default value | meaning |
| - | - | - |
| CGOL_CELL_SIZE | 5 | How many pixel per cell have. Smaller size allows more cells to be displayed on the screen. |
| CGOL_INITIALIZE_ALIVE_PROBABILITY | 0.2 | The probability of initial cell alive status. |
| CGOL_MUTATION_ROUND_INTERVAL | 10 | How many rounds between mutations, if it is negative, the mutation will never be triggered. The mutation will randomly flip the cell's status, letting the game won't be stuck in a repeating loop. |
| CGOL_MUTATION_PROBABILITY| 0.001 | The probability of cell mutation. |
| CGOL_ROUND_SLEEP_TIME | 0.1 | Sleep time between rounds. |
