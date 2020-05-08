# eyhv

![demo](https://asrcpq.github.io/resources/2005/eyhv_demo.gif)

## Run

* Download and compile(libsdl2-dev is required)

* Run eyhv --help to get more options

* Caution: a .eyhv\_replay file will be created under working directory

## Key features

* High speed and high fault tolerance(of player performance).
Player's performance is evaluated by final global difficulty at death.

* Minimalist design(game mechanism and design style):
No bombs, drop items, etc.

* Randomly generated and reproducible(by seed number) stages.
Game replay is automatically saved.

* Time in game always synchronized: player will suffer, not benefit from low FPS

* Everything drawed, no resource file, replaceable GUI frontend

## Gameplay

* Arrow keys: Move, in a constant speed

* z: Fire, cause reduced moving speed

* LAlt: Pause, press again to resume

* LShift: World slowdown, in a limited time and will disable side cannons

* Space: Fast forward, only in replay mode
