#![allow(dead_code)]
mod array;
mod buffer;
mod closure;
mod enumeration;
mod format;
// mod iter;
mod list;
mod matchs;
mod mini_string;
mod parse;
mod size_of;
mod structure;
mod tuple;
mod types;
mod vtable;
mod traits;
mod phantom;
mod errors;
mod boxed;
mod paths;
mod files;
mod processes;
mod env;
mod ffi;
mod sort;
mod gzip;
mod digest;
mod claps;
mod stack;
mod string;
mod sort_zh;

#[derive(Debug, Default)]
enum Kind {
    #[default]
    A,
    B,
    C,
}

// fn main() -> std::io::Result<()> {
// 	use std::os::windows::process::CommandExt;
// 	std::process::Command::new("./test.bat")
// 		.env("PATH^", "123")
// 		.raw_arg("^%PATH^%")
// 		.spawn()?
// 		.wait()?;
// 	Ok(())
// }

// fn main() {
    // let order1 = Kind::default();
    // println!("{:?}", order1);
    
    // size_of::show_size_fn();
    // println!("{}", "-".repeat(64));
    // vtable::vtest();
    // println!("{}", "-".repeat(64));
    // buffer::main_run();
    // println!("{}", "-".repeat(64));
    // format::main_fn();
    // println!("{}", "-".repeat(64));
    // tuple::main();
    // array::main();
    // structure::main();
    // enumeration::main();
    // types::main();
    // iter::main();
    // matchs::main();
    // closure::main();
    // traits::main();
    // phantom::main();
    // errors::main();
    // boxed::main();
    // paths::main();
    // files::main();
    // processes::main();
    // env::main();
    // ffi::main();
    // sort::main_run();
    // gzip::main_run();
    // digest::main_run();
    // claps::main_run();
    
    // string::main();

    // sort_zh::sort_zh_test();// 测试中文排序

// }

// https://youerning.top
use async_trait::async_trait;
use log::info;
use pingora_core::services::background::background_service;
use std::{sync::Arc, time::Duration};
use structopt::StructOpt;

use pingora_core::server::configuration::Opt;
use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::{Result, Error, ErrorType};
use pingora_load_balancing::{health_check, selection::RoundRobin, LoadBalancer};
use pingora_proxy::{ProxyHttp, Session};

// 随便定义一个可以实现trait的struct对象
// 为了简单起见当然是要包含Pingora提供的负载均衡对象啦.
pub struct LB(Arc<LoadBalancer<RoundRobin>>);


#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    // ProxyHttp唯一必须要自己定义的方法, 也就是每次请求来会调用这个方法以选择后端(upstream)
    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = match self
            .0
            // 这里使用的负载均衡算法是RoundRobin, 所以key不重要, key是用用于hash算法的
            // 后面做灰度发布会用到hash算法
            .select(b"", 256) { 
                Some(upstream) => upstream,
                None => {
                    // 因为本代码中创建了一个健康检查的服务，所以可能会出现没有后端的情况
                    return Err(Error::new(ErrorType::new("没有健康的后端可以选择.")))
                }
            };
            

        info!("选择的后端是: {:?}", upstream);
        let peer = Box::new(HttpPeer::new(upstream, false, "".to_string()));
        Ok(peer)
    }

    // Pingora提供了很多的钩子函数，这个钩子函数用于修改或者说过滤客户端请求
    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut pingora_http::RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        // 将发送给后端的请求插入一个Host请求头
        upstream_request
            .insert_header("Host", "youering.top")
            .unwrap();
        Ok(())
    }
}

fn main() {
    // 初始化日志服务，默认级别是Error及以上才会显示, 可以通过RUST_LOG=INFO来调整日志输出级别
    env_logger::init();

    // 读取命令行参数
    let opt = Opt::from_args();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    // 127.0.0.1:343 是一个不存在的服务, 这样就会在Pingora的输出看到一个错误，说127.0.0.1:343不健康
    // 不健康的服务就不会被选择了
    let mut upstreams =
        LoadBalancer::try_from_iter(["127.0.0.1:10081", "127.0.0.1:10082", "127.0.0.1:343"]).unwrap();

    // 这里创建了一个健康检查的服务，健康检查服务会保证不健康的服务不会被选择
    let hc = health_check::TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(Duration::from_secs(1));

    // 创建一个后台服务
    let background = background_service("健康检查", upstreams);
    let upstreams = background.task();

    // 创建一个代理服务
    let mut lb = pingora_proxy::http_proxy_service(&my_server.configuration, LB(upstreams));
    lb.add_tcp("0.0.0.0:10080");

    my_server.add_service(lb);
    my_server.add_service(background);
    my_server.run_forever();
}