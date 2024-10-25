pub mod serializer_traits{
    pub mod serialize{
        pub mod serializer_trait;
        pub mod serialize_collection_trait;
    }
    pub mod deserialize{
        pub mod deserializer_trait;
    }
}

pub mod serializable_error;


pub mod serializer_members{
    pub mod json{
        pub mod json_serializer;
        pub mod json_formatter;
        pub mod json_writer;
        pub mod json_reader;
    }
    pub mod xml{
        pub mod xml_serializer;
    }
}