# Dungeon Planner

## Feature List

- [x] Grid
- [x] Drawing rooms using straight edges
    - [x] Grid Snapping
- [ ] Placing object markers (stairs, chests)
- [ ] Assign doors/properties to edges
    - [ ] hidden doors
- [ ] GM Notes on rooms
- [ ] Prints
    - [ ] Full map with numbers assigned to rooms
        - [ ] GM Notes
    - [ ] Rooms seperated to cut out
        - [ ] Labels to show neigboring rooms (not on hidden rooms)

## System / UI

```
|--------|-----------------------|
|  Tools |                       |
|--------|                       |
|        |          Canvas       |
| Context|                       |
|  Menu  |                       |
|        |                       |
|--------|-----------------------|
```

- View
    - Offset XY to enable a infinite dungeons
    - Grid config (size, color, style)

- Room
    - vertex list
    - notes
    - placed objects

Object
    - notes
