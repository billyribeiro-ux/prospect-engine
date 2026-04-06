use uuid::Uuid;

/// Generates a new time-ordered UUID (v7).
pub fn new_uuid_v7() -> Uuid {
	Uuid::now_v7()
}
