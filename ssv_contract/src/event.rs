//
//
// pub struct OperatorRegistration {
//     name: String,
//     owner_address: String,
//     public_key: String,
//     id: u32,
//     fee: u64,
// }
//
//
// pub struct OperatorRemoval {
//     id: u32,
//     owner_address: String,
// }
//
//
// pub enum Content {
//     OperatorRegistration(OperatorRegistration),
//     OperatorRemoval(OperatorRemoval)
// }
//
//
// pub struct Event {
//     pub block_number: u64,
//     pub content: Content
// }