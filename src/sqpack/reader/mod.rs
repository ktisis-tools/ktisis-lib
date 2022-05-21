pub mod chunk;

use crate::sqpack::{SqPack, SqPackChunk};

use chunk::ChunkReader;

use std::collections::HashMap;

// SqPackRead

pub struct SqPackReader {
	pub chunks: HashMap<u32, ChunkReader>
}

impl SqPackReader {
	pub fn new(pack: SqPack) -> SqPackReader {
		return SqPackReader {
			chunks: HashMap::<u32, ChunkReader>::new()
		};
	}
}