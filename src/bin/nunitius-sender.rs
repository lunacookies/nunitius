use jsonl::Connection;
use nunitius::sender::ui;
use nunitius::{Color, ConnectionKind, LoginResponse, Message, SenderEvent, User};
use std::net::TcpStream;
use std::{io, thread};

type TcpConnection = Connection<io::BufReader<TcpStream>, TcpStream>;

fn main() -> anyhow::Result<()> {
    let mut stdout = io::stdout();

    let stream = TcpStream::connect("127.0.0.1:9999")?;
    let mut connection = Connection::new_from_tcp_stream(stream)?;

    connection.write(&ConnectionKind::Sender)?;

    let user = login(&mut stdout, &mut connection)?;

    let (typing_event_tx, typing_event_rx) = flume::bounded(100);
    let (sender_event_tx, sender_event_rx) = flume::bounded(100);

    thread::spawn({
        let sender_event_tx = sender_event_tx.clone();
        let user = user.clone();

        move || {
            for typing_event in typing_event_rx {
                sender_event_tx
                    .send(SenderEvent::Typing {
                        event: typing_event,
                        user: user.clone(),
                    })
                    .unwrap();
            }
        }
    });

    thread::spawn(move || {
        for sender_event in sender_event_rx {
            connection.write(&sender_event).unwrap();
        }
    });

    loop {
        let input = ui::read_input_evented("Type a message", &mut stdout, typing_event_tx.clone())?;

        let input = if let Some(i) = input {
            i
        } else {
            continue;
        };

        let message = Message {
            body: input,
            author: user.clone(),
        };

        sender_event_tx.send(SenderEvent::Message(message)).unwrap();
    }
}

fn login(stdout: &mut io::Stdout, connection: &mut TcpConnection) -> anyhow::Result<User> {
    loop {
        let nickname = ui::read_input("Choose a nickname", stdout)?;

        let nickname = if let Some(n) = nickname {
            n
        } else {
            continue;
        };

        let user = User {
            nickname: nickname.clone(),
            color: read_color(stdout)?,
        };

        connection.write(&user)?;

        let response: LoginResponse = connection.read()?;

        if response.nickname_taken {
            eprintln!("Nickname ‘{}’ taken. Try another one.", nickname);
        } else {
            return Ok(user);
        }
    }
}

fn read_color(stdout: &mut io::Stdout) -> anyhow::Result<Option<Color>> {
    loop {
        let color = if let Some(s) = ui::read_input("Choose a color", stdout)? {
            s
        } else {
            return Ok(None);
        };

        let color = match color.as_str() {
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            _ => {
                eprintln!("‘{}’ is an invalid color.", color);
                continue;
            }
        };

        return Ok(Some(color));
    }
}
