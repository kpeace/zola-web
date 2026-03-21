## Running in Dev Mode
This project uses [Yew](https://yew.rs).

To build the project we need to add the wasm target for the rust compiler

`rustup target add wasm32-unknown-unknown`

Trunk is a tool to package wasm. There are other tools avilble (see the Yew documentation).

`cargo install --locked trunk`

To run the project localy

`trunk serve --open`


## Running tests

`wasm-pack test --headless --firefox`

## License
Currently distributed under MIT license. This license was chosen because https://choosealicense.com/ recommended it as the most permisive license. If there is a need for an aditional license, please contact me.