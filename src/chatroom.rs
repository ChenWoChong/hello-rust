#[derive(Debug)]
enum Gender {
    Unknown = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserID(u64);

#[derive(Debug, Copy, Clone)]
struct TopicID(u64);

#[derive(Debug)]
struct User {
    id: UserID,
    name: String,
    gender: Gender,
}

struct Topic {
    id: TopicID,
    name: String,
    owner: UserID,
}

#[derive(Debug)]
enum Event {
    Join((UserID, TopicID)),
    Leave((UserID, TopicID)),
    Message((UserID, TopicID, String)),
}

pub fn chat() {
    let wochong = User {
        id: UserID(1),
        name: "wochong".to_string(),
        gender: Gender::Male,
    };
    let alice = User {
        id: UserID(2),
        name: "alice".into(),
        gender: Gender::Female,
    };

    let topic = Topic {
        id: TopicID(1),
        name: "rust".into(),
        owner: wochong.id,
    };

    let event1 = Event::Join((wochong.id, topic.id));
    let event2 = Event::Join((alice.id, topic.id));
    let event3 = Event::Message((wochong.id, topic.id, "hello world".into()));

    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );
}

#[cfg(test)]
mod test {}
