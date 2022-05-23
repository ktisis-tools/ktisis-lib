use binread::BinRead;

// ExhHeader (.exh) file definitions

#[derive(BinRead)]
#[derive(Debug)]
pub struct ExcelColumnDefinition {
	data_type: u16,
	offset: u16
}

#[derive(BinRead)]
#[derive(Debug)]
pub struct ExcelPageDefinition {
	start_id: u32,
	row_count: u32
}