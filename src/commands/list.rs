// This is the `ls` module. It is responsible for exposing a public interface
// making it easy for the CLI to execute the ls command. This generally
// means there is one public function. In this case the `ls()` function. All
// other functions in here are purely private supporting functions and should
// be strongly considered if they fit better in one of the other modules
// inside the ps module and should be exposed via the library public interface.

use super::utils::print_error_chain;
use gps as ps;

pub fn list(color: bool) {
    match ps::list(color) {
        Ok(_) => {}
        Err(e) => {
            print_error_chain(color, e.into());
            std::process::exit(1);
        }
    };
}
