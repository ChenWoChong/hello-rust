use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Eq, PartialEq, Default, Debug)]
pub struct Consumer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

pub trait Free {
    #[allow(dead_code)]
    fn feature1(&self);
    #[allow(dead_code)]
    fn feature2(&self);
}

pub trait Personal: Free {
    #[allow(dead_code)]
    fn advance_feature(&self);
}

pub struct FreePlan;
#[allow(dead_code)]
pub struct PersonalPlan(f32);

impl<T> Free for Consumer<T> {
    fn feature1(&self) {
        println!("feature 1 for {}", self.name);
    }

    fn feature2(&self) {
        println!("feature 2 for {}", self.name);
    }
}

impl<T> Consumer<T> {
    fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

impl Personal for Consumer<PersonalPlan> {
    fn advance_feature(&self) {
        println!(
            "Dear {}(as our valuable customer {}), enjoy this advanced feature!",
            self.name, self.id
        );
    }
}

impl From<Consumer<FreePlan>> for Consumer<PersonalPlan> {
    fn from(value: Consumer<FreePlan>) -> Self {
        Self::new(value.name)
    }
}

#[allow(dead_code)]
pub fn subscribe(consumer: Consumer<FreePlan>, payment: f32) -> Consumer<PersonalPlan> {
    let plan = PersonalPlan(payment);
    println!(
        "upgrade consumer {} to personal, pay money is {}",
        consumer.id, plan.0
    );
    consumer.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn consumer_should_work() {
        let free_consumer = Consumer::<FreePlan>::new("chen".into());
        free_consumer.feature1();
        free_consumer.feature2();
        let personal_consumer = subscribe(free_consumer, 9.99);
        personal_consumer.advance_feature();
    }
}
