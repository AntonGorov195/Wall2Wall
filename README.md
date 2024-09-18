# Wall 2 Wall
A port of a game I made a long time in [Unity](https://unity.com/). 
This time it is written in the [Rust](https://www.rust-lang.org/) programming language, with the help of the [Macroquad](https://docs.rs/macroquad/latest/macroquad/) crate.
While Macroquad handled the rendering and the event loop of the game, I had to implement my own collision system.
Said system ended up being very simple and suited exacly for this game.
This was a fun little project which taught me many important concepts like:

- Basic collision system.
- A different work flow from the one inforced in Unity.
- The basics of how to use wasm in Rust.
- How to upload a game on itch.io.

Overall, I'm proud of this creation although there is a lot of room for improvement such as:

- Make the game scale for different screen sizes and resolutions.
- Get rid of all third party dependencies. Besides Macroquad, all other dependancies should not be difficult to re-implement.
- Add more game mechanics to the game.
- Align the text correctly.
- Sound.

Maybe one day I will come back to add these thing but that is enough for now.
I have many other ideas for cool projects which I can't wait and do.