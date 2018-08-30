# rq3vm

Ever wanted to switch from C to Rust only to write your app again in C? Then this Project is for you!

I think this project only exists because I'm so facinated with the Quake 3 Engine Architecture. And I recently saw that someone ([jnz](https://github.com/jnz)) seperated the Quake 3 Arena Virtual Machine from the rest and made it avaialiable for use without the game.

At first I wanted to reimplement the whole VM in Rust but after I while I realized if I continue on this path my idea backlog will never go down.

So I used this as an opportunity to learn about mixing C Code with Rust and Bindgen.

## So what is working and what not?

This is only a proof of concept at the moment until I have something where I can use it. Working with the Q3VM is super easy if you know old school C.

At the moment I implemented only the following in the safe bindings:

    * Loading and starting VM Files from a Binary Blob.
    * Rust-like RAII like you know it from your other code. No manual freeing of resources.
    * Safe Interface for the Syscall Callback. That includes argument parsing and function resolution. That's one part of the project that really still needs more love because I left out some sanity checks that you wouldn't need for a demo but for production
    * Sample implementation of some Q3VM --> Rust calls. Couldn't figure out yet how to pass values from Rust to Q3VM. Maybe I should start reading the manual.

## How to build the project?

Since modifiy the compiler wasn't part of this experiment you can find the build tools on the [q3vm](https://github.com/jnz/q3vm) github page. If your binaries refuse to build any code you might have more luck building everything with clang at the moment.

After you build the compiler and the infrastructure put your freshly build tools somewhere in your PATH, goto example and run "make". You should end up with a File named "rustic.qvm". After that it's just a simple cargo run from the root directory.

## How do I use this in my Project

Sorry this was a one day Project. So the best documentation is the src folder of rq3vm. I have build a little sample program that shouldn't be too hard to understand.

## Still reading?

Have fun! And tell me if you improved it or build something with it even tho it's totally half baked at the moment. Working on this for a day was worth it for sure so I think you wouldn't waste your time here. Unfortunally I don't have a need for a scripting language right now but if I would have this would be my choice.

Special Thanks to jnz without him I would have still wondered why GCC is building a broken LCC compiler. And of course Id Software for coming up with the idea in the first place and open source it.
