use anyhow::bail;

struct NesFile {}

impl TryFrom<Vec<u8>> for NesFile {
	type Error = anyhow::Error;

	fn try_from(buffer: Vec<u8>) -> Result<Self, Self::Error> {
		if buffer[..4] != [b'N', b'E', b'S', 0x1A] {
			bail!("Missing header!");
		}

		todo!()
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn load_smb3() {
	}
}
