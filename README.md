# Lighthouse

A project with firmware for the [Eierland lighthouse PCB](https://github.com/pimdegroot/eierland) made by [Pim de Groot](https://github.com/pimdegroot/), written in [rustlang](https://rust-lang.org).

## Run this project

Open two terminal windows. On the first terminal, run:

`openocd -f openocd.cfg`

In the second terminal run

`cargo run`

If everything goes well, you should be dropped in a gdb session with a breakpoint on `main`. Type `continue` to start the actual code.
