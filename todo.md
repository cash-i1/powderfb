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
- [ ] add functions for world struct for manipulate particles
- [ ] make the ui system good
    - 'element' trait or something so that there can be buttons and checkboxes etc with differnt look and functionality
- [ ] remake graphics
    - as its own crate
    - with text rendering
    - with image rendering
- [ ] remake particle system
- [x] add color struct
    - [ ] with color manipulation functions
