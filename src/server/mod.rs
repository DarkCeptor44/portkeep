// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod api;
mod utils;

use crate::{
    App, AppArgs, NAME, VERSION,
    config::Data,
    server::utils::{ServerArgs, Service, init_logger},
};
use anyhow::{Context, Result, anyhow};
use log::{debug, error, info};
use parking_lot::RwLock;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    process::exit,
    sync::Arc,
};
use tokio::net::TcpListener;

pub async fn handle_server(args: App, config: Data) -> Result<()> {
    let AppArgs::Serve { host, port, debug } = args.command else {
        return Ok(());
    };

    if let Err(e) = init_logger(debug).context("Failed to init logger") {
        eprintln!("CRITICAL: {e:?}");
        exit(1);
    }

    info!(
        "\n===================================================\n------------------ PortKeep v{VERSION} ------------------\n===================================================\n",
    );
    let server_args = ServerArgs { host, port, debug };
    if let Err(e) = serve(config, server_args).await {
        error!("{NAME} application logic: {e:?}");
        exit(1);
    }

    Ok(())
}

async fn serve(config: Data, args: ServerArgs) -> Result<()> {
    let service = Service {
        config: RwLock::new(config),
    };
    debug!("service={service:?}");

    let app = api::routes(&service).with_state(Arc::new(service));
    let addr = SocketAddr::new(
        args.host
            .parse::<IpAddr>()
            .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
        args.port,
    );
    let listener = TcpListener::bind(addr)
        .await
        .context(anyhow!("Failed to bind to address: {addr}"))?;
    info!(
        "\n    listening on http://{}:{}\n    listening on http://localhost:{}\n",
        args.host, args.port, args.port
    );

    let shutdown_signal = async {
        let event: &str;

        #[cfg(unix)]
        {
            let mut terminate =
                tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                    .expect("Failed to install signal handler");

            tokio::select! {
                _ = ctrl_c => event = "SIGINT",
                _ = terminate.recv() => event = "SIGTERM",
            };
        }

        #[cfg(windows)]
        {
            use tokio::signal::windows::{ctrl_break, ctrl_c};
            let mut sig_c = ctrl_c().expect("failed to install ctrl+c handler");
            let mut sig_break = ctrl_break().expect("failed to install ctrl+break handler");

            tokio::select! {
                _ = sig_c.recv() => event = "CTRL_C_EVENT",
                _ = sig_break.recv() => event = "CTRL_BREAK_EVENT",
            };
        }

        info!("{event} signal received, shutting down...");
    };

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal)
        .await
        .context("Failed to serve axum app")
}
