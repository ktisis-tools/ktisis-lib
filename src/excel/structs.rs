use binread::BinRead;

// ExhHeader (.exh) file definitions

#[derive(BinRead)]
#[derive(Debug)]
pub struct ExcelColumnDefinition {
	pub data_type: u16,
	pub offset: u16
}

#[derive(BinRead)]
#[derive(Debug)]
pub struct ExcelPageDefinition {
	pub start_id: u32,
	pub row_count: u32
}