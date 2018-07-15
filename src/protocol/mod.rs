
mod packet;

mod types {
	const Encapsulated: i8 = 0x01;
	const OpenSession: i8 = 0x02;
	const CloseSession: i8 = 0x03;
	const InvalidSession: i8 = 0x04;
	const SendQueue: i8 = 0x05;
	const AckNotification: i8 = 0x06;
	const SetOption: i8 = 0x07;
	const Raw: i8 = 0x08;
	const BlockAddress: i8 = 0x09;
	const UnblockAddress: i8 = 0x10;
	const Shutdown: i8 = 0x7e;
	const EmergencyShutdown: i8 = 0x7f;
}
