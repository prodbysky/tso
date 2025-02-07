fn main() {
    match lalrpop::process_root() {
        Ok(()) => {}
        Err(e) => {
            println!("Failed to do lalrpop build step due to: {e:?}");
            panic!()
        }
    }
}
