#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ElfError {
    InvalidMagic,
    UnsupportedClass,
    UnsupportedEndian,
    UnsupportedVersion,
    UnsupportedType,
    InvalidProgramHeader,
    Truncated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ElfProgramHeader {
    pub file_offset: u64,
    pub virtual_address: u64,
    pub file_size: u64,
    pub memory_size: u64,
    pub flags: u32,
    pub alignment: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ElfImage {
    pub entry_point: u64,
    pub segment_count: usize,
    pub segments: [ElfProgramHeader; 16],
}

impl ElfImage {
    pub fn iter_segments(&self) -> &[ElfProgramHeader] {
        &self.segments[..self.segment_count]
    }
}

pub fn parse_elf64(image: &[u8]) -> Result<ElfImage, ElfError> {
    if image.len() < 64 {
        return Err(ElfError::Truncated);
    }

    if &image[0..4] != [0x7F, b'E', b'L', b'F'] {
        return Err(ElfError::InvalidMagic);
    }

    if image[4] != 2 {
        return Err(ElfError::UnsupportedClass);
    }

    if image[5] != 1 {
        return Err(ElfError::UnsupportedEndian);
    }

    if image[6] != 1 {
        return Err(ElfError::UnsupportedVersion);
    }

    let file_type = u16::from_le_bytes([image[16], image[17]]);
    if !matches!(file_type, 2 | 3) {
        return Err(ElfError::UnsupportedType);
    }

    let entry_point = u64::from_le_bytes(
        image[24..32]
            .try_into()
            .map_err(|_| ElfError::Truncated)?,
    );

    let program_header_offset = u64::from_le_bytes(
        image[32..40]
            .try_into()
            .map_err(|_| ElfError::Truncated)?,
    );
    let program_header_size = u16::from_le_bytes([image[54], image[55]]) as usize;
    let program_header_count = u16::from_le_bytes([image[56], image[57]]) as usize;

    if program_header_size == 0 || program_header_count == 0 {
        return Err(ElfError::InvalidProgramHeader);
    }

    if program_header_count > 16 {
        return Err(ElfError::InvalidProgramHeader);
    }

    let mut segments = [ElfProgramHeader {
        file_offset: 0,
        virtual_address: 0,
        file_size: 0,
        memory_size: 0,
        flags: 0,
        alignment: 0,
    }; 16];
    let mut segment_count = 0usize;

    for index in 0..program_header_count {
        let offset = program_header_offset as usize + index * program_header_size;
        if offset + program_header_size > image.len() {
            return Err(ElfError::Truncated);
        }

        let entry_type = u32::from_le_bytes([
            image[offset],
            image[offset + 1],
            image[offset + 2],
            image[offset + 3],
        ]);

        if entry_type != 1 {
            continue;
        }

        let file_offset = u64::from_le_bytes(
            image[offset + 8..offset + 16]
                .try_into()
                .map_err(|_| ElfError::Truncated)?,
        );
        let virtual_address = u64::from_le_bytes(
            image[offset + 16..offset + 24]
                .try_into()
                .map_err(|_| ElfError::Truncated)?,
        );
        let file_size = u64::from_le_bytes(
            image[offset + 32..offset + 40]
                .try_into()
                .map_err(|_| ElfError::Truncated)?,
        );
        let memory_size = u64::from_le_bytes(
            image[offset + 40..offset + 48]
                .try_into()
                .map_err(|_| ElfError::Truncated)?,
        );
        let flags = u32::from_le_bytes([
            image[offset + 4],
            image[offset + 5],
            image[offset + 6],
            image[offset + 7],
        ]);
        let alignment = u64::from_le_bytes(
            image[offset + 48..offset + 56]
                .try_into()
                .map_err(|_| ElfError::Truncated)?,
        );

        if file_offset + file_size > image.len() as u64 {
            return Err(ElfError::Truncated);
        }

        segments[segment_count] = ElfProgramHeader {
            file_offset,
            virtual_address,
            file_size,
            memory_size,
            flags,
            alignment,
        };
        segment_count += 1;
    }

    if segment_count == 0 {
        return Err(ElfError::InvalidProgramHeader);
    }

    Ok(ElfImage {
        entry_point,
        segment_count,
        segments,
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_elf64, ElfError};

    fn make_valid_elf64() -> Vec<u8> {
        let mut bytes = vec![0u8; 64 + 56];
        bytes[0..4].copy_from_slice(&[0x7F, b'E', b'L', b'F']);
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[6] = 1;
        bytes[7] = 0;
        bytes[8..16].copy_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
        bytes[16..18].copy_from_slice(&2u16.to_le_bytes());
        bytes[18..20].copy_from_slice(&1u16.to_le_bytes());
        bytes[20..22].copy_from_slice(&1u16.to_le_bytes());
        bytes[22..24].copy_from_slice(&0u16.to_le_bytes());
        bytes[24..32].copy_from_slice(&(0x400_000u64).to_le_bytes());
        bytes[32..40].copy_from_slice(&(64u64).to_le_bytes());
        bytes[40..48].copy_from_slice(&0u64.to_le_bytes());
        bytes[48..52].copy_from_slice(&0u32.to_le_bytes());
        bytes[52..54].copy_from_slice(&0u16.to_le_bytes());
        bytes[54..56].copy_from_slice(&56u16.to_le_bytes());
        bytes[56..58].copy_from_slice(&1u16.to_le_bytes());

        let load = 64u64;
        bytes[64..120].copy_from_slice(&[0u8; 56]);
        bytes[64..68].copy_from_slice(&1u32.to_le_bytes());
        bytes[68..72].copy_from_slice(&5u32.to_le_bytes());
        bytes[72..80].copy_from_slice(&load.to_le_bytes());
        bytes[80..88].copy_from_slice(&(0x400_000u64).to_le_bytes());
        bytes[88..96].copy_from_slice(&(0x100u64).to_le_bytes());
        bytes[96..104].copy_from_slice(&(0x200u64).to_le_bytes());
        bytes[104..112].copy_from_slice(&0u64.to_le_bytes());
        bytes[112..120].copy_from_slice(&(0x1000u64).to_le_bytes());
        bytes.resize(64 + 56 + 0x100, 0);
        bytes
    }

    #[test]
    fn parses_valid_elf64_executable() {
        let image = make_valid_elf64();
        let parsed = parse_elf64(&image).unwrap();
        assert_eq!(parsed.entry_point, 0x400_000);
        assert_eq!(parsed.segment_count, 1);
        assert_eq!(parsed.segments[0].virtual_address, 0x400_000);
    }

    #[test]
    fn rejects_invalid_elf_header() {
        let result = parse_elf64(&[0, 1, 2, 3]);
        assert_eq!(result, Err(ElfError::InvalidMagic));
    }
}
