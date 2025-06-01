# PORCWEB
PORCWEB is a website developed by the mod team of the rumble competition PORC.  
It is part of a larger system meant to help organize the PORC tournament, which is built on the [PORC Discord server](https://discord.gg/jaXkQs6tEm).  
If you want to run this project yourself, quite a bit of setup will be required, which is why you should probably contact us directly via the Discord server or by email.  
The following instructions are for hosting the website locally and assume you have access to the required secrets and environment files.

---

## Getting Rust
To get started, you need Rust to run this project. You can find a guide on how to install Rust [here](https://doc.rust-lang.org/book/ch01-00-getting-started.html).  
After you have successfully installed Rust, you will be able to compile the project by running:
```sh
cargo build
```
in the command line.

---

## Running the backend
After successfully installing Rust, you can build and run the project by typing:
```sh
cargo run
```
in the command line.  
Upon launching, the program will show you the current server config. You can change the config in the `config.json` file.  
**Please only run the project in dev mode unless you intend to deploy it.**

---

## Running the frontend
After starting the backend, you will also need to start the frontend using Vue’s built-in development mode with the following command:
```sh
npm run dev
```
You can refer to the `README.md` in the `PORC-Front` directory for more info on how to do this.
