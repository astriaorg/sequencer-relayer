use eyre::WrapErr as _;
use jsonrpsee_core::RpcResult;
use jsonrpsee_server::{RpcModule, ServerBuilder, ServerHandle};

pub struct RpcServer {
    server_handle: ServerHandle,
}

impl RpcServer {
    pub async fn new(listen_addr: &str, listen_port: u16) -> eyre::Result<RpcServer> {
        let addrs: &[std::net::SocketAddr] =
            &[format!("{}:{}", listen_addr, listen_port).parse().unwrap()];

        let server = ServerBuilder::default()
            .build(addrs)
            .await
            .wrap_err("failed to build RpcServer")?;

        let mut module = RpcModule::new(());
        module.register_method::<RpcResult<u64>, _>("system_health", |params, _| {
            params.one::<u64>().map_err(Into::into)
        })?;

        let server_handle = server.start(module)?;
        Ok(RpcServer { server_handle })
    }

    pub async fn stop(self) -> eyre::Result<()> {
        self.server_handle
            .stop()
            .wrap_err("failed to stop RpcServer")
    }
}
