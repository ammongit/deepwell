/*
 * server.rs
 *
 * deepwell-rpc - RPC server to provide database management and migrations
 * Copyright (C) 2019-2020 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::api::{Deepwell as DeepwellApi, PROTOCOL_VERSION};
use crate::Result;
use futures::future::{self, Ready};
use futures::prelude::*;
use std::io;
use std::net::SocketAddr;
use std::time::SystemTime;
use tarpc::context::Context;
use tarpc::serde_transport::tcp;
use tarpc::server::{BaseChannel, Channel};
use tokio_serde::formats::Json;

#[derive(Debug, Clone)]
pub struct Server;

impl Server {
    #[inline]
    pub fn new() -> Self {
        Server
    }

    pub async fn run(&self, address: SocketAddr) -> io::Result<()> {
        tcp::listen(&address, Json::default)
            .await?
            // Log requests
            .filter_map(|conn| {
                async move {
                    match conn {
                        Ok(conn) => {
                            match conn.peer_addr() {
                                Ok(addr) => info!("Accepted connection from {}", addr),
                                Err(error) => warn!("Unable to get peer address: {}", error),
                            }

                            Some(conn)
                        }
                        Err(error) => {
                            warn!("Error accepting connection: {}", error);

                            None
                        }
                    }
                }
            })
            // Create and fulfill channels for each request
            .map(BaseChannel::with_defaults)
            .map(|chan| {
                let resp = self.clone().serve();
                chan.respond_with(resp).execute()
            })
            .for_each(|fut| fut)
            .await;

        Ok(())
    }
}

impl DeepwellApi for Server {
    // Misc

    type ProtocolFut = Ready<String>;

    #[inline]
    fn protocol(self, _: Context) -> Self::ProtocolFut {
        info!("Method: protocol");

        future::ready(str!(PROTOCOL_VERSION))
    }

    type PingFut = Ready<String>;

    #[inline]
    fn ping(self, _: Context) -> Self::PingFut {
        info!("Method: ping");

        future::ready(str!("pong!"))
    }

    type TimeFut = Ready<f64>;

    #[inline]
    fn time(self, _: Context) -> Self::TimeFut {
        info!("Method: time");

        let now = SystemTime::now();
        let unix_time = now
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("System time before epoch")
            .as_secs_f64();

        future::ready(unix_time)
    }

    // TODO
}