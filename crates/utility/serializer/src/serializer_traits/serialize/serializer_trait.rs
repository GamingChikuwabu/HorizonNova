pub trait Serialize {
    fn serialize<S>(&self,serializer:S)
    where
        S:Serializer;
}

pub trait SerializeValue {
    fn serialize(&self) -> String;
}


pub trait Serializer : Sized {
    fn serialize_bool(self, value:bool);
    fn serialize_i8(self, value:i8);
    fn serialize_i16(self, value:i16);
    fn serialize_i32(self, value:i32);
    fn serialize_i64(self, value:i64);
    fn serialize_u8(self, value:u8);
    fn serialize_u16(self, value:u16);
    fn serialize_u32(self, value:u32);
    fn serialize_u64(self, value:u64);
    fn serialize_f32(self, value:f32);
    fn serialize_f64(self, value:f64);
    fn serialize_char(self, value:char);
    fn serialize_str(self, value:&str);
    fn serialize_feild<T:SerializeValue>(&mut self, key:&str, value:T);
}