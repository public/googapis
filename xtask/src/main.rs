use std::{env, fs, path::PathBuf};

mod gen;

fn main() {
    match env::args().nth(1) {
        Some(cmd) => match cmd.as_str() {
            "gen" => gen(),
            _ => print_help(),
        },
        _ => print_help(),
    }
}

fn print_help() {
    println!(r#"cargo xtask gen"#)
}

fn gen() {
    let proto_root = PathBuf::from("xtask/proto/googleapis");
    let protos = gen::find_proto(proto_root.clone());

    // let gates = gen::feature_gates(&protos);
    // println!("{}", gates);

    let out_dir = PathBuf::from("googapis/genproto");
    let _ = fs::remove_dir_all(out_dir.as_path());
    let _ = fs::create_dir(out_dir.as_path());
    tonic_build::configure()
        .out_dir(out_dir)
        .compile(&gen::proto_path(&protos), &[proto_root])
        .unwrap();

    let mut out_path = PathBuf::from("googapis/src/googapis.rs");
    let root = gen::from_protos(protos);
    fs::write(out_path.clone(), root.gen_code()).unwrap();

    out_path.pop();
    tonic_build::fmt(out_path.to_str().unwrap());
}
