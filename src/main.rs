use std::fs;
use std::time;

fn main() {
    let mut src_path;
	let mut dst_path1;
	let mut dst_path2;
	let mut try_count;
	let sleep_time = time::Duration::from_millis(500);

	println!("whazzup_helper is running...");
	try_count = 0;
	src_path = dirs::home_dir().expect("ホームディレクトリの取得に失敗しました");
	src_path.push("Appdata/Local/JoinFS-FS2020/whazzup.txt");
	dst_path1 = dirs::home_dir().expect("ホームディレクトリの取得に失敗しました");
	dst_path1.push("Appdata/Local/JoinFS/whazzup.txt");
	dst_path2 = dirs::home_dir().unwrap();
	dst_path2.push("Documents/JoinFS/whazzup.txt");
	loop {
		let result = fs::copy(&src_path, &dst_path1);
		let _result = match result {
			Ok(count) => count,
			Err(_error) => {
				try_count += 1;
				println!("failed to copy file. but we are continue. {}", try_count);
				std::thread::sleep(sleep_time);
				continue;
			},
		};
		let result = fs::copy(&src_path, &dst_path2);
		let _result = match result {
			Ok(count) => count,
			Err(_error) => {
				try_count += 1;
				println!("failed to copy file. but we are continue. {}", try_count);
				std::thread::sleep(sleep_time);
				continue;
			},
		};
		std::thread::sleep(sleep_time);
	}
}