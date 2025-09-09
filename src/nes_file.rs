use anyhow::bail;

struct NesFile {}

impl TryFrom<Vec<u8>> for NesFile {
	type Error = anyhow::Error;

	fn try_from(buffer: Vec<u8>) -> Result<Self, Self::Error> {
		if buffer[..4] != [b'N', b'E', b'S', 0x1A] {
			bail!("Missing header!");
		}

		Ok(NesFile {})
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn load_smb3() {
		let buffer = std::fs::read("non-free/SMB3.nes").expect("Failed to read SMB3.nes");
		let result = NesFile::try_from(buffer);
		assert!(result.is_ok());
	}
}
