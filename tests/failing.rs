use ctor::dtor;



#[dtor]
fn on_shutdown() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            println!("HELLO WORLD");
        });
}
