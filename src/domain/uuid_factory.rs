use uuid::Uuid;

pub fn new_uuid() -> Uuid {
    if cfg!(test) {
        Uuid::nil()
    } else {
        Uuid::new_v4()
    }
}
