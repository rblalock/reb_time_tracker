mod cmd;
mod core;

fn main() {
    // One day, this might be run outside of just CLI
    // context...like as a library...so assuming that
    // this would be the right place to separate them even 
    // though I don't know how libraries work yet.
    cmd::main::run();
}
