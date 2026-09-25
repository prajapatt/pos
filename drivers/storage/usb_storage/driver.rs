#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct CommandBlockWrapper { pub signature: u32, pub tag: u32, pub transfer_length: u32, pub flags: u8, pub lun: u8, pub command_length: u8, pub command: [u8; 16] }

impl CommandBlockWrapper { pub fn read10(tag: u32, lba: u32, blocks: u16, block_size: u32) -> Self { let mut command = [0; 16]; command[0] = 0x28; command[2..6].copy_from_slice(&lba.to_be_bytes()); command[7..9].copy_from_slice(&blocks.to_be_bytes()); Self { signature: 0x4342_5355, tag, transfer_length: u32::from(blocks) * block_size, flags: 0x80, lun: 0, command_length: 10, command } } }
