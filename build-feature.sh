mkdir feature/tmp
cargo run -p diplomat-java -- -e feature/src/lib.rs -l feature/diplomat-java-conf.toml feature/tmp/
rm -r feature/java/somelib/src/main
mv feature/tmp/src/main feature/java/somelib/src/main
rm -r feature/tmp
