// In order to be compatible with both desktop, wasm, and android, the example is both a binary and a library.
// Just forward to the library in main

#[tokio::main]
async fn main() {
    wojo_lib::main();
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn name() {
        todo!();
    }
}
