## Systems Overview

Lambda is built as four core systems.

1. Lambda ES
2. Lambda DS
3. Lambda Web
4. Lambda STD

The interaction, purpose, and ideas behind each core system will be described in this article.

### Lambda ES

Lambda ES is the core of the lambda programming language. It encompasses all the functionality that defines Lambda. Notably absent is any IO functionality. However, all the core data types and functions are available, as well as its embeddable API.

ES stands for Embedded System and that's what Lambda ES is - a fully and easily embeddable runtime so anyone can easily build scripting into their application.

Lambda ES has the goal of providing the cleanest and most easily embeddable API for any scripting language around while maintaining high enough performance characteristics to make it a good choice for gaming and resource sensitive systems.

### Lambda DS

DS stands for distributed system. Included in DS are a lot of tools for dealing with asynchronous behavior and building distributed/concurrent applications. Also there are a few more built-in functions mainly for doing things that are assumed to exists in an operating system level runtime and for doing numerical computational processing.

### Lambda Web

Lambda Web is a webassembly compiler backend with FFI for all the standard web api's so you can write Lambda apps that easily run on the web.

### Lambda STD

As much functionality as possible is stored here. This is really just a special namespace called `std` in the public packages repository, and this is where vital libraries for building any application are stored. Some of the libraries in std are:

```scheme
std.io      ; input/output
std.crypto  ; cryptography
std.math    ; mathematics
std.net     ; networking
std.os      ; operating systems
std.spec    ; data specifications
```
