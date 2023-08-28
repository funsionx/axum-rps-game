# Rock-Paper-Scissors game

Minigame written in Rust using Axum web-framework

## Project structure
```
- src 
	- routes
		- html.rs
		- rps.rs
		- mod.rs
	- main.rs
...
index.html	
```

- *routes* -> folder with endpoints
  - *html.rs* -> get request on route "/", returns html page
  - *rps.rs* -> post request on route "/", returns randomly generated game item
- *main.rs* -> here app starts and router is defined
- *index.html* -> the only page with content, tailwind and daisyui installed via cdn

## How to install and run

Installing: 
```bash
git clone https://github.com/funsionx/axum-rps-game.git
cd axum-rps-game
```

Running with cargo (make sure you have cargo and rustup installed)
```bash
cargo run
```

OR running with docker-compose:
```bash
docker-compose up
```

## TODO

- May be make some kind of challenge
- Add some more games