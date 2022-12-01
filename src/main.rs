mod helpers;
mod system;

pub enum ActionType {
    HostInfo,
    Delimiter,
    Details
}

pub struct Action<'a> {
    name: &'a str,
    func: fn() -> String,
}

const ACTIONS: [Action; 8] = [
    Action {
        name: "Distro",
        func: system::host::get_distro,
    },
    Action {
        name: "Kernel",
        func: system::specs::get_kernel,
    },
    Action {
        name: "Uptime",
        func: system::host::get_uptime,
    },
    Action {
        name: "Shell",
        func: system::host::get_shell,
    },
    Action {
        name: "Resolution",
        func: system::host::get_resolution,
    },
    Action {
        name: "IP",
        func: system::net::get_ipaddr,
    },
    Action {
        name: "CPU",
        func: system::specs::get_cpu,
    },
    Action {
        name: "RAM",
        func: system::specs::get_ram_used,
    }
];

fn main() {
    for i in 0..12 {
        helpers::print::print_ponyline(i, "rainbowdash");

        if i == 0 {
            helpers::print::print_detail(
                &system::host::get_user(), 
                system::host::get_hostname(),
                true
            );
        } else if ACTIONS.len() > (i as usize) && i != 0 {
            helpers::print::print_detail(
                ACTIONS[i as usize - 1].name, 
                (ACTIONS[i as usize - 1].func)(),
                false
            );
        }

        println!();
    }
}
