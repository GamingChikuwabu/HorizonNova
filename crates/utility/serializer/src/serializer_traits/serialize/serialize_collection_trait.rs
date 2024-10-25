use crate::serializer_traits::serialize::serializer_trait::{Serialize,Serializer};

impl Serialize for i8 {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_i8(*self);
    }
}

impl Serialize for i16 {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_i16(*self);
    }
}

impl Serialize for i32 {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_i32(*self);
    }
}

impl Serialize for i64 {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_i64(*self);
    }
}

impl Serialize for u8 {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_u8(*self);
    }
}

impl Serialize for u16 {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_u16(*self);
    }
}

impl Serialize for u32 {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_u32(*self);
    }
}

impl Serialize for u64 {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_u64(*self);
    }
}

impl Serialize for f32 {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_f32(*self);
    }
}

impl Serialize for f64 {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_f64(*self);
    }
}

impl Serialize for char {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_char(*self);
    }
}

impl Serialize for &str {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_str(*self);
    }
}

impl Serialize for String {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_str(self);
    }
}

impl Serialize for bool {
    fn serialize<S:Serializer>(&self,serializer:S){
        serializer.serialize_bool(*self);
    }
}