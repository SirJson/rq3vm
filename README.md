# rq3vm

### Be sure to know that this crate is at the moment in a transition phase from "prototype" to a publish-ready crate which means some high level redesigning, writing better examples and some project reorganizing.

### _What you will find on the master branch right now might not even build._

### The moment I'm done with everything you can download this smoothly from crates.io but until then you might have to get your hands dirty to get something running.

***

Ever wanted to switch from C to Rust only to write your app again in C? Then this Project is for you!

I think this project only exists because I'm so fascinated with the Quake 3 Engine Architecture. And I recently saw that someone on Reddit called ([jnz](https://github.com/jnz)) that separated the Quake 3 Arena Virtual Machine from the rest of the game and made it available for general purpose use. The original thread can be found [here](https://www.reddit.com/r/programming/comments/9b839q/embedding_the_quake_3_virtual_machine_in_your_own/)


At first I wanted to re-implement the whole VM in Rust but after I while I realized if I continue on this path my idea backlog will never go down.

So I used this as an opportunity to learn about mixing and linking C Code with Rust and bindgen.

## So what is working and what not?

This is only a proof of concept at the moment until I have something where I can use it. Working with the Q3VM is super easy if you know old school C.

At the moment I implemented only the following in the safe bindings:

* Loading and starting VM Files from a Binary Blob.
* Rust-like RAII like you know it from your other code. No manual freeing of resources or unsafe code you have to handle yourself.
* Safe Interface for the Syscall Callback. Might need more love to be production ready.
* Example implementation of some Q3VM --> Rust calls and back.

Also note that thanks to the `cc` crate, `bindgen` and the simplicity of the VM it should be pretty portable. There is nothing JITed and I build the Q3VM directly from cargo into a static library that gets linked with your rust code. No DLLs or dependencies for the end user needed.

## How to build the project?

Since modifying the compiler wasn't part of this experiment you can find the build tools on the [q3vm](https://github.com/jnz/q3vm) GitHub page. If your binaries refuse to build any code you might have more luck building everything with clang at the moment.

After you build the compiler and the infrastructure put your freshly build tools somewhere in your PATH. The qvm C code will be automatically linked to your code. I also added automatic QVM rebuilds for this project. All you have to do execute is cargo run. That is if you are on a Unix like machine. I didn't test this at all on Windows.

## How do I use this in my Project

At the moment Library and example are mixed. libq3vm-sys contains just the raw and unsafe bindings to the Q3VM. It should be quite complete but who want's to use unsafe in every function?
In order to to use the library right now just copy `vm.rs` and implement the `SyscallHandler` callback. The heavy lifiting is done by the VM module so all you have left to do is create a Q3VM object and pass it a Vec<u8> of a QVM File. `std::fs::File` should make that easy.

Remember you have to provide your own standard library in the end. You can get inspired by mine (system.rs) but in the in end you have to implement the Syscalls you need for your Application. For more information have a look at the [q3vm](https://github.com/jnz/q3vm) GitHub page. Also make sure to read the excelent Quake 3 Source Code Review from Fabien Sanglard. [He did a chapter about the VM](http://fabiensanglard.net/quake3/qvm.php) and it helped me a lot when I was working on my example.

## Still reading?

Have fun! And tell me if you improved it or build something with it. Working on this for a day was worth it for sure so I think you wouldn't waste your time here. Unfortunately I don't have a need for a scripting language right now but if I would have this would be my choice.

Special thanks to **jnz** without him I would have still wondered why GCC is building a broken LCC compiler. Of course **id Software** for coming up with the idea in the first place and open source it and the **ioquake3 Team** to cleanup that mess that id Software left us.