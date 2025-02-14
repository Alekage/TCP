use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    let  nbytes = nic.recv(&mut buf[..])?;

    eprintln!("Read {} bytes:\n{}", nbytes, hex::encode(&buf[..nbytes]));
    Ok(())
}
