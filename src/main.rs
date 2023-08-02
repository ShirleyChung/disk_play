fn main() {
    let ss = 512; // sector size
    let data = vec![0; 100 * ss as usize];
    let mut cur = std::io::Cursor::new(data);

    let mut mbr = mbrman::MBR::new_from(&mut cur, ss as u32, [0xff; 4])
        .expect("could not create partition table");
    let _ = mbr.write_into(&mut cur);
    println!("Hello, world!");
}
