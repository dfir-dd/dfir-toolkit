use std::fmt::Display;

pub trait EvtxFieldView: Display {
    fn maximum_display_length(&self) -> usize { 0 }
    fn value_with_padding(&self) -> String { "".to_owned() } 
}