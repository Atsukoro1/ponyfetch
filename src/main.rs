mod helpers;
mod system;

fn main() {
    helpers::print::print_detail("User", system::host::get_user());
    helpers::print::print_detail("Hostname", system::host::get_hostname());
    helpers::print::print_detail("IP", system::net::get_ipaddr());
    helpers::print::print_detail("Kernel", system::specs::get_kernel());
    helpers::print::print_detail("RAM", system::specs::get_ram_used());
    helpers::print::print_detail("CPU", system::specs::get_cpu());
    helpers::print::print_detail("Distro", system::host::get_distro());
    helpers::print::print_detail("Shell", system::host::get_shell());
}
