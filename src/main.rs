//! The tictactoe binary crate.
//! It does nothing except call into the library.
extern crate tictactoelib;

/// The main entry point of the binary.
///
/// It simply calls the main() function in the associated tictactoelib library.
fn main() {
    tictactoelib::main();
}
