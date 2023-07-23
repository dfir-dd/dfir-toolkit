pub trait HiveStatus {}

pub struct CleanHive;
pub struct DirtyHive;

impl HiveStatus for CleanHive {}
impl HiveStatus for DirtyHive {}