# Rust LRU Cache Simulator
## Background

The last assignment I wrote for a systems programming class was a C implementation of an LRU cache simulator, taking in a text valgrind memory trace file
and the parameters of associativity, block bits, and set bits for a given cache. Since I'm already pretty comfortable in C like languages like C++
and C#/Java, I thought it might be time to expand my horizons by learning a hip new language like Rust.

Rust has a reputation for being quite a difficult and frustrating to learn language given its stricter rules around memory and variable ownership/mutability.
But the benefit of having strict rules around these has become apparent to me as I've been dipping my feet more and more into concurrency programming.
I feel it would be valuable to be able to think and program in a system's level language built from the groundup with concurrency in mind!

## The Project

As mentioned before, the final product of this project should be a program that mimics the output of the cache simulator I made for the class but written in Rust.
I will limit myself to writing everything from scratch, and will use the trace files that were provided in the course. Since I wrote the C program for a class,
and since that class is still being taught at the university, I cannot release the source code for the original. That being said, any cs student who finds this repo 
in a last ditch Google search be warned: <b>Rust is not C</b>. The general approach will be the same, but their are much better resources out there for writing
C LRU cache simulators out there, and I'm sure your class has resources available as well. Feel free to read my, but don't try and use it for a class.

