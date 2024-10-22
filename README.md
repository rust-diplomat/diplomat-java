# diplomat-java
Panama-based Java backend for Diplomat. This backend depends on the [`jextract`](https://github.com/openjdk/jextract) command-line tool
to generate [JEP 454](https://openjdk.org/jeps/454) access and calling code for the generated native library. Because of this dependency
this backend won't be packaged with the [diplomat tool](https://rust-diplomat.github.io/book/).


# JEP 454 binding generation
In order to package this backend with the other backends there is a long term plan to implement a subset of `jextract`'s functionality.
The reasons for this include:
- reducing the amount of generated code: By default `jextract` produces wrappers for standard library code as well as other methods
that should not be unnecessary for a Diplomat wrapped library.
- simplifying development setup: most Diplomat developers are not familiar with Java and setting up a Java development environment
causes unnecessary friction. Diplomat strives to enable contributors to only have to set up an environment for the language that they 
need a backend for. But all Diplomat contributors need to generate the code for all backends.
- simplifying ci setup: `jextract` does not appear to be available via package manager, and so needs to be downloaded via URL. This is
an undesired step for Diplomat. 

This [JEP 454](https://openjdk.org/jeps/454) binding generation will be developed as a library [`jextrsact`](./jextrsact/README.md) in 
this repository. It will not be a replacement for jextract as it will focus only on 

# Running
You need to specify a toml config file with a domain a library name
```toml
domain = "dev.diplomattest"
lib_name = "somelib"
```
then pass the location of the library with `-e`, with a config path specified with `-l` and then an output directory

```sh
diplomat-java -e feature/src/lib.rs -l feature/diplomat-java-conf.toml feature/tmp/
```
