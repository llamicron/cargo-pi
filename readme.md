# cargo-pi
I like to run my code on a Raspberry Pi, but they're rather slow. Compile times with Rust are already bad, and the RPi makes it unbearable in some cases. This cargo tool will compile your binaries on your development machine, then send them to your Raspberry Pi to be run.

In most cases, you can probably develop on any machine then compile once for RPi. In other cases, you need to compile and test frequently on the RPi. I use the RPi for process control on a brewing rig. The code I write to run on the RPi depends on hardware connected to the RPi. Using another development machine without this hardware makes the job very difficult.


# Installation and Usage
Be sure you have cargo installed, then install this tool with

```
$ cargo install cargo-pi
```

and run with

```
$ cargo pi
```
