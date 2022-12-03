use clap::Parser;
use args::Args;

mod helpers;
mod system;
mod args;

#[derive(Clone, Copy, Debug)]
pub enum ActionType {
    HostInfo,
    Delimiter,
    Details,
    Colors
}

#[derive(Debug)]
pub struct Action<'a> {
    action_type: ActionType,
    name: Option<&'a str>,
    func: Option<fn() -> String>,
}

const ACTIONS: [Action; 13] = [
    Action {
        action_type: ActionType::HostInfo,
        name: None,
        func: Some(system::host::get_hostname),
    },
    Action {
        action_type: ActionType::Delimiter,
        name: None,
        func: None,
    },
    #[cfg(target_os = "linux")]
    Action {
        action_type: ActionType::Details,
        name: Some("Distro"),
        func: Some(system::host::get_distro),
    },
    #[cfg(target_os = "windows")]
    Action {
        action_type: ActionType::Details,
        name: Some("Product"),
        func: Some(system::specs::get_kernel),
    },
    #[cfg(target_os = "linux")]
    Action {
        action_type: ActionType::Details,
        name: Some("Kernel"),
        func: Some(system::host::get_distro),
    },
    Action {
        action_type: ActionType::Details,
        name: Some("Arch"),
        func: Some(system::specs::get_arch),
    },
    Action {
        action_type: ActionType::Details,
        name: Some("Shell"),
        func: Some(system::host::get_shell),
    },
    Action {
        action_type: ActionType::Details,
        name: Some("Resolution"),
        func: Some(system::host::get_resolution),
    },
    Action {
        action_type: ActionType::Details,
        name: Some("IP"),
        func: Some(system::net::get_ipaddr),
    },
    Action {
        action_type: ActionType::Details,
        name: Some("CPU"),
        func: Some(system::specs::get_cpu),
    },
    #[cfg(target_os = "windows")]
    Action {
        action_type: ActionType::Details,
        name: Some("Disk usage"),
        func: Some(system::specs::get_disk_usage),
    },
    Action {
        action_type: ActionType::Details,
        name: Some("RAM"),
        func: Some(system::specs::get_ram_used),
    },
    #[cfg(target_os = "linux")]
    Action {
        action_type: ActionType::Details,
        name: Some("Init System"),
        func: Some(system::host::get_init_system),
    },
    #[cfg(target_os = "windows")]
    Action {
        action_type: ActionType::Details,
        name: Some("GPU"),
        func: Some(system::specs::get_gpu),
    },
    Action {
        action_type: ActionType::Delimiter,
        name: None,
        func: None,
    },
    Action {
        action_type: ActionType::Colors,
        name: None,
        func: None,
    }
];

fn main() {
    let args: Args = Args::parse();

    let line_count = helpers::file::get_file_linecount(
        &format!("{}{}.txt", helpers::paths::get_pony_path(), &args.pony)
    );

    let to_skip = ((line_count / 2) as f32).floor() - 6.0;

    for i in 0..line_count {
        helpers::print::print_ponyline(i, &args.pony, &args.color);

        let pad_i = (i as f32 - to_skip).floor();

        if ACTIONS.get(pad_i as usize).is_none() || pad_i < 0.0 {
            println!();
            continue;
        }

        match ACTIONS[pad_i as usize].action_type {
            ActionType::HostInfo => {
                helpers::print::print_detail(
                    &system::host::get_user(),
                    system::host::get_hostname(),
                    ActionType::HostInfo,
                    &args.color
                );
            },
                    
            ActionType::Delimiter => {
                helpers::print::print_detail(
                    "",
                    "".to_string(),
                    ActionType::Delimiter,
                    args.color.as_str()
                );
            },
                    
            ActionType::Details => {
                helpers::print::print_detail(
                    ACTIONS[pad_i as usize].name.unwrap(),
                    ACTIONS[pad_i as usize].func.unwrap()(),
                    ACTIONS[pad_i as usize].action_type,
                    args.color.as_str()
                );
            },

            ActionType::Colors => {
                helpers::print::print_detail(
                    "",
                    "".to_string(),
                    ActionType::Colors,
                    args.color.as_str()
                );
            }
        }

        println!();
    }
}