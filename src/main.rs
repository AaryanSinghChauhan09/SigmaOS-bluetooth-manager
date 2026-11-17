//! SigmaOS-mint-bluetooth-manager — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_mint_bluetooth_manager_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-mint-bluetooth-manager: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-mint-bluetooth-manager: ok");
}
