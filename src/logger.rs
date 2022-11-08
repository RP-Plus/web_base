use env_logger::{Builder, Env, Target};
use std::fs::File;

pub fn init_log_to_file(file_name: &str) {

	let env = Env::default();
    let f = File::options()
                    .create(true)
                    .append(true)
                    .open(file_name)
                    .unwrap();

    Builder::from_env(env)
        .target(Target::Pipe(Box::new(f)))
        .init();

}

pub fn init_log() {

	env_logger::init();
	
}