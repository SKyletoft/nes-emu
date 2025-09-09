use anyhow::bail;

pub struct NesFile {
	prg_roms: Vec<[u8; 16 * 1024]>,
	chr_roms: Vec<[u8; 8 * 1024]>,
}

impl TryFrom<Vec<u8>> for NesFile {
	type Error = anyhow::Error;

	fn try_from(buffer: Vec<u8>) -> Result<Self, Self::Error> {
		let [
			b'N',
			b'E',
			b'S',
			0x1A,
			prg_size,
			chr_size,
			flags_6,
			flags_7,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
			_,
		] = &buffer[0..16]
		else {
			bail!("Missing header!");
		};

		let trainer_present = flags_6 & 0x04 != 0;
		let trainer_offset = if trainer_present { 512 } else { 0 };
		let prg_offset = 16 + trainer_offset;
		let chr_offset = prg_offset + (*prg_size as usize * 16 * 1024);

		// Parse PRG ROM banks
		let mut prg_roms = Vec::new();
		for i in 0..*prg_size {
			let start = prg_offset + (i as usize * 16 * 1024);
			let end = start + (16 * 1024);

			if end > buffer.len() {
				bail!("PRG ROM data is too short");
			}

			let mut bank = [0u8; 16 * 1024];
			bank.copy_from_slice(&buffer[start..end]);
			prg_roms.push(bank);
		}

		// Parse CHR ROM banks
		let mut chr_roms = Vec::new();
		for i in 0..*chr_size {
			let start = chr_offset + (i as usize * 8 * 1024);
			let end = start + (8 * 1024);

			if end > buffer.len() {
				bail!("CHR ROM data is too short");
			}

			let mut bank = [0u8; 8 * 1024];
			bank.copy_from_slice(&buffer[start..end]);
			chr_roms.push(bank);
		}

		Ok(NesFile {
			prg_roms,
			chr_roms,
		})
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn load_smb3() {
		let buffer = std::fs::read("non-free/SMB3.nes").unwrap();
		NesFile::try_from(buffer).unwrap();
	}
}
