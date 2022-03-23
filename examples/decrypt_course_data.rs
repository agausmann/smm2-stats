use smm2_stats::course_decryptor::decrypt_course_data;

fn main() {
    let mut args = std::env::args_os().skip(1);
    let infile = args.next().expect("missing argument [infile]");
    let outfile = args.next().expect("missing argument [outfile]");
    let encrypted = std::fs::read(infile).expect("cannot read input file");
    let decrypted = decrypt_course_data(&encrypted);
    std::fs::write(outfile, &decrypted).expect("cannot write output file");
}
