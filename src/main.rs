use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cube{
    #[arg(short, long)]
    x: f64,
    #[arg(short, long)]
    y: f64,
    #[arg(short, long)]
    z: f64,

}

fn main() {
    let dimensions = Cube::parse();
    let volume = dimensions.x * dimensions.y * dimensions.z;
    println!("Volume of the cube is: {}", volume);
}