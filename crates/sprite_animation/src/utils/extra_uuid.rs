use bevy::utils::Uuid;

pub trait ExtraUuidUtil {
    fn from_object<T>(object: &T) -> Uuid;
}
impl ExtraUuidUtil for Uuid {
    fn from_object<T>(object: &T) -> Uuid {
        let ptr = object as *const T;
        Uuid::from_u128(ptr as u128)
    }
}

pub trait ToUuid {
    fn new_uuid(&self) -> Uuid {
        Uuid::new_v4()
    }
}
