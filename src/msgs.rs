#[derive(Serialize, Deserialize)]
pub enum MsgTy {
    GetPubKey,
    PubKey(String),
}

#[derive(Serialize, Deserialize)]
pub struct Msg {
    ty: MsgTy,
}
