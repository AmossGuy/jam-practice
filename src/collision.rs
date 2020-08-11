use collider::{HbId, HbProfile};

#[derive(Copy, Clone, Debug)]
pub struct MyHbProfile {
    pub id: HbId,
}

impl HbProfile for MyHbProfile {
    fn id(&self) -> u64 {
        self.id
    }

    fn can_interact(&self, _other: &MyHbProfile) -> bool {
        true
    }

    fn cell_width() -> f64 {
        8.
    }

    fn padding() -> f64 {
        0.01
    }
}
