use tokio::prelude::*;
use tuntap::*;

#[tokio::main]
async fn main() {
	let flags = TunTapFlags::IFF_TUN | TunTapFlags::IFF_NO_PI;
	let (mut tun1, (mut tx1, mut rx1)) = TunTap::new(flags).await.unwrap();
	let (mut tun2, (mut tx2, mut rx2)) = TunTap::new(flags).await.unwrap();

	tun1.set_owner(1001).unwrap();
	tun1.set_group(1001).unwrap();
	//tun.set_mac([11,22,33,44,55,66+i as u8]).unwrap();

	tun1.set_ip("10.0.0.1".parse().unwrap()).unwrap();
	tun1.set_ip("fe80::0db8:1234:1211".parse().unwrap()).unwrap();

	tun1.set_mtu(1400).unwrap();
	tun1.set_up().unwrap();


	tun2.set_owner(1001).unwrap();
	tun2.set_group(1001).unwrap();
	//tun.set_mac([11,22,33,44,55,66+i as u8]).unwrap();

	tun2.set_ip("10.0.0.2".parse().unwrap()).unwrap();
	tun2.set_ip("fe80::0db8:1234:1212".parse().unwrap()).unwrap();

	tun2.set_mtu(1400).unwrap();
	tun2.set_up().unwrap();

	tokio::spawn(async move {
		println!("send0");
		while let Some(packet) = rx1.next().await {
			println!("rx1 {}", packet.len());
			tx2.send(packet).await.unwrap();
		}
	});

	while let Some(packet) = rx2.next().await {
		println!("rx2 {}", packet.len());
		tx1.send(packet).await.unwrap();
	}
}
