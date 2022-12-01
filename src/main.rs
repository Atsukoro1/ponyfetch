use clap::Parser;
use args::Args;

mod helpers;
mod system;
mod args;

#[derive(Clone, Copy)]
pub enum ActionType {
    HostInfo,
    Delimiter,
    Details
}

pub struct Action<'a> {
    action_type: ActionType,
    name: Option<&'a str>,
    func: Option<fn() -> String>,
}

const ACTIONS: [Action; 10] = [
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
    Action {
        action_type: ActionType::Details,
        name: Some("Distro"),
        func: Some(system::host::get_distro),
    },
    Action {
        action_type: ActionType::Details,
        name: Some("Kernel"),
        func: Some(system::specs::get_kernel),
    },
    Action {
        action_type: ActionType::Details,
        name: Some("Uptime"),
        func: Some(system::host::get_uptime),
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
    Action {
        action_type: ActionType::Details,
        name: Some("RAM"),
        func: Some(system::specs::get_ram_used),
    }
];

fn main() {
    let args: Args = Args::parse();

    for i in 0..12 {
        helpers::print::print_ponyline(i, &args.pony, &args.color);

        if ACTIONS.get(i as usize).is_none() {
            println!();
            continue;
        }

        match ACTIONS[i as usize].action_type {
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
                    ACTIONS[i as usize].name.unwrap(),
                    ACTIONS[i as usize].func.unwrap()(),
                    ACTIONS[i as usize].action_type,
                    args.color.as_str()
                );
            }
        }

        println!();
    }
}
