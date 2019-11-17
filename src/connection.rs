use futures::prelude::*;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::net::TcpStream;
use tokio_tungstenite::{
    self as async_ws, tungstenite::Message as WsMessage, MaybeTlsStream, WebSocketStream,
};

use crate::message::{client, server};
use crate::{Error, Server};

pub struct Connection(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl Stream for Connection {
    type Item = Result<server::Message, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let pinned = Pin::new(&mut self.0);
        match Stream::poll_next(pinned, cx) {
            Poll::Ready(Some(Ok(message))) => match message {
                WsMessage::Text(ref data) => Poll::Ready(Some(
                    server::Message::from_slice(data.as_bytes()).map_err(Error::from),
                )),
                WsMessage::Binary(ref data) => {
                    Poll::Ready(Some(server::Message::from_slice(data).map_err(Error::from)))
                }
                _ => Poll::Pending,
            },
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err.into()))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Sink<client::Message> for Connection {
    type Error = crate::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let pinned = Pin::new(&mut self.0);
        pinned.poll_ready(cx).map_err(Error::from)
    }

    fn start_send(mut self: Pin<&mut Self>, item: client::Message) -> Result<(), Self::Error> {
        let pinned = Pin::new(&mut self.0);
        pinned
            .start_send(WsMessage::Text(item.to_string()))
            .map_err(Error::from)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let pinned = Pin::new(&mut self.0);
        pinned.poll_flush(cx).map_err(Error::from)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let pinned = Pin::new(&mut self.0);
        pinned.poll_close(cx).map_err(Error::from)
    }
}

impl Connection {
    pub async fn connect(server: &Server) -> Result<Connection, Error> {
        let url = server.url().expect("Invalid server URL provided");
        let (connection, _response) = async_ws::connect_async(url).await?;
        Ok(Connection(connection))
    }

    pub async fn identify(
        &mut self,
        ticket: &crate::Ticket,
        character: String,
        client_name: String,
        client_version: String,
    ) -> Result<(), Error> {
        let idn = client::Message::IDN {
            method: crate::enums::IdnMethod::Ticket,
            ticket: ticket.ticket(),
            account: ticket.username.clone(),
            character,
            cname: client_name,
            cversion: client_version,
        };
        self.send(idn).await
    }
}
