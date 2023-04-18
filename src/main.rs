mod logos;
mod get;

use libmacchina::{
    GeneralReadout,
    MemoryReadout,
    KernelReadout,
    PackageReadout,
};
use libmacchina::traits::PackageManager;

use libmacchina::traits::GeneralReadout as _;
use libmacchina::traits::MemoryReadout as _;
use libmacchina::traits::KernelReadout as _;
use libmacchina::traits::PackageReadout as _;
use libmacchina::traits::ReadoutError;

use math;

use colorful::Colorful;
//use colorful::Color;

struct Pairs ( pub Vec<Pair> );

impl Pairs {
    fn new(pairs: Vec<Pair>) -> Self {
        return Self (
                pairs
            )
    }
}

struct Pair {
    key: String,
    value: Option<String>,
}

//let longest_key_lenght = pairs.0.iter().map(|x| x.key.len()).max().unwrap();

impl Pair {
    fn new(key: &str, value: Option<String>) -> Self {
        return Self {
            key: key.to_string(),
            value, // same as value: value
        }
    }

    fn print(&self, longest_key_lenght: usize, art: &mut std::slice::Iter<'_, &str>) {
        self.value.clone().and_then(|v| {
            art.next().and_then(|a| {
                println!("{}     {}: {}{}", a, self.key.clone().cyan().bold(), " ".repeat(longest_key_lenght - self.key.len()), v.remove_newline());

                return None::<()>;
            });
            return None::<()>;
        });
    }
}

fn old_print_next<'a, I: Iterator>(iter: &mut I) -> Option<<I as Iterator>::Item> where <I as Iterator>::Item: std::fmt::Display {
    let next = iter.next();
    print!("{}     ", match next {
        Some(ref expr) => expr,
        None => {
            return None;
        },
    });
    return next;
}

fn format_packages(packages: Option<Vec<(PackageManager, usize)>>) -> Option<String> {
    match packages {
        Some(packages) => {
            let packages_iter = packages.iter();
            let mut string = String::new();

            for next in packages_iter {
                string += &(next.1.to_string() + " (");
                string += &(next.0.to_string() + "), ");
            }

            string.pop();
            string.pop();

            return Some(string);
        }
        None => None
    }
}
fn format_time(uptime: Option<usize>) -> Option<String> {
    match uptime {
        Some(uptime) => {
            let uptime_hours_seconds = math::round::floor((uptime as f64 / 60.0) / 60.0, 0) * 60.0 * 60.0;
            return Some(
                format!("{:.0} Hours, {:.0} Minutes",
                        (uptime_hours_seconds / 60.0) / 60.0,
                        math::round::floor((uptime as f64 - uptime_hours_seconds) / 60.0, 0)
                       )
                )
        }
        None => None
    }
}

trait Format {
    fn remove_newline(&self) -> Self;
}
impl Format for String {
    fn remove_newline(&self) -> Self {
        let mut string = self.clone();
        if string.ends_with('\n') {
            string.pop();
            if string.ends_with('\r') {
                string.pop();
            }
        }
        return string;
    }
}
fn format_gpu(gpu: Option<Vec<String>>) -> Option<String> {
    match gpu {
        Some(gpu) => {
            let gpu_iter = gpu.iter();
            let mut string = String::new();

            for next in gpu_iter {
                string += &(next.to_string() + ", ");
            }

            string.pop();
            string.pop();

            return Some(string);
        }
        None => None
    }
}
fn format_memory(memory: Option<(u64, u64)>) -> Option<String> {
    match memory {
        Some(memory) => {
            return Some(format!("{:.2}GiB / {:.2}GiB",
                    (memory.0   as f32 / 1024.0) / 1024.0,
                    (memory.1   as f32 / 1024.0) / 1024.0
                    ));
        }
        None => None
    }
}

fn format_color(len_a: usize, len_b: usize) -> (usize, usize) {
    if len_a > len_b {
        return (0, len_a - len_b);
    }
    else
    if len_b > len_a {
        return (len_b - len_a, 0);
    }
    else
    {
       return (0, 0);
    }
}

fn main() {
    let general_readout = GeneralReadout::new();
    let memory_readout  = MemoryReadout::new();
    let kernel_readout  = KernelReadout::new();
    let package_readout = PackageReadout::new();

    let user                = general_readout.username();
    //let user                = get::new_error("not defined");
    let hostname            = general_readout.hostname();
    //let hostname            = get::new_error("not defined");
    let distro              = general_readout.distribution();
    //let distro              = get::new_error("not defined");
    let kernel              = kernel_readout.os_release();
    //let kernel              = get::new_error("not defined");
    let uptime              = general_readout.uptime();
    //let uptime              = get::new_error("not defined");
    
    let packages: Result<Vec<(PackageManager, usize)>, ReadoutError> = Ok(package_readout.count_pkgs());
    //let packages            = get::new_error("not defined");
    
    let shell               = general_readout.shell(libmacchina::traits::ShellFormat::Relative, libmacchina::traits::ShellKind::Current);
    //let shell               = get::new_error("not defined");
    let resolution          = general_readout.resolution();
    //let resolution          = get::new_error("not defined");
    let window_manager      = general_readout.window_manager();
    //let window_manager      = get::new_error("not defined");
    let theme               = get::theme("gtk-theme-name");
    //let theme               = get::new_error("not defined");
    let icons               = get::theme("gtk-icon-theme-name");
    //let icons               = get::new_error("not defined");
    let terminal            = general_readout.terminal();
    //let terminal            = get::new_error("not defined");
    let cpu                 = general_readout.cpu_model_name();
    //let cpu                 = get::new_error("not defined");
    let gpu                 = general_readout.gpus();
    //let gpu                 = Err(get::new_error("not defined"));
    
    let used_mem            = memory_readout.used();
    //let used_mem            = get::new_error("not defined");
    let total_mem           = memory_readout.total();
    //let total_mem           = get::new_error("not defined");
    
    let mut memory: Result<(u64, u64), ReadoutError> = Err(ReadoutError::NotImplemented);
    if used_mem.is_ok() && total_mem.is_ok() {
        memory = Ok((used_mem.unwrap(), total_mem.unwrap()));
    }

    let art = logos::ARCH_ART;
    let mut art_iter = art.iter();

    if user.is_ok() && hostname.is_ok() {
        old_print_next(&mut art_iter);
        println!("{}@{}", user.clone().unwrap().cyan().bold(), hostname.clone().unwrap().cyan().bold());
        old_print_next(&mut art_iter);
        println!("{:-<1$}", "", format!("{}@{}", user.unwrap(), hostname.unwrap()).len());
    }

    let pairs = Pairs::new(vec!(
            Pair::new("OS",         distro                          .ok()),
            Pair::new("Kernel",     kernel                          .ok()),
            Pair::new("Uptime",     format_time(        uptime      .ok())),
            Pair::new("Packages",   format_packages(    packages    .ok())),
            Pair::new("Shell",      shell                           .ok()),
            Pair::new("Resolution", resolution                      .ok()),
            Pair::new("WM",         window_manager                  .ok()),
            Pair::new("Theme",      theme                           .ok()),
            Pair::new("Icons",      icons                           .ok()),
            Pair::new("Terminal",   terminal                        .ok()),
            Pair::new("CPU",        cpu                             .ok()),
            Pair::new("GPU",        format_gpu(         gpu         .ok())),
            Pair::new("Memory",     format_memory(      memory      .ok())),
            ));

    let longest_key_lenght = pairs.0.iter().map(|x| x.key.len()).max().unwrap();

    //pairs.0.iter().for_each(|x| x.print(longest_key_lenght, &mut Box::new(art_iter.map(|x| x.to_string()) ) ));
    pairs.0.iter().for_each(|x| x.print(longest_key_lenght, &mut art_iter ));

    /* colors */ {
        old_print_next(&mut art_iter);
        println!();

        // FIXME: do not use unwrap
        let (len_normal, len_light) = format_color(
            old_print_next(&mut art_iter).unwrap().chars().filter(|x| !x.is_control()).collect::<Vec<char>>().len(),
            art_iter.clone().peekable().peek().unwrap().chars().filter(|x| !x.is_control()).collect::<Vec<char>>().len()
            );

        //println!("{}, {}", print_next(&mut art_iter).unwrap().len(), art_iter.peek().unwrap().len());

        //let (len_normal, len_light) = format_color(5, 6);

        print!("{: <1$}", "", len_normal);
        println!(
            "{}{}{}{}{}{}{}{}",
            "   ".bg_black(),
            "   ".bg_red(),
            "   ".bg_green(),
            "   ".bg_yellow(),
            "   ".bg_blue(),
            "   ".bg_magenta(),
            "   ".bg_cyan(),
            "   ".bg_light_gray(),
            );

        old_print_next(&mut art_iter);
        print!("{: <1$}", "", len_light);
        println!(
            "{}{}{}{}{}{}{}{}",
            "   ".bg_dark_gray(),
            "   ".bg_light_red(),
            "   ".bg_light_green(),
            "   ".bg_light_yellow(),
            "   ".bg_light_blue(),
            "   ".bg_light_magenta(),
            "   ".bg_light_cyan(),
            "   ".bg_color(colorful::Color::White),
            );
    }

    while old_print_next(&mut art_iter).is_some() {
        println!();
    };

    println!();

}
