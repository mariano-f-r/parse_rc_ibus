//! # parse_rc_ibus
//!
//! A crate meant to make parsing FlySky IBUS packets easy.
//!
//! FlySky IBUS is a serial communications protocol that contains 14 channels of data and a
//! checksum. Many of their receivers output this protocol.

#![no_std]

use core::error::Error;
use core::fmt;

#[derive(Debug)]
pub struct IbusPacket {
    channels: [u16; 14],
}

impl IbusPacket {
    /// Tries to create an IBUS packet from a group of 32 bytes.
    ///
    /// Will return an Err value if:
    /// - The lead byte is not `0x20`
    /// - The second byte is not `0x40`
    /// - The checksum fails  
    pub fn try_from_bytes(bytes: &[u8; 32]) -> Result<Self, ParsingError> {
        if bytes[0] != 0x20 || bytes[1] != 0x40 {
            return Err(ParsingError::InvalidPacket);
        }

        let mut channels = [0u16; 14];

        let mut channels_iter = bytes[2..30].iter();
        let mut channel_sum = 0u16;
        for idx in 0..14 {
            let low_byte = *channels_iter.next().unwrap();
            let high_byte = *channels_iter.next().unwrap();
            let channel = ((high_byte as u16) << 8) | low_byte as u16;
            channel_sum += low_byte as u16 + high_byte as u16;
            channels[idx] = channel;
        }

        channel_sum += bytes[1] as u16 + bytes[0] as u16;
        let calculated_checksum: u16 = (0xFFFF as u16) - channel_sum;
        let actual_checksum = ((bytes[31] as u16) << 8) | bytes[30] as u16;
        if calculated_checksum == actual_checksum {
            Ok(IbusPacket { channels })
        } else {
            return Err(ParsingError::FailsChecksum);
        }
    }

    /// Gets an individual channel's data, which is a value between 1000 and 2000. There are 14
    /// channels, but only some of them will have data that changes from 1500 (the
    /// default for inactive channels). Returns None if you select a channel out of range.
    /// Starts indexing at 1
    pub fn get_channel(&self, number: usize) -> Option<&u16> {
        self.channels.get(number - 1)
    }
    pub fn get_all_channels(&self) -> [u16; 14] {
        self.channels
    }
}

#[derive(Debug)]
pub enum ParsingError {
    InvalidPacket,
    FailsChecksum,
}

impl Error for ParsingError {}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &ParsingError::InvalidPacket => write!(f, "Parsing Error: Packet not valid")?,
            &ParsingError::FailsChecksum => write!(
                f,
                "Parsing Error: Packet fails checksum and should not be used"
            )?,
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_correct_packet() {
        let data: [u8; 32] = [
            0x20, 0x40, 0xDB, 0x05, 0xDC, 0x05, 0x54, 0x05, 0xDC, 0x05, 0xE8, 0x03, 0xD0, 0x07,
            0xD2, 0x05, 0xE8, 0x03, 0xDC, 0x05, 0xDC, 0x05, 0xDC, 0x05, 0xDC, 0x05, 0xDC, 0x05,
            0xDC, 0x05, 0xDA, 0xF3,
        ];

        let packet = IbusPacket::try_from_bytes(&data).expect("Should be valid packet");

        assert_eq!(*packet.get_channel(3).unwrap(), 1364u16);
    }

    #[test]
    fn fails_invalid_packet() {
        let data: [u8; 32] = [0x02; 32];

        let packet = IbusPacket::try_from_bytes(&data);

        assert!(matches!(packet, Err(ParsingError::InvalidPacket)));
    }

    #[test]
    fn fails_bad_checksum() {
        let data: [u8; 32] = [
            0x20, 0x40, 0xDB, 0x05, 0xDC, 0x05, 0x54, 0x05, 0xDC, 0x05, 0xE8, 0x03, 0xD0, 0x07,
            0xD2, 0x05, 0xE8, 0x03, 0xDC, 0x05, 0xDC, 0x05, 0xDC, 0x05, 0xDC, 0x05, 0xDC, 0x05,
            0xDC, 0x05, 0xDA, 0xFF,
        ];

        let packet = IbusPacket::try_from_bytes(&data);

        assert!(matches!(packet, Err(ParsingError::FailsChecksum)));
    }
}
