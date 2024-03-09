use crate::prisma::file_version;

file_version::select!(file_version_select {
	file: select {
		id
		extension
	}
	version_number
});

// pub type FileVersion = file_version::Data;
pub type FileVersionSelect = file_version_select::Data;
