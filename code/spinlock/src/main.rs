use crate::channel::Channel;
use crate::spinlock::SpinLock;
use std::thread;

mod channel;
mod spinlock;
mod one_shot_channel;
mod arc;
mod mutex;

#[test]
fn using_spinlock() {
    let synced_vec = SpinLock::new(Vec::new());

    thread::scope(|s| {
        s.spawn(|| synced_vec.lock().push(1));
        s.spawn(|| {
            let mut g = synced_vec.lock();
            g.push(2);
            g.push(3);
        });
    });

    let g = synced_vec.lock();
    assert!(g.as_slice() == &[1, 2, 3] || g.as_slice() == &[2, 3, 1]);
}

fn consumer(id: i32, channel: &Channel<Option<String>>) {
    loop {
        if let Some(msg) = channel.receive() {
            println!("Consumer#{id} received: {}", msg);
        } else {
            println!("Consumer#{id} disconnected");
            break;
        }
    }
}

#[test]
fn using_channel() {
    let mut channel: Channel<Option<String>> = Channel::new();

    thread::scope(|s| {
        s.spawn(|| {
            consumer(1, &channel);
        });
        s.spawn(|| {
            consumer(2, &channel);
        });

        for i in 1..=10 {
            let msg: String = format!("Message #{i}");
            channel.send(Some(msg.clone()));
        }

        channel.send(None);
        channel.send(None);
    });
}

fn main() {
    println!("Spinlock!");
    println!("Channel");
}
