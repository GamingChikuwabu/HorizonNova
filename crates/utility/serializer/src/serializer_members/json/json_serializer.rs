use crate::serializer_traits::serialize::serializer_trait::{Serialize,Serializer,SerializeStruct};
use crate::serializable_error::SerializableError;

struct Serializer<W>{
    W:Writer
    
}



fn to_writer()
{

}

pub fn to_string<T:Serialize>(value:&T)->Result<String,SerializableError>{
    Ok("".to_string())
}