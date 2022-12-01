mod helpers;
mod system;

fn main() {
    println!("{}", system::specs::get_kernel());
}
