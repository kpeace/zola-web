## Running in Dev Mode
This project uses [Yew](https://yew.rs).

To build the project we need to add the wasm target for the rust compiler

`rustup target add wasm32-unknown-unknown`

Trunk is a tool to package wasm. There are other tools avilble (see the Yew documentation).

`cargo install --locked trunk`

To run the project localy first create a Trunk.toml file containing
```
[build]
dist = "dist"

# Copy config.json to the dist folder (works with both `trunk serve` and `trunk build`)
[[hooks]]
stage = "build"
command = "cp"
command_arguments = ["path/to/your/zola_blog>/config.json", "dist/config.json"]
```

The config file should be created by the [Zola app](https://github.com/kpeace/zola) and edited to fit your needs

`trunk serve --open`


## Running tests
`cargo install wasm-pack`
`wasm-pack test --headless --firefox`

## TODO
The real list is endless. This is just a "short" list of things that I don't want to forget

* Enable markdown in post summary
* Support npstr long form event summary in post list
* Enable/Disable Next/Prev button depending on if there are any posts to show
* Handle new posts better (notification and/or auto reload first window)
* support blog sections (main, about etc.)

## License
Currently distributed under MIT license. This license was chosen because https://choosealicense.com/ recommended it as the most permisive license. If there is a need for an aditional license, please contact me.