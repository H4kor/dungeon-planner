[![Run Tests](https://github.com/H4kor/dungeon-planner/actions/workflows/test.yml/badge.svg)](https://github.com/H4kor/dungeon-planner/actions/workflows/test.yml)

# Dungeon Planner

Dungeon Planner is a small and simple tool to plan and organize dungeons for tabletop games.
Dungeons are composed out of chambers and doors, for which notes can be added.
The application is game system agnostic and can be used for any system.
Dungeons can be exported as PDF.

The [User Documentation](https://h4kor.github.io/dungeon-planner/quickstart/) can be found on the [project website](https://h4kor.github.io/dungeon-planner/).

<p align="center">
  <img width="512" height="512" src="assets/DungeonPlanner.svg">
</p>

## Tools

- **Add Chamber**: Creates a new chamber to the dungeon. The chamber will appear in the Chamber list.
- **Selection**: Select chambers or doors by clicking on them in the canvas. Some tools only work with an active chamber or door.
- **Cut Tool**: Used to split a chamber wall in half, adding a new corner to the chamber.
- **Draw Tool**: Used to draw a chamber. This tool will always continue from the last corner of the chamber.
- **Add Door**: Allows adding a door to the selected chamber. Door are always placed on walls.
- **Chamber List**: Lists all chambers. Allows selection of chambers.
- **Chamber Details**: Change the name and notes of a chamber
- **Door List**: Lists all doors. Allows selection of doors.
- **Door Details**: Change name and notes of a door. You can also define to which chamber a door leads.

## Roadmap / Feature List to Version 0.1.0

- [x] Grid
- [x] Drawing chambers using straight edges
- [x] Grid Snapping
- [x] Assign doors/properties to edges
    - [x] hidden doors and chambers
- [x] GM Notes on chambers
- [x] Prints
    - [x] Full map with numbers assigned to chambers
        - [x] GM Notes
    - [x] Chambers seperated to cut out 
    - [x] Player Map
- [x] Placing object markers (stairs, chests)


## Developmnet

### Setup

[Install GTK dependencies](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html). This project requires at least GTK 4.6 (standard for Ubuntu 22.04).


### Tests

```
cargo test
```

### Run Development Version

```
cargo run
```

### Build Release Version

```
cargo build --release
```



