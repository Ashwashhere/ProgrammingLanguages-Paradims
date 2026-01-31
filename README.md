Multi-Paradigmatic Battleships 

This project implements a custom version of **Battleships** across three distinct programming languages to demonstrate the strengths of different programming paradigms. Each solution follows the same core logic but adapts its structure to fit the specific philosophy of the chosen language.

---

## 🎮 Game Features

* **Custom Board Sizes:** Users can initialize grids ranging from  up to .


* **Standard Fleet:** Includes Aircraft Carrier (5), Battleship (4), Cruiser (3), Submarine (3), and Destroyer (2).


* **Special Missile:** A collectable token that enables a one-time  area attack.


* **Seed-Based Logic:** Random ship and item placement seeded by Student ID for consistent testing.



---

## 🛠 Paradigms & Tech Stack

1. Functional Paradigm (**Rust**) 

* **Immutability:** Uses `let` bindings to ensure state is not changed inadvertently.


* **Declarative Logic:** Employs iterators like `.all()` for validation and pattern matching for turn processing.


* **Error Safety:** Utilizes the `Option` type to handle invalid user inputs safely.



2. Logic Paradigm (**Prolog**) 

* **Declarative Rules:** Game state is expressed through logical relationships and constraints rather than explicit control flow.


* **Dynamic Predicates:** Tracks ship positions and hits through predicates like `ship_positions/1`.


* **Backtracking:** Automatically evaluates shot outcomes using built-in search mechanisms.



3. Object-Oriented Paradigm (**Python**) 

* **Encapsulation:** Logic is split into `Board` (grid state) and `Game` (turn/win management) classes.


* **Abstraction:** Complex behaviors like ship placement are abstracted into reusable class methods.


* **Modularity:** High separation of concerns between game data and execution flow.



---

## 📂 Project Structure

The repository is organized following the standard assignment brief requirements:

* `Rust/` - Functional implementation source code.
* `Prolog/` - Logic-based source code.
* `Python/` - Object-Oriented source code.
* 
`Documentation/` - Full design pseudo-code, UML diagrams, and paradigm comparisons.

**Student ID:** 21362241 

Would you like me to generate a specific **Installation** or **How to Run** section based on the setup research found in the documentation?
