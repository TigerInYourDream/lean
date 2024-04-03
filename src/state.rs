use std::marker::PhantomData;

struct State1;
struct State2;
struct State3;

pub struct StateMachine<'a, S: Max> {
    state: PhantomData<S>,
    common_info: &'a str,
}

trait Min {}

#[allow(private_bounds)]
pub trait Max: Min {}

impl Min for State1 {}
impl Min for State2 {}
impl Min for State3 {}
impl Max for State1 {}
impl Max for State2 {}
impl Max for State3 {}

impl<'a, S> StateMachine<'a, S>
where
    S: Max,
{
    pub fn new(common_info: &'a str) -> StateMachine<'a, S> {
        StateMachine {
            state: PhantomData,
            common_info,
        }
    }
}

impl<'a> StateMachine<'a, State1> {
    pub fn private1(&self) {
        println!("Private State1");
    }

    pub fn knife(&self) {
        println!("Knife");
    }
}

impl StateMachine<'_, State2> {
    pub fn private2(&self) {
        println!("Private State2");
    }

    pub fn gun(&self) {
        println!("GUN");
    }
}

impl StateMachine<'_, State3> {
    pub fn private3(&self) {
        println!("Private State3");
    }
    pub fn sword(&self) {
        println!("Sword");
    }
}

impl<'a> From<StateMachine<'a, State1>> for StateMachine<'a, State2> {
    fn from(s: StateMachine<'a, State1>) -> StateMachine<'a, State2> {
        StateMachine {
            state: PhantomData,
            common_info: s.common_info,
        }
    }
}

impl<'a> From<StateMachine<'a, State2>> for StateMachine<'a, State3> {
    fn from(s: StateMachine<'a, State2>) -> StateMachine<'a, State3> {
        StateMachine {
            state: PhantomData,
            common_info: s.common_info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine() {
        let state1 = StateMachine::<State1>::new("Common");
        state1.private1();
        state1.knife();

        let n2: StateMachine<State2> = state1.into();
        n2.private2();
        n2.gun();

        let n3: StateMachine<State3> = n2.into();
        n3.private3();
        n3.sword();
    }
}
