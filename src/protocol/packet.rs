
const MAGIC: [u8; 16] = [0x00, 0xff, 0xff, 0x00,
						 0xfe, 0xfe, 0xfe, 0xfe,
						 0xfd, 0xfd, 0xfd, 0xfd,
						 0x12, 0x34, 0x56, 0x78];

pub enum PacketTypes {
	/// 0x00
	///
	/// # Parameters
	///
	/// 1. Time, `long`
	ConnectedPing(i64),
	/// 0x01, 0x02
	///
	/// # Parameters
	///
	/// 1. Time, `long`
	/// 2. Magic, `Magic`
	UnconnectedPing(i64, u8),
	/// 0x03
	///
	/// # Parameters
	///
	/// 1. Ping time, `long`
	/// 2. Pong time, `long`
	ConnectedPong(i64, i64),
	/// 0x05
	///
	/// # Parameters
	///
	/// 1. Magic, `magic`
	/// 2. Protocol version, `byte`.
	/// 3. MTU, exact type is unknown.
	OfflineConnectionRequest1(u8, i8, i16),
	/// 0x06
	///
	/// # Parameters
	///
	/// 1. Magic, `magic`
	/// 2. Server Address, `Address`
	/// 3. MTU, `short`
	/// 4. Client GUID `long`
	OfflineConnectionResponse1(u8, i64, bool, i64),
	/// 0x07
	///
	/// # Parameters
	///
	/// 1. Magic, `magic`
	/// 2. Server GUID, `long`
	/// 3. Client Address, `Address`
	/// 4. MTU, `short`
	/// 5. Encryption status, `byte`. Acts as a boolean.
	OfflineConnectionRequest2(u8, i64, String, i16, i8),
	/// 0x08
	///
	/// # Parameters
	///
	/// 1. Magic, `magic`
	/// 2. Server GUID, `long`
	/// 3. Client Address, `address`
	/// 4. MTU, `short`
	/// 5. Encryption status, `byte`. Acts as a boolean.
	OfflineConnectionResponse2(u8, i64, String, i16, i8),
	/// 0x09
	///
	/// # Parameters
	///
	/// 1. GUID, `long`
	/// 2. Time, `long`
	OnlineConnectionRequest(i64, i64),
	/// 0x10
	///
	/// # Parameters
	///
	/// 1. Client Address, `string`
	/// 2. System index, `short`
	/// 3. Internal IDs, `[address; 10]`
	/// 4. Request Time, `long`
	/// 5. Time, `long`
	OnlineConnectionRequestAccepted(String, i16, [String; 10], i64, i64),
	/// 0x19
	///
	/// # Parameters
	///
	/// 1. Protocol, `byte`
	/// 2. Magic, `magic`
	/// 3. Server GUID, `long`
	IncompatibleProtocol(i32, u8, i64)
}
