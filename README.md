# Lightspeed Sensor Demo
Example implementation of lightspeed limited sensors in space video games

This code is ugly.

Every detectable entity places a "ghost" each turn which contains snapshot of it's location.
Every entity with a sensor then loops over all ghosts and checks if it's distance in time is equal to it's distance in lightturns.
If it is, that means it's detected and information is (optionally) added to a vector of detection information.

Player unit (only one with sensor) is marked in blue. Other units are marked in red.\
Detections (taken from player's detection information vector) from present turn are displayed in green.\
Red diagonally moving units (one of them faster than light) are displayed for presentation of effects of lightspeed limit on sensor
(detections represent object's positions more in the past with increasing distance).\
Units on the right (100 added every turn) are there for benchmarking purposes.
In release mode performance isn't as bad as I expected (because Rust is `blazingly fast`).

You can move the map with WASD keys.

Because rendering isn't done directly after detection claculations (it's done after all turn calculations, including movement)
it looks like even at distance below one lightturn ghosts are behind their entities. I'm too lazy to change that.

Doing this with continuous space coordinates instead of integers and using smaller timestep may improve things,
but I did it on a grid for simplicity.
