pub mod tns {
    pub type Sdl = RootType;
    #[derive(Debug)]
    pub struct RootType {
        pub container: ContainerType,
    }
    #[derive(Debug)]
    pub struct ContainerType {
        pub content: ::std::vec::Vec<ContainerTypeContent>,
    }
    #[derive(Debug)]
    pub enum ContainerTypeContent {
        Known(KnownType),
        Text(::xsd_parser::xml::Text),
    }
    #[derive(Debug)]
    pub struct KnownType {
        pub name: ::core::option::Option<::std::string::String>,
    }
}
