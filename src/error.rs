use soroban_sdk::{contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    FaucetNotInit = 0,
    NoAdmin = 1,
    FaucetAlreadyInit = 2,
    FaucetClosed = 3,
    NoSignups = 4,
    SignupsFull = 5,
    InvalidSignup = 6,
    NoCrossContract = 7,
    Unauthorized = 8,
    NoSuchMember = 9,
    DisburseRequired = 10,
}