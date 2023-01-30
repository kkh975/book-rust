use image::{GenericImage, GenericImageView, Rgba};

fn main() {
    // 명령줄 인수 얻기
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] image_filter imagefile");
        return;
    }

    let infile = args[1].clone();
    let file_name: Vec<&str> = infile.split(".").collect();
    let outfile = format!("{}-out.jpg", file_name[0]);
    println!("input: {}", infile);
    println!("output: {}", outfile);

    // 이미지 읽기
    let mut img = image::open(infile).expect("파일을 읽어올 수 없습니다.");
    let (w, h) = img.dimensions(); // 폭x높이

    // 행과 열을 반복 
    for y in 0..h {
        for x in 0..w {
            let c: Rgba<u8> = img.get_pixel(x, y);
            let c = Rgba([
                255 - c[0],
                255 - c[1],
                255 - c[2],
                c[3]
            ]);
            img.put_pixel(x, y, c);
        }
    }

    img.save(outfile).unwrap();
}
