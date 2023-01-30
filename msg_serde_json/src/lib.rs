pub trait MsgSerdeJson<T> {
    fn from_serde_json_buf(buf: &[u8]) -> std::option::Option<T>;
    fn to_serde_json_buf(msg: &T) -> std::option::Option<Vec<u8>>;
}
