***This project is still an in early stage. I've started to use it for my own games and will try not to break project files. But no guarantees yet!***

# Dungeon Planner

Dungeon Planner is a small and simple tool to plan and organize dungeons for tabletop games.
Dungeons are composed out of chambers and doors, for which notes can be added.
The application is game system agnostic and can be used for any system.
Dungeons can be exported as PDF.

## Tools

- **Add Chamber**: Creates a new chamber to the dungeon. The chamber will appear in the Chamber list
- **Selection**: Select chambers or doors by clicking on them in the canvas. Some tools only work with an active chamber or door.
- **Cut Tool**: Used to split a chamber wall in half, adding a new corner to the room
- **Draw Tool**: Used to draw a chamber. This tool will always continue from the last corner of the chamber.
- **Add Door**: Allows adding a door to the selected chamber. Door are always placed on walls.
- **Chamber List**: Lists all chambers. Allows selection of chambers.
- **Chamber Details**: Change the name and notes of a chamber
- **Door List**: Lists all doors. Allows selection of doors.
- **Door Details**: Change name and notes of a door. You can also define to which chamber a door leads.

## Roadmap / Feature List

- [x] Grid
- [x] Drawing chambers using straight edges
    - [x] Grid Snapping
- [ ] Placing object markers (stairs, chests)
- [x] Assign doors/properties to edges
    - [ ] hidden doors
- [x] GM Notes on chambers
- [ ] Prints
    - [x] Full map with numbers assigned to chambers
        - [x] GM Notes
    - [ ] Chambers seperated to cut out
        - [ ] Labels to show neigboring chambers (not on hidden chambers)
