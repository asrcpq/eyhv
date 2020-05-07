# eyhv

## Run

* Download and compile(libsdl2-dev is required)

* Run eyhv --help to get more options

## Key features

* High speed and high fault tolerance(of player performance).
Player's performance is evaluated by final global difficulty at death.

* Minimalist design(game mechanism and design style) 

* Randomly generated and reproducible(by u64 seed) enemies, simple replay mechanism

* Time in game always synchronized: player will suffer, not benefit from low FPS

* Everything drawed, no resource file, replaceable GUI frontend

## Gameplay

* Arrow keys: Move, in a constant speed

* z: Fire, cause reduced moving speed

* LAlt: Pause, press again to resume

* LShift: World slowdown, will disable side cannons

* Space: Fast forward, only in replay mode
