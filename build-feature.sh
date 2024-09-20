mkdir feature/tmp
cargo run -p diplomat-java -- -e feature/src/lib.rs -l feature/diplomat-java-conf.toml feature/tmp/
mv feature/tmp/* feature/java/somelib/
