# Ratventure - Rust version
Ratventure is a CLI game I had to code in one of my school assignments.
The assignment was for a module that taught programming in Python, and hence it was also written in Python.
I decided to create a Rust version to help me learn the language better.

## Gameplay
The Hero `'H'`, starts on the top left corner of the map.
The goal of the game is to reach the bottom right corner of the map, where the Rat King `'K'` is waiting.
In order to fight the Rat King, the Orb of Power has to be found first.
It is hidden in open spaces `' '` and gives a stats boost to the Hero when found.

Along the way, there will be Towns `'T'` and open spaces `' '`. In open spaces, enemies may spawn and the player has to fight them. Towns don't have enemies and players can choose to rest here to regain health points.

## Scoring
The high score is determined by the number of days taken to defeat the Rat King.
The least number of days taken results in the best score.
Moving and resting each add to the day count.
