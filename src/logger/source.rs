use std::panic::Location;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

#[track_caller]
pub fn get_caller_location() -> &'static Location<'static> {
    Location::caller()
}

pub fn setup_logger() -> Result<(), fern::InitError> {

    let mut colors = ColoredLevelConfig::new();
    colors.warn = Color::Yellow;
    colors.info = Color::Green;
    colors.error = Color::Red;
    colors.debug = Color::BrightBlack;
    colors.trace = Color::Magenta;

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}: {} {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}