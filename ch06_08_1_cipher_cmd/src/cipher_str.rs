use aes::Aes256;
use block_modes::{BlockMode, Cbc, block_padding::Pkcs7};
use sha2::{Sha256, Digest};

// 블록 암호의 종류와 암호 모드 지정
type AesCbc = Cbc<Aes256, Pkcs7>; // Aes256 라는 타입으로 만듦
const SALT: &str = "LFsMH#kL!IfY:dcEz9F/dvj17nUN"; // 임시 고정 값 사용

// password로 data를 암호화하는 함수
pub fn encrypt(password: &str, data: &str) -> String {
    // 패스워드를 고정 길이 기코 변환
    let key = get_key(password);
    let iv = get_iv(); // 초기 벡터 구하기

    // 암호화
    let cipher = AesCbc::new_from_slices(&key, &iv).unwrap();
    let result = cipher.encrypt_vec(data.as_bytes());

    // 암호화 결과 앞에 iv를 추가
    let mut ivres: Vec<u8> = vec![];
    ivres.extend(iv);
    ivres.extend(result);

    // base64 인코딩행 반환
    base64::encode(ivres)
}

// 초기화 벡터를 무작위로 생성
fn get_iv() -> Vec<u8> {
    let mut res: Vec<u8> = vec![0,0,0,0, 0,0,0,0, 0,0,0,0, 0,0,0,0];
    getrandom::getrandom(&mut res).unwrap();
    res
}

// 패스워드로부터 32 바이트 암호 키 얻기
fn get_key(password: &str) -> Vec<u8> {
    let pw: String = format!("{}::{}", password, SALT);
    let mut h = Sha256::new();
    h.update(pw.as_bytes());
    h.finalize().to_vec()
}

// 복호화 함수
pub fn decrypt(password: &str, data: &str) -> String {
    let key = get_key(password);
    let bytes = base64::decode(data).unwrap();

    // 데이터 앞에 있는 초기화 벡터 꺼내기
    let iv = &bytes[..16];

    // 복호화
    let cipher = AesCbc::new_from_slices(&key, iv).unwrap();
    let result = cipher.decrypt_vec(&bytes[16..]).unwrap();
    String::from_utf8(result).unwrap()
}

#[cfg(test)]
mod cipher_tests {
    use super::*;

    #[test]
    fn enc_dec_test() {
        let password = "wikibooks";
        let data = "IT 도서 전문 출판사 위키북스";
        let enc = encrypt(password, data);
        let dec = decrypt(password, &enc);
        assert_eq!(data, dec);
    }
}