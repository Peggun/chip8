# Chip8
A simple Chip-8 Emulation tool built on Rust fit with multiple ROM's (thanks to [https://github.com/dmatlack/chip8/](https://github.com/dmatlack/chip8/))<br>
If you would like to play the game but are unsure how, please go to this repository, and read the .txt files for the game that you would like to play, these have the binds and more.
<br>
<br>
Here are the keybinds in correspondence to the original Chip-8 Keypad:<br>
Key 1 - Equivalent to 1 on a modern keyboard<br>
Key 2 - Equivalent to 2 on a modern keyboard<br>
Key 3 - Equivalent to 3 on a modern keyboard<br>
Key 4 - Equivalent to Q on a modern keyboard<br>
Key 5 - Equivalent to W on a modern keyboard<br>
Key 6 - Equivalent to E on a modern keyboard<br>
Key 7 - Equivalent to A on a modern keyboard<br>
Key 8 - Equivalent to S on a modern keyboard<br>
Key 9 - Equivalent to D on a modern keyboard<br>
Key 0 - Equivalent to X on a modern keyboard<br>
Key A - Equivalent to Z on a modern keyboard<br>
Key B - Equivalent to C on a modern keyboard<br>
Key C - Equivalent to 4 on a modern keyboard<br>
Key D - Equivalent to R on a modern keyboard<br>
Key E - Equivalent to F on a modern keyboard<br>
Key F - Equivalent to V on a modern keyboard<br>

This Chip-8 Emulation runs on 250Hz meaning that it runs 250 operations per second.

Make sure that you have the latest version of [Rustup/Rust](https://www.rust-lang.org/tools/install) installed on your computer.

## SDL2 Installation (Windows Only):
For this emulation to work we will need to install SDL2 rust bindings in the places where it should be found.

First off we need to install SDL2 onto our machine, download the files [here](https://github.com/libsdl-org/SDL/releases/). Make sure that we are downloading the SDL2-devel-{latest version}-VC.zip
Next we need to extract those files. Go on and extract them to whatever directory you like, this wont matter later when we move the extracted files. 
Next navigate to the newly extracted folder, and find the lib folder and your current computer's architecture.

Now copy all of the files in that folder.
Navigate to this folder:
C:\Users\{}\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib"
and paste all of the files there.

## Chip8 Setup
Now we need to setup the Emulation
Clone the repository using:
```sh
git clone https://github.com/Peggun/chip8.git
```

Then go into this new folder and create a new .env file. This is where you will put your Windows PC username for the computer to find the SDL2.dll file
Add your username using this syntax:
```sh
USERNAME="username"
```

And now we are all done!
Just run the emulator using: Just note the cycle delay doesnt do anything.
```sh
cargo run [display scale] [cycle delay] [path to ROM]
```
