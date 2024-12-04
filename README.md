# Megaverse API Project

> Crossmint coding challenge: minting a new megaverse into existence!

Welcome to the Megaverse API Project! This project interacts with the Megaverse API to create and manage objects in a 2D space. You can build maps based on the goal map provided by the API, delete specific objects, and display the current goal map with visual representations of the objects.

---

## Features

- **Display the Goal Map** - View the goal map provided by the API with visual representations of objects (🪐, 🔵, ☄️, etc.).

- **Create Map from Goal** - Automatically generate the map in the Megaverse by creating the objects defined in the goal map.

- **Delete Specific Objects** - Remove specific objects (Polyanet, Soloon, or Cometh) by specifying their coordinates and type.

---

## Setup

### Prerequisites

- [Rust](https://rustup.rs/) installed on your system.
- An API key (candidate ID) for the Megaverse API.

### Environment Variables

Create a `.env` file in the root directory of the project and add the following:

```plaintext
CANDIDATE_ID=your_candidate_id_here
```

Replace `your_candidate_id_here` with the candidate ID provided for your API access.

---

## How to Run

1. Clone the Repository

```bash
git clone https://github.com/miquelcabot/crossmint-megaverse.git
cd crossmint-megaverse
````

2. Install Dependencies Ensure all necessary Rust dependencies are installed by running:

```bash
cargo build
```

3. Run the Program Start the application with:
```bash
cargo run
```

4. **Navigate the Menu**. Follow the interactive menu options to display the goal map, create the map, or delete specific objects.

---

## Menu Options

The interactive menu provides the following options:

```plaintext
  __  __                                                           
 |  \/  |   ___    __ _    __ _  __   __   ___   _ __   ___    ___ 
 | |\/| |  / _ \  / _` |  / _` | \ \ / /  / _ \ | '__| / __|  / _ \
 | |  | | |  __/ | (_| | | (_| |  \ V /  |  __/ | |    \__ \ |  __/
 |_|  |_|  \___|  \__, |  \__,_|   \_/    \___| |_|    |___/  \___|
                  |___/                                            

? Select an option: ›
❯ Show goal map
  Create map based on goal map
  Delete object at specific position
  Exit
```

---

## Code Structure

- `object_type.rs`: Contains enums and logic for representing objects (`Polyanet`, `Soloon`, `Cometh`) and their attributes (`Color`, `Direction`).

- `lib.rs`: Contains the main API client logic, including fetching the goal map, creating and deleting objects, and parsing and mapping goal map data to object types.

- `main.rs`: Entry point for the application, including the interactive menu and integration with the API client.

---

## Icons and Representations

Table of icons and their corresponding object types:

| Object Type | Icon              | Details                                            |
|-------------|-------------------|----------------------------------------------------|
| Space       | 🌌                | Represents an empty cell                           |
| Polyanet    | 🪐                | A generic Polyanet object                          |
| Soloon      | 🔵 / 🔴 / 🟣 / ⚪  | Represents Soloon objects with specific colors     |
| Cometh      | ☄️↑ / ☄️↓ / ☄️← / ☄️→ | Represents Cometh objects with specific directions |

---

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

---

## Author

👤 **Miquel Cabot** ([@miquelcabot](https://github.com/miquelcabot))

Enjoy exploring the Megaverse! 🚀
