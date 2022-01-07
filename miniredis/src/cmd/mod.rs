#[derive(Debug)]
pub enum Command {
    Get,
    Publish,
    Set,
    Subscribe,
    UbSubscribe,
    Unknown
}
