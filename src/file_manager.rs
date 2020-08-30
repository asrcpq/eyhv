pub struct FileManager {
	eyhv_data_dir: String,
}

impl FileManager {
	pub fn new() -> FileManager {
		let eyhv_data_dir: String = match std::env::var("XDG_DATA_HOME") {
			Ok(val) => val + "/eyhv",
			_ => match std::env::var("HOME") {
				Ok(val) => val + "/.local/share/eyhv",
				_ => panic!("Cannot save replay file"),
			},
		};
		std::fs::create_dir_all(eyhv_data_dir.clone()).unwrap();
		FileManager { eyhv_data_dir }
	}

	pub fn get_replay_path(&self) -> String {
		return self.eyhv_data_dir.clone() + "/latest_replay";
	}
}
