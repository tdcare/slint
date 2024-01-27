// slint::include_modules!();

use std::str::FromStr;
use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::time::Duration;
// use tokio_util::codec::{BytesCodec, FramedRead};

use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::APIBuilder;
use webrtc::data_channel::RTCDataChannel;
use webrtc::ice_transport::ice_connection_state::RTCIceConnectionState;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;


#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PEER_CONNECTION_MUTEX: Arc<Mutex<Option<Arc<RTCPeerConnection>>>> =
        Arc::new(Mutex::new(None));
}


#[tokio::main]
pub async  fn main() {
    // let main_window = Demo::new().unwrap();
    // main_window.run().unwrap();
    let pc = {
        let mut peer_connection = PEER_CONNECTION_MUTEX.lock().await;
        if let Some(pc) = &*peer_connection {
            Arc::clone(pc)
        } else {
            // Create a MediaEngine object to configure the supported codec
            let mut m = MediaEngine::default();

            match m.register_default_codecs() {
                Ok(_) => {}
                Err(err) => panic!("{}", err),
            };

            // Create a InterceptorRegistry. This is the user configurable RTP/RTCP Pipeline.
            // This provides NACKs, RTCP Reports and other features. If you use `webrtc.NewPeerConnection`
            // this is enabled by default. If you are manually managing You MUST create a InterceptorRegistry
            // for each PeerConnection.
            let mut registry = Registry::new();

            // Use the default set of Interceptors
            registry = match register_default_interceptors(registry, &mut m) {
                Ok(r) => r,
                Err(err) => panic!("{}", err),
            };

            // Create the API object with the MediaEngine
            let api = APIBuilder::new()
                .with_media_engine(m)
                .with_interceptor_registry(registry)
                .build();

            // Create a new RTCPeerConnection
            let pc = match api.new_peer_connection(RTCConfiguration::default()).await {
                Ok(p) => p,
                Err(err) => panic!("{}", err),
            };
            let pc = Arc::new(pc);

            // Set the handler for ICE connection state
            // This will notify you when the peer has connected/disconnected
            pc.on_ice_connection_state_change(Box::new(
                |connection_state: RTCIceConnectionState| {
                    println!("ICE Connection State has changed: {connection_state}");
                    Box::pin(async {})
                },
            ));

            // Send the current time via a DataChannel to the remote peer every 3 seconds
            pc.on_data_channel(Box::new(|d: Arc<RTCDataChannel>| {
                Box::pin(async move {
                    let d2 = Arc::clone(&d);
                    d.on_open(Box::new(move || {
                        Box::pin(async move {
                            while d2
                                .send_text(format!("{:?}", tokio::time::Instant::now()))
                                .await
                                .is_ok()
                            {
                                tokio::time::sleep(Duration::from_secs(3)).await;
                            }
                        })
                    }));
                })
            }));

            *peer_connection = Some(Arc::clone(&pc));
            pc
        }
    };


}