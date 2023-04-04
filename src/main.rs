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

use math;

use colorful::Colorful;
//use colorful::Color;


fn print_next<'a, I: Iterator>(iter: &mut I) -> Option<<I as Iterator>::Item> where <I as Iterator>::Item: std::fmt::Display {
    let next = iter.next();
    print!("{}     ", match next {
        Some(ref expr) => expr,
        None => {
            return None;
        },
    });
    return next;
}

fn format_packages(packages: Vec<(PackageManager, usize)>) -> String {
    let packages_iter = packages.iter();
    let mut string = String::new();

    for next in packages_iter {
        string += &(next.1.to_string() + " (");
        string += &(next.0.to_string() + "), ");
    }

    string.pop();
    string.pop();

    return string;
}
fn format_time(uptime: usize) -> (f64, f64) {
    let uptime_hours_seconds = math::round::floor((uptime as f64 / 60.0) / 60.0, 0) * 60.0 * 60.0;
    return (
        (uptime_hours_seconds / 60.0) / 60.0,
        math::round::floor((uptime as f64 - uptime_hours_seconds) / 60.0, 0)
        );
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
fn format_gpu(gpu: Vec<String>) -> String {
    let gpu_iter = gpu.iter();
    let mut string = String::new();

    for next in gpu_iter {
        string += &(next.to_string() + ", ");
    }

    string.pop();
    string.pop();

    return string;
}
fn format_memory(used_mem: u64, total_mem: u64) -> (f32, f32) {
    return (
        (used_mem   as f32 / 1024.0) / 1024.0,
        (total_mem  as f32 / 1024.0) / 1024.0
        );
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
    let mut art_iter = logos::ARCH_ART.iter().peekable();

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
    let packages            = package_readout.count_pkgs();
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
    let terminal_font       = get::new_error("unfinished (this one is really hard)");
    //let terminal_font       = get::new_error("not defined");
    let cpu                 = general_readout.cpu_model_name();
    //let cpu                 = get::new_error("not defined");
    let gpu                 = general_readout.gpus();
    //let gpu                 = Err(get::new_error("not defined"));
    let total_mem           = memory_readout.total();
    //let total_mem           = get::new_error("not defined");
    let used_mem            = memory_readout.used();
    //let used_mem            = get::new_error("not defined");

    if user.is_ok() && hostname.is_ok() {
        print_next(&mut art_iter);
        println!("{}@{}", user.clone().unwrap().cyan().bold(), hostname.clone().unwrap().cyan().bold());
        print_next(&mut art_iter);
        println!("{:-<1$}", "", format!("{}@{}", user.unwrap(), hostname.unwrap()).len());
    }

    if distro.is_ok() {
        print_next(&mut art_iter);
        println!("{}: {}", "OS".cyan().bold(), distro.unwrap());
    }

    if kernel.is_ok() {
        print_next(&mut art_iter);
        println!("{}: {}", "Kernel".cyan().bold(), kernel.unwrap());
    }

    if uptime.is_ok() {
        print_next(&mut art_iter);
        let formatted_time = format_time(uptime.unwrap());
        println!("{}: {:.0} Hours, {:.0} Minutes", "Uptime".cyan().bold(), formatted_time.0, formatted_time.1);
    }

    /* packages */ {
        print_next(&mut art_iter);
        println!("{}: {}", "Packages".cyan().bold(), format_packages(packages));
    }

    if shell.is_ok() {
        print_next(&mut art_iter);
        println!("{}: {}", "Shell".cyan().bold(), shell.unwrap().remove_newline());
    }

    if resolution.is_ok() {
        print_next(&mut art_iter);
        println!("{}: {}", "Resolution".cyan().bold(), resolution.unwrap());
    }

    if window_manager.is_ok() {
        print_next(&mut art_iter);
        println!("{}: {}", "WM".cyan().bold(), window_manager.unwrap());
    }

    if theme.is_ok() {
        print_next(&mut art_iter);
        println!("{}: {}", "Theme".cyan().bold(), theme.unwrap().remove_newline());
    }

    if icons.is_ok() {
        print_next(&mut art_iter);
        println!("{}: {}", "Icons".cyan().bold(), icons.unwrap().remove_newline());
    }

    if terminal.is_ok() {
        print_next(&mut art_iter);
        println!("{}: {}", "Terminal".cyan().bold(), terminal.unwrap().remove_newline());
    }

    if terminal_font.is_ok() {
        print_next(&mut art_iter);
        println!("{}: {}", "Terminal Font".cyan().bold(), terminal_font.unwrap());
    }

    if cpu.is_ok() {
        print_next(&mut art_iter);
        println!("{}: {}", "CPU".cyan().bold(), cpu.unwrap());
    }

    if gpu.is_ok() {
        print_next(&mut art_iter);
        println!("{}: {}", "GPU".cyan().bold(), format_gpu(gpu.unwrap()));
    }

    if used_mem.is_ok() && total_mem.is_ok() {
        print_next(&mut art_iter);
        let formatted_memory = format_memory(used_mem.unwrap(), total_mem.unwrap());
        println!("{}: {:.2}GiB / {:.2}GiB", "Memory".cyan().bold(), formatted_memory.0, formatted_memory.1);
    }

    /* colors */ {
        print_next(&mut art_iter);
        println!();

        // FIXME: do not use unwrap
        let (len_normal, len_light) = format_color(
            print_next(&mut art_iter).unwrap().chars().filter(|x| !x.is_control()).collect::<Vec<char>>().len(),
            art_iter.peek().unwrap().chars().filter(|x| !x.is_control()).collect::<Vec<char>>().len()
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

        print_next(&mut art_iter);
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




    while print_next(&mut art_iter).is_some() {
        println!();
    };

    println!();

}
