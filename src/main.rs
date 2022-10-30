use std::{net::SocketAddr, time::Duration, sync::Arc};

use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};

async fn handle_incoming(mut s: TcpStream, needle: Arc<Vec<u8>>, sa: SocketAddr, prepend: Arc<Vec<u8>>) -> anyhow::Result<()> {
    if needle.len() > 0 {
        let mut buf : Vec<u8> = Vec::with_capacity(needle.len()*2);
        'wait_needle: loop {
            let mut b = [0u8; 1];
            match s.read(&mut b).await? {
                0 => anyhow::bail!("Premature end of client stream"),
                1 => {
                    buf.push(b[0]);
                    if buf.len() > needle.len() {
                        if &buf[(buf.len() - needle.len())..buf.len()] == &needle[..] {
                            eprintln!("  found matching request bytes");
                            break 'wait_needle;
                        }
                    }
                    if buf.len() >= needle.len()*2 {
                        buf.copy_within(needle.len().., 0);
                        buf.resize(needle.len(), 0);
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    let mut c = TcpStream::connect(sa).await?;

    eprintln!("   connected to upstream");
    s.write_all(&prepend).await?;
    eprintln!("   wrote prepender bytes");

    tokio::io::copy_bidirectional(&mut c, &mut s).await?;

    eprintln!("   finished proxying");
    
    Ok(())
}

#[tokio::main(flavor="current_thread")]
async fn main()  -> anyhow::Result<()>{
    let opts = xflags::parse_or_exit!(
        required listen : SocketAddr
        required request_needle_base64: String
        required connect : SocketAddr
        required response_prepend_base64: String
    );

    let needle = Arc::new(base64::decode(opts.request_needle_base64)?);
    let prepend = Arc::new(base64::decode(opts.response_prepend_base64)?);

    let l : TcpListener = TcpListener::bind(opts.listen).await?;
    loop {
        match l.accept().await {
            Err(e) => {
                eprintln!("Accept error: {}", e);
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            Ok((s,a)) => {
                eprintln!("Incoming connection from {}", a);
                let needle = needle.clone();
                let prepend = prepend.clone();
                tokio::spawn( async move {
                    if let Err(e) = handle_incoming(s, needle, opts.connect, prepend).await {
                        eprintln!("   {}", e);
                    }
                });
            }
        }
    }
}
