use image::{self, imageops, GenericImageView};

fn main() {
    // 리사이즈 후 크기
    let size = 128;

    // 명령줄 인수 얻기
    let args: Vec<String> = std::env::args().collect(); 
    if args.len() < 2 {
        println!("[USAGE] image_thumb imagefile");
        return;
    }

    // 입력 파일과 출력 파일을 지정
    let infile = String::from(&args[1]);
    let file_name: Vec<&str> = infile.split(".").collect();
    let outfile = format!("{}-thumb.png", file_name[0]);
    println!("input: {}", infile);
    println!("output: {}", outfile);

    // 이미지 파일 얻기
    let mut img = image::open(infile)
        .expect("파일을 읽어올 수 없습니다.");
    
    // 크기 얻기
    let dim = img.dimensions();

    // 정사각형 자르기
    let w = if dim.0 > dim.1 {dim.1} else {dim.0}; // 더 짧은 길이
    let mut img2 = imageops::crop(&mut img, (dim.0-w)/2, (dim.1-w)/2, w, w).to_image(); // (img, x, y, w, h)

    // 지정한 크기로 리사이즈
    let img3 = imageops::resize(&mut img2, size, size, imageops::Lanczos3); // Lanczos3필터 사용, 화질 좋지만 느림

    img3.save(outfile).unwrap();
}
