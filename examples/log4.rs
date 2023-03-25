// use error_chain::error_chain;

use anyhow::Result;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

// error_chain! {
//    foreign_links {
//        Io(std::io::Error);
//        LogConfig(log4rs::config::Errors);
//        SetLogger(log::SetLoggerError);
//    }
// }

fn main() -> Result<()> {
    // 创建日志配置，并指定输出的位置
    let logfile = FileAppender::builder()
        // 编码模式的详情参见: https://docs.rs/log4rs/1.0.0/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    log::info!("Hello, world!");

    Ok(())
}

