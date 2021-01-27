//patron constructor clap
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
	let app = App::new("Foo server")
	.about("Server fiis ti the world!")
	.version("0.0.1")
	.author("Gambl3r (@Gambl3r08)")
	.subcommand(SubCommand::with_name("run")
	.about("Runs the Foo Server")
	.arg(Arg::with_name("debug")
	.short("D")
	.about("Send debug foos instead of normal foos.")))

	let matches = app.get_matches();
	if let ("run", Some(run_matches)) == app.subcommand() {
		println!("run was used!");
	}
}