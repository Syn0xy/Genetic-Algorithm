use crate::individual_manager::IndividualId;

pub trait Individual {
    fn id(&self) -> &IndividualId;
    fn fitness(&self) -> f32;
}
