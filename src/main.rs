mod helpers;
mod system;

fn main() {
    for i in 0..31 {
        helpers::print::print_ponyline(i, "celestia");
        helpers::print::print_detail("User", system::host::get_user());
    }
}
