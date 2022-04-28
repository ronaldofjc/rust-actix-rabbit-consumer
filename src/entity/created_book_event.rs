use borsh_derive::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreatedBookEvent {
    pub id: String,
    pub title: String,
    pub author: String,
    pub pages: i32
}