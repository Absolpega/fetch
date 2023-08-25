mod get;

use std::cmp::Ordering;

use libmacchina::traits::PackageManager;
use libmacchina::{GeneralReadout, KernelReadout, MemoryReadout, PackageReadout};

use libmacchina::traits::GeneralReadout as _;
use libmacchina::traits::KernelReadout as _;
use libmacchina::traits::MemoryReadout as _;
use libmacchina::traits::PackageReadout as _;
use libmacchina::traits::ReadoutError;

use colorful::Colorful;
//use colorful::Color;

const SPACING: usize = 5;

struct Pair {
    key: String,
    value: Option<String>,
}

//let longest_key_lenght = pairs.0.iter().map(|x| x.key.len()).max().unwrap();

impl Pair {
    fn new(key: &str, value: Option<String>) -> Self {
        Self {
            key: key.to_string(),
            value, // same as value: value
        }
    }

    fn print(&self, longest_key_lenght: usize, art: &mut dyn std::iter::Iterator<Item = &str>) {
        self.value.clone().and_then(|v| {
            art.next().and_then(|a| {
                println!(
                    "{}{}{}: {}{}",
                    a,
                    " ".repeat(SPACING),
                    self.key.clone().cyan().bold(),
                    " ".repeat(longest_key_lenght - self.key.len()),
                    v.remove_newline()
                );

                None::<()>
            });
            None::<()>
        });
    }
}

fn print_next_art_line<I: Iterator>(iter: &mut I) -> Option<<I as Iterator>::Item>
where
    <I as Iterator>::Item: std::fmt::Display,
{
    let next = iter.next();
    print!(
        "{}{}",
        match next {
            Some(ref expr) => expr,
            None => {
                return None;
            }
        },
        " ".repeat(SPACING)
    );
    next
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
        string
    }
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

            Some(string)
        }
        None => None,
    }
}
fn format_time(uptime: Option<usize>) -> Option<String> {
    match uptime {
        Some(uptime) => {
            let uptime_hours_seconds =
                math::round::floor((uptime as f64 / 60.0) / 60.0, 0) * 60.0 * 60.0;
            Some(format!(
                "{:.0} Hours, {:.0} Minutes",
                (uptime_hours_seconds / 60.0) / 60.0,
                math::round::floor((uptime as f64 - uptime_hours_seconds) / 60.0, 0)
            ))
        }
        None => None,
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

            Some(string)
        }
        None => None,
    }
}
fn format_memory(memory: Option<(u64, u64)>) -> Option<String> {
    memory.map(|(used, total)| {
        format!(
            "{:.2}GiB / {:.2}GiB",
            (used as f32 / 1024.0) / 1024.0,
            (total as f32 / 1024.0) / 1024.0
        )
    })
}

trait AddTuple {
    fn add(self, other: Self) -> Self;
}

impl AddTuple for (usize, usize) {
    fn add(self, other: Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }
}

fn main() {
    let general_readout = GeneralReadout::new();
    let memory_readout = MemoryReadout::new();
    let kernel_readout = KernelReadout::new();
    let package_readout = PackageReadout::new();

    let user = general_readout.username();
    let hostname = general_readout.hostname();

    let distro = general_readout.distribution().or(general_readout.os_name());
    let kernel = kernel_readout.os_release();
    let uptime = general_readout.uptime();

    let packages: Result<Vec<(PackageManager, usize)>, ReadoutError> =
        Ok(package_readout.count_pkgs());

    let shell = general_readout.shell(
        libmacchina::traits::ShellFormat::Relative,
        libmacchina::traits::ShellKind::Current,
    );

    let resolution = general_readout.resolution();
    let window_manager = general_readout.window_manager().or(get::window_manager());
    let desktop_environment = general_readout.desktop_environment();
    let theme = get::theme("gtk-theme-name");
    let icons = get::theme("gtk-icon-theme-name");
    let terminal = general_readout.terminal();
    let cpu = general_readout.cpu_model_name();
    let gpu = general_readout.gpus();

    let memory = match (memory_readout.used(), memory_readout.total()) {
        (Ok(used_mem), Ok(total_mem)) => Ok((used_mem, total_mem)),
        _ => Err(ReadoutError::MetricNotAvailable),
    };

    let art = include_str!("arch.txt");
    let mut art_iter = art.lines();

    if let (Ok(user), Ok(hostname)) = (user, hostname) {
        print_next_art_line(&mut art_iter);
        println!(
            "{}@{}",
            user.clone().cyan().bold(),
            hostname.clone().cyan().bold()
        );

        print_next_art_line(&mut art_iter);
        println!(
            "{}",
            "-".repeat(format!("{}@{}", user, hostname).chars().count())
        );
    }

    let pairs = vec![
        Pair::new("OS", distro.ok()),
        Pair::new("Kernel", kernel.ok()),
        Pair::new("Uptime", format_time(uptime.ok())),
        Pair::new("Packages", format_packages(packages.ok())),
        Pair::new("Shell", shell.ok()),
        Pair::new("Resolution", resolution.ok()),
        Pair::new("DE", desktop_environment.ok()),
        Pair::new("WM", window_manager.ok()),
        Pair::new("Theme", theme.ok()),
        Pair::new("Icons", icons.ok()),
        Pair::new("Terminal", terminal.ok()),
        Pair::new("CPU", cpu.ok()),
        Pair::new("GPU", format_gpu(gpu.ok())),
        Pair::new("Memory", format_memory(memory.ok())),
    ];

    let longest_key_lenght = pairs.iter().map(|x| x.key.len()).max().unwrap();

    pairs
        .iter()
        .for_each(|x| x.print(longest_key_lenght, &mut art_iter));

    /* colors */
    {
        // FIXME: tomorrow

        print_next_art_line(&mut art_iter);
        println!();

        let next = print_next_art_line(&mut art_iter.clone())
            .map(|x| x.chars().filter(|c| !c.is_control()).count());

        art_iter.next();

        let peek = art_iter
            .clone()
            .peekable()
            .peek()
            .copied()
            .map(|x| x.chars().filter(|c| !c.is_control()).count());

        let last_len = art
            .lines()
            .last()
            .unwrap()
            .chars()
            .filter(|c| !c.is_control())
            .count();

        let (len_normal, len_light) = match (next, peek) {
            (Some(next), Some(peek)) => match next.cmp(&peek) {
                Ordering::Greater => (0, next - peek),
                Ordering::Less => (peek - next, 0),
                _ => (0, 0),
            },
            (Some(next), None) => (next, last_len + next + SPACING),
            (None, None) => (last_len + SPACING, last_len + SPACING),
            _ => (0, 0),
        };

        print!("{}", " ".repeat(len_normal));
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

        print_next_art_line(&mut art_iter);
        print!("{}", " ".repeat(len_light));
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

    while print_next_art_line(&mut art_iter).is_some() {
        println!();
    }

    println!();
}
