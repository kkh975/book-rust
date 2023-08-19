#[tokio::main]
async fn main() {
    // 비동기 함수 준비
    let f = say_later("포기에도 때가 있다.");

    // 메시지 표시
    println!("아무 때나 포기하지 마라!");

    // 비동기 처리 실행 완료
    f.await;
}

async fn say_later(msg: &'static str) {
    println!("{}", msg);
}
