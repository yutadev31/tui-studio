use chrono::Local;
use fern::Dispatch;

pub fn init_logger() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
