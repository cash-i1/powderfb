# bugs
- [x] fix water crashing game when going off the left side
- [x] fix water physics being shit
    - water goes left
    - water also doesn't settle flat and looks lumpy
- [ ] water jitters around
- [x] make water and sand simulation code less messy
- [ ] particles phasing through things
- [x] make sand physics less liquid-like
- [ ] fix placing particles on left and right of world makes them not have physics
- [x] clean up main function
- [ ] clicking on a particle button spawns a particle apart from the last button

# features
- [x] add acid
- [ ] make drawing particles better (no gaps in stroke)
- [ ] add deselecting particles
- [ ] improve ui
    - make particle icons more intuative
- [ ] add resizing
- [ ] water rusts metal
- [ ] more dense particles sink into less dense particles
- [ ] add 'asleep' to particles so they stop simulating if no one interacts with them

# improvements
- [ ] text
- [x] add functions for world struct for manipulate particles
    - set function that returns a result whether it could be placed or not 
- [x] make the ui system good
    - 'element' trait or something so that there can be buttons and checkboxes etc with differnt look and functionality
- [ ] remake graphics
    - as its own crate
    - with text rendering
    - with image rendering
    - with 'render layers' which are just like photoshop layers
    - [ ] use pixels instead of minifb
- [ ] remake particle system
- [x] add color struct
    - [ ] with color manipulation functions
- [x] added position struct
