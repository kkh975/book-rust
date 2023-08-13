use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 1초마다 메시지를 보내는 함수
fn sleep_sender(name: &str, sender: mpsc::Sender<String>) {
    let whales = ["큰고래", "흑등고래", "향유고래", "남방큰돌고래", "북극고래"];
    for whale in whales {
        let msg = format!("{}: {}", name, whale);
        sender.send(msg).unwrap(); // 송신
        thread::sleep(Duration::from_millis(1000));
    }
    sender.send("quit".to_string()).unwrap();
}

fn main() {
    // 스레드 간 통신용 채널
    let (tx, rx) = mpsc::channel::<String>();

    // 스레드1 생성
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("우영우", sender)
    });

    // 스레드2 생성
    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("이준호", sender)
    });

    // 스레드로부터 메시지를 반복해서 받음
    loop {
        let buf = rx.recv().unwrap();
        println!("[수신] {}", buf);
        if buf == "quit" { break; }
    }
}
