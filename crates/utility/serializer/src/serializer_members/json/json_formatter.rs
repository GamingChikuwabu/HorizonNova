use std::io;

use crate::serializable_error::SerializableError;

pub(crate) trait Formatter{
    fn begin_array<W:?Sized + io::Write>(&mut self,writer:W)->Result<(),SerializableError>;
    fn end_array(&self)->Result<(),SerializableError>;
    fn begin_object(&self)->Result<(),SerializableError>;
    fn end_object(&self)->Result<(),SerializableError>;
}

pub(crate) struct PrettyFormatter;

impl PrettyFormatter{
    pub fn new()->PrettyFormatter{
        PrettyFormatter{}
    }
}

impl Formatter for PrettyFormatter{
    fn begin_array<W:?Sized + io::Write>(&mut self,writer:W)->Result<(),SerializableError>{
        writer.write_all(b"[\n")?;
        Ok(())
    }

    fn end_array(&self)->Result<(),SerializableError>{
        Ok(())
    }

    fn begin_object(&self)->Result<(),SerializableError>{
        Ok(())
    }

    fn end_object(&self)->Result<(),SerializableError>{
        Ok(())
    }
    
}