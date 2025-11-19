pub const NS_XS: xsd_parser::models::schema::Namespace =
    xsd_parser::models::schema::Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: xsd_parser::models::schema::Namespace =
    xsd_parser::models::schema::Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: xsd_parser::models::schema::Namespace =
    xsd_parser::models::schema::Namespace::new_const(b"http://example.com");
pub mod tns {
    use xsd_parser::xml::AnyElement;
    pub type Sdl = RootType;
    #[derive(Debug)]
    pub struct RootType {
        pub container: ContainerType,
    }
    impl ::xsd_parser::quick_xml::WithSerializer for RootType {
        type Serializer<'x> = quick_xml_serialize::RootTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: ::core::option::Option<&'ser str>,
            is_root: bool,
        ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser::quick_xml::Error>
        {
            Ok(quick_xml_serialize::RootTypeSerializer {
                value: self,
                state: ::std::boxed::Box::new(quick_xml_serialize::RootTypeSerializerState::Init__),
                name: name.unwrap_or("tns:RootType"),
                is_root,
            })
        }
    }
    impl ::xsd_parser::quick_xml::WithDeserializer for RootType {
        type Deserializer = quick_xml_deserialize::RootTypeDeserializer;
    }
    #[derive(Debug)]
    pub struct ContainerType {
        pub content: ::std::vec::Vec<ContainerTypeContent>,
    }
    #[derive(Debug)]
    pub enum ContainerTypeContent {
        Known(KnownType),
        Any(AnyElement),
        Text(::xsd_parser::xml::Text),
    }
    impl ::xsd_parser::quick_xml::WithSerializer for ContainerType {
        type Serializer<'x> = quick_xml_serialize::ContainerTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: ::core::option::Option<&'ser str>,
            is_root: bool,
        ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser::quick_xml::Error>
        {
            Ok(quick_xml_serialize::ContainerTypeSerializer {
                value: self,
                state: ::std::boxed::Box::new(
                    quick_xml_serialize::ContainerTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("tns:ContainerType"),
                is_root,
            })
        }
    }
    impl ::xsd_parser::quick_xml::WithSerializer for ContainerTypeContent {
        type Serializer<'x> = quick_xml_serialize::ContainerTypeContentSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: ::core::option::Option<&'ser str>,
            is_root: bool,
        ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser::quick_xml::Error>
        {
            let _name = name;
            let _is_root = is_root;
            Ok(quick_xml_serialize::ContainerTypeContentSerializer {
                value: self,
                state: ::std::boxed::Box::new(
                    quick_xml_serialize::ContainerTypeContentSerializerState::Init__,
                ),
            })
        }
    }
    impl ::xsd_parser::quick_xml::WithDeserializer for ContainerType {
        type Deserializer = quick_xml_deserialize::ContainerTypeDeserializer;
    }
    impl ::xsd_parser::quick_xml::WithDeserializer for ContainerTypeContent {
        type Deserializer = quick_xml_deserialize::ContainerTypeContentDeserializer;
    }
    #[derive(Debug)]
    pub struct KnownType {
        pub name: ::core::option::Option<::std::string::String>,
    }
    impl ::xsd_parser::quick_xml::WithSerializer for KnownType {
        type Serializer<'x> = quick_xml_serialize::KnownTypeSerializer<'x>;
        fn serializer<'ser>(
            &'ser self,
            name: ::core::option::Option<&'ser str>,
            is_root: bool,
        ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser::quick_xml::Error>
        {
            Ok(quick_xml_serialize::KnownTypeSerializer {
                value: self,
                state: ::std::boxed::Box::new(
                    quick_xml_serialize::KnownTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("tns:KnownType"),
                is_root,
            })
        }
    }
    impl ::xsd_parser::quick_xml::WithDeserializer for KnownType {
        type Deserializer = quick_xml_deserialize::KnownTypeDeserializer;
    }
    pub mod quick_xml_deserialize {
        use xsd_parser::{quick_xml::Deserializer as _, xml::AnyElement};
        #[derive(Debug)]
        pub struct RootTypeDeserializer {
            container: ::core::option::Option<super::ContainerType>,
            state__: ::std::boxed::Box<RootTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum RootTypeDeserializerState {
            Init__ , Container (:: core :: option :: Option << super :: ContainerType as :: xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
        impl RootTypeDeserializer {
            fn from_bytes_start<R>(
                reader: &R,
                bytes_start: &::xsd_parser::quick_xml::BytesStart<'_>,
            ) -> ::core::result::Result<Self, ::xsd_parser::quick_xml::Error>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                for attrib in ::xsd_parser::quick_xml::filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
                Ok(Self {
                    container: None,
                    state__: ::std::boxed::Box::new(RootTypeDeserializerState::Init__),
                })
            }
            fn finish_state<R>(
                &mut self,
                reader: &R,
                state: RootTypeDeserializerState,
            ) -> ::core::result::Result<(), ::xsd_parser::quick_xml::Error>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                use RootTypeDeserializerState as S;
                match state {
                    S::Container(Some(deserializer)) => {
                        self.store_container(deserializer.finish(reader)?)?
                    }
                    _ => (),
                }
                Ok(())
            }
            fn store_container(
                &mut self,
                value: super::ContainerType,
            ) -> ::core::result::Result<(), ::xsd_parser::quick_xml::Error> {
                if self.container.is_some() {
                    Err(::xsd_parser::quick_xml::ErrorKind::DuplicateElement(
                        ::xsd_parser::quick_xml::RawByteStr::from_slice(b"Container"),
                    ))?;
                }
                self.container = Some(value);
                Ok(())
            }
            fn handle_container<'de, R>(
                &mut self,
                reader: &R,
                output: ::xsd_parser::quick_xml::DeserializerOutput<'de, super::ContainerType>,
                fallback: &mut ::core::option::Option<RootTypeDeserializerState>,
            ) -> ::core::result::Result<
                ::xsd_parser::quick_xml::ElementHandlerOutput<'de>,
                ::xsd_parser::quick_xml::Error,
            >
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                let ::xsd_parser::quick_xml::DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    if self.container.is_some() {
                        fallback.get_or_insert(RootTypeDeserializerState::Container(None));
                        *self.state__ = RootTypeDeserializerState::Done__;
                        return Ok(::xsd_parser::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ));
                    } else {
                        *self.state__ = RootTypeDeserializerState::Container(None);
                        return Ok(::xsd_parser::quick_xml::ElementHandlerOutput::break_(
                            event, allow_any,
                        ));
                    }
                }
                if let Some(fallback) = fallback.take() {
                    self.finish_state(reader, fallback)?;
                }
                Ok(match artifact {
                    ::xsd_parser::quick_xml::DeserializerArtifact::None => unreachable!(),
                    ::xsd_parser::quick_xml::DeserializerArtifact::Data(data) => {
                        self.store_container(data)?;
                        *self.state__ = RootTypeDeserializerState::Done__;
                        ::xsd_parser::quick_xml::ElementHandlerOutput::from_event(event, allow_any)
                    }
                    ::xsd_parser::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                        let ret = ::xsd_parser::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        );
                        match &ret {
                            ::xsd_parser::quick_xml::ElementHandlerOutput::Continue { .. } => {
                                fallback.get_or_insert(RootTypeDeserializerState::Container(Some(
                                    deserializer,
                                )));
                                *self.state__ = RootTypeDeserializerState::Done__;
                            }
                            ::xsd_parser::quick_xml::ElementHandlerOutput::Break { .. } => {
                                *self.state__ =
                                    RootTypeDeserializerState::Container(Some(deserializer));
                            }
                        }
                        ret
                    }
                })
            }
        }
        impl<'de> ::xsd_parser::quick_xml::Deserializer<'de, super::RootType> for RootTypeDeserializer {
            fn init<R>(
                reader: &R,
                event: ::xsd_parser::quick_xml::Event<'de>,
            ) -> ::xsd_parser::quick_xml::DeserializerResult<'de, super::RootType>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next<R>(
                mut self,
                reader: &R,
                event: ::xsd_parser::quick_xml::Event<'de>,
            ) -> ::xsd_parser::quick_xml::DeserializerResult<'de, super::RootType>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                use RootTypeDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let mut allow_any_element = false;
                let (event, allow_any) = loop {
                    let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::Container(Some(deserializer)), event) => {
                            let output = deserializer.next(reader, event)?;
                            match self.handle_container(reader, output, &mut fallback)? {
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                    event,
                                    allow_any,
                                } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Break {
                                    event,
                                    allow_any,
                                } => break (event, allow_any),
                            }
                        }
                        (_, ::xsd_parser::quick_xml::Event::End(_)) => {
                            if let Some(fallback) = fallback.take() {
                                self.finish_state(reader, fallback)?;
                            }
                            return Ok(::xsd_parser::quick_xml::DeserializerOutput {
                                artifact: ::xsd_parser::quick_xml::DeserializerArtifact::Data(
                                    self.finish(reader)?,
                                ),
                                event: ::xsd_parser::quick_xml::DeserializerEvent::None,
                                allow_any: false,
                            });
                        }
                        (S::Init__, event) => {
                            fallback.get_or_insert(S::Init__);
                            *self.state__ = RootTypeDeserializerState::Container(None);
                            event
                        }
                        (
                            S::Container(None),
                            event @ (::xsd_parser::quick_xml::Event::Start(_)
                            | ::xsd_parser::quick_xml::Event::Empty(_)),
                        ) => {
                            let output = reader.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"Container",
                                true,
                            )?;
                            match self.handle_container(reader, output, &mut fallback)? {
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                    event,
                                    allow_any,
                                } => {
                                    allow_any_element = allow_any_element || allow_any;
                                    event
                                }
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Break {
                                    event,
                                    allow_any,
                                } => break (event, allow_any),
                            }
                        }
                        (S::Done__, event) => {
                            fallback.get_or_insert(S::Done__);
                            break (
                                ::xsd_parser::quick_xml::DeserializerEvent::Continue(event),
                                allow_any_element,
                            );
                        }
                        (state, event) => {
                            *self.state__ = state;
                            break (
                                ::xsd_parser::quick_xml::DeserializerEvent::Break(event),
                                false,
                            );
                        }
                    }
                };
                if let Some(fallback) = fallback {
                    *self.state__ = fallback;
                }
                Ok(::xsd_parser::quick_xml::DeserializerOutput {
                    artifact: ::xsd_parser::quick_xml::DeserializerArtifact::Deserializer(self),
                    event,
                    allow_any,
                })
            }
            fn finish<R>(
                mut self,
                reader: &R,
            ) -> ::core::result::Result<super::RootType, ::xsd_parser::quick_xml::Error>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                let state =
                    ::core::mem::replace(&mut *self.state__, RootTypeDeserializerState::Unknown__);
                self.finish_state(reader, state)?;
                Ok(super::RootType {
                    container: self.container.ok_or_else(|| {
                        ::xsd_parser::quick_xml::ErrorKind::MissingElement("Container".into())
                    })?,
                })
            }
        }
        #[derive(Debug)]
        pub struct ContainerTypeDeserializer {
            content: ::std::vec::Vec<super::ContainerTypeContent>,
            state__: ::std::boxed::Box<ContainerTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum ContainerTypeDeserializerState {
            Init__ , Next__ , Content__ (< super :: ContainerTypeContent as :: xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
        impl ContainerTypeDeserializer {
            fn from_bytes_start<R>(
                reader: &R,
                bytes_start: &::xsd_parser::quick_xml::BytesStart<'_>,
            ) -> ::core::result::Result<Self, ::xsd_parser::quick_xml::Error>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                for attrib in ::xsd_parser::quick_xml::filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    reader.raise_unexpected_attrib_checked(attrib)?;
                }
                Ok(Self {
                    content: ::std::vec::Vec::new(),
                    state__: ::std::boxed::Box::new(ContainerTypeDeserializerState::Init__),
                })
            }
            fn finish_state<R>(
                &mut self,
                reader: &R,
                state: ContainerTypeDeserializerState,
            ) -> ::core::result::Result<(), ::xsd_parser::quick_xml::Error>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                if let ContainerTypeDeserializerState::Content__(deserializer) = state {
                    self.store_content(deserializer.finish(reader)?)?;
                }
                Ok(())
            }
            fn store_content(
                &mut self,
                value: super::ContainerTypeContent,
            ) -> ::core::result::Result<(), ::xsd_parser::quick_xml::Error> {
                self.content.push(value);
                Ok(())
            }
            fn handle_content<'de, R>(
                &mut self,
                reader: &R,
                output: ::xsd_parser::quick_xml::DeserializerOutput<
                    'de,
                    super::ContainerTypeContent,
                >,
                fallback: &mut ::core::option::Option<ContainerTypeDeserializerState>,
            ) -> ::core::result::Result<
                ::xsd_parser::quick_xml::ElementHandlerOutput<'de>,
                ::xsd_parser::quick_xml::Error,
            >
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                let ::xsd_parser::quick_xml::DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    *self.state__ = fallback
                        .take()
                        .unwrap_or(ContainerTypeDeserializerState::Next__);
                    return Ok(::xsd_parser::quick_xml::ElementHandlerOutput::break_(
                        event, allow_any,
                    ));
                }
                if let Some(fallback) = fallback.take() {
                    self.finish_state(reader, fallback)?;
                }
                Ok(match artifact {
                    ::xsd_parser::quick_xml::DeserializerArtifact::None => unreachable!(),
                    ::xsd_parser::quick_xml::DeserializerArtifact::Data(data) => {
                        self.store_content(data)?;
                        *self.state__ = ContainerTypeDeserializerState::Next__;
                        ::xsd_parser::quick_xml::ElementHandlerOutput::from_event(event, allow_any)
                    }
                    ::xsd_parser::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                        let ret = ::xsd_parser::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        );
                        match &ret {
                            ::xsd_parser::quick_xml::ElementHandlerOutput::Break { .. } => {
                                *self.state__ =
                                    ContainerTypeDeserializerState::Content__(deserializer);
                            }
                            ::xsd_parser::quick_xml::ElementHandlerOutput::Continue { .. } => {
                                fallback.get_or_insert(ContainerTypeDeserializerState::Content__(
                                    deserializer,
                                ));
                                *self.state__ = ContainerTypeDeserializerState::Next__;
                            }
                        }
                        ret
                    }
                })
            }
        }
        impl<'de> ::xsd_parser::quick_xml::Deserializer<'de, super::ContainerType>
            for ContainerTypeDeserializer
        {
            fn init<R>(
                reader: &R,
                event: ::xsd_parser::quick_xml::Event<'de>,
            ) -> ::xsd_parser::quick_xml::DeserializerResult<'de, super::ContainerType>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next<R>(
                mut self,
                reader: &R,
                event: ::xsd_parser::quick_xml::Event<'de>,
            ) -> ::xsd_parser::quick_xml::DeserializerResult<'de, super::ContainerType>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                use ContainerTypeDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let (event, allow_any) = loop {
                    let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::Content__(deserializer), event) => {
                            let output = deserializer.next(reader, event)?;
                            match self.handle_content(reader, output, &mut fallback)? {
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Break {
                                    event,
                                    allow_any,
                                } => break (event, allow_any),
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                    event,
                                    ..
                                } => event,
                            }
                        }
                        (_, ::xsd_parser::quick_xml::Event::End(_)) => {
                            return Ok(::xsd_parser::quick_xml::DeserializerOutput {
                                artifact: ::xsd_parser::quick_xml::DeserializerArtifact::Data(
                                    self.finish(reader)?,
                                ),
                                event: ::xsd_parser::quick_xml::DeserializerEvent::None,
                                allow_any: false,
                            });
                        }
                        (state @ (S::Init__ | S::Next__), event) => {
                            fallback.get_or_insert(state);
                            let output = < super :: ContainerTypeContent as :: xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_content(reader, output, &mut fallback)? {
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Break {
                                    event,
                                    allow_any,
                                } => break (event, allow_any),
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                    event,
                                    ..
                                } => event,
                            }
                        }
                    }
                };
                let artifact = ::xsd_parser::quick_xml::DeserializerArtifact::Deserializer(self);
                Ok(::xsd_parser::quick_xml::DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                })
            }
            fn finish<R>(
                mut self,
                reader: &R,
            ) -> ::core::result::Result<super::ContainerType, ::xsd_parser::quick_xml::Error>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                let state = ::core::mem::replace(
                    &mut *self.state__,
                    ContainerTypeDeserializerState::Unknown__,
                );
                self.finish_state(reader, state)?;
                Ok(super::ContainerType {
                    content: self.content,
                })
            }
        }
        #[derive(Debug)]
        pub struct ContainerTypeContentDeserializer {
            state__: ::std::boxed::Box<ContainerTypeContentDeserializerState>,
        }
        #[derive(Debug)]
        pub enum ContainerTypeContentDeserializerState {
            Init__ , Known (:: core :: option :: Option < super :: KnownType > , :: core :: option :: Option << super :: KnownType as :: xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , Any (:: core :: option :: Option < AnyElement > , :: core :: option :: Option << AnyElement as :: xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , Text (:: core :: option :: Option < :: xsd_parser :: xml :: Text > , :: core :: option :: Option << :: xsd_parser :: xml :: Text as :: xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ (super :: ContainerTypeContent) , Unknown__ , }
        impl ContainerTypeContentDeserializer {
            fn find_suitable<'de, R>(
                &mut self,
                reader: &R,
                event: ::xsd_parser::quick_xml::Event<'de>,
                fallback: &mut ::core::option::Option<ContainerTypeContentDeserializerState>,
            ) -> ::core::result::Result<
                ::xsd_parser::quick_xml::ElementHandlerOutput<'de>,
                ::xsd_parser::quick_xml::Error,
            >
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                let mut event = event;
                if let ::xsd_parser::quick_xml::Event::Start(x)
                | ::xsd_parser::quick_xml::Event::Empty(x) = &event
                {
                    if matches!(
                        reader.resolve_local_name(x.name(), &super::super::NS_TNS),
                        Some(b"Known")
                    ) {
                        let output = < super :: KnownType as :: xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        return self.handle_known(
                            reader,
                            Default::default(),
                            output,
                            &mut *fallback,
                        );
                    }
                    event = {
                        let output = < AnyElement as :: xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                        match self.handle_any(reader, Default::default(), output, &mut *fallback)? {
                            ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            output => {
                                return Ok(output);
                            }
                        }
                    };
                }
                let output = < :: xsd_parser :: xml :: Text as :: xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                self.handle_text(reader, Default::default(), output, &mut *fallback)
            }
            fn finish_state<R>(
                reader: &R,
                state: ContainerTypeContentDeserializerState,
            ) -> ::core::result::Result<super::ContainerTypeContent, ::xsd_parser::quick_xml::Error>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                use ContainerTypeContentDeserializerState as S;
                match state {
                    S::Unknown__ => unreachable!(),
                    S::Init__ => Err(::xsd_parser::quick_xml::ErrorKind::MissingContent.into()),
                    S::Known(mut values, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(reader)?;
                            Self::store_known(&mut values, value)?;
                        }
                        Ok(super::ContainerTypeContent::Known(values.ok_or_else(
                            || ::xsd_parser::quick_xml::ErrorKind::MissingElement("Known".into()),
                        )?))
                    }
                    S::Any(mut values, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(reader)?;
                            Self::store_any(&mut values, value)?;
                        }
                        Ok(super::ContainerTypeContent::Any(values.ok_or_else(
                            || ::xsd_parser::quick_xml::ErrorKind::MissingElement("any2".into()),
                        )?))
                    }
                    S::Text(mut values, deserializer) => {
                        if let Some(deserializer) = deserializer {
                            let value = deserializer.finish(reader)?;
                            Self::store_text(&mut values, value)?;
                        }
                        Ok(super::ContainerTypeContent::Text(values.ok_or_else(
                            || ::xsd_parser::quick_xml::ErrorKind::MissingElement("text".into()),
                        )?))
                    }
                    S::Done__(data) => Ok(data),
                }
            }
            fn store_known(
                values: &mut ::core::option::Option<super::KnownType>,
                value: super::KnownType,
            ) -> ::core::result::Result<(), ::xsd_parser::quick_xml::Error> {
                if values.is_some() {
                    Err(::xsd_parser::quick_xml::ErrorKind::DuplicateElement(
                        ::xsd_parser::quick_xml::RawByteStr::from_slice(b"Known"),
                    ))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_any(
                values: &mut ::core::option::Option<AnyElement>,
                value: AnyElement,
            ) -> ::core::result::Result<(), ::xsd_parser::quick_xml::Error> {
                if values.is_some() {
                    Err(::xsd_parser::quick_xml::ErrorKind::DuplicateElement(
                        ::xsd_parser::quick_xml::RawByteStr::from_slice(b"any2"),
                    ))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn store_text(
                values: &mut ::core::option::Option<::xsd_parser::xml::Text>,
                value: ::xsd_parser::xml::Text,
            ) -> ::core::result::Result<(), ::xsd_parser::quick_xml::Error> {
                if values.is_some() {
                    Err(::xsd_parser::quick_xml::ErrorKind::DuplicateElement(
                        ::xsd_parser::quick_xml::RawByteStr::from_slice(b"text"),
                    ))?;
                }
                *values = Some(value);
                Ok(())
            }
            fn handle_known<'de, R>(
                &mut self,
                reader: &R,
                mut values: ::core::option::Option<super::KnownType>,
                output: ::xsd_parser::quick_xml::DeserializerOutput<'de, super::KnownType>,
                fallback: &mut ::core::option::Option<ContainerTypeContentDeserializerState>,
            ) -> ::core::result::Result<
                ::xsd_parser::quick_xml::ElementHandlerOutput<'de>,
                ::xsd_parser::quick_xml::Error,
            >
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                let ::xsd_parser::quick_xml::DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    *self.state__ = match fallback.take() {
                        None if values.is_none() => {
                            *self.state__ = ContainerTypeContentDeserializerState::Init__;
                            return Ok(::xsd_parser::quick_xml::ElementHandlerOutput::from_event(
                                event, allow_any,
                            ));
                        }
                        None => ContainerTypeContentDeserializerState::Known(values, None),
                        Some(ContainerTypeContentDeserializerState::Known(
                            _,
                            Some(deserializer),
                        )) => {
                            ContainerTypeContentDeserializerState::Known(values, Some(deserializer))
                        }
                        _ => unreachable!(),
                    };
                    return Ok(::xsd_parser::quick_xml::ElementHandlerOutput::break_(
                        event, allow_any,
                    ));
                }
                match fallback.take() {
                    None => (),
                    Some(ContainerTypeContentDeserializerState::Known(_, Some(deserializer))) => {
                        let data = deserializer.finish(reader)?;
                        Self::store_known(&mut values, data)?;
                    }
                    Some(_) => unreachable!(),
                }
                Ok(match artifact {
                    ::xsd_parser::quick_xml::DeserializerArtifact::None => unreachable!(),
                    ::xsd_parser::quick_xml::DeserializerArtifact::Data(data) => {
                        Self::store_known(&mut values, data)?;
                        let data = Self::finish_state(
                            reader,
                            ContainerTypeContentDeserializerState::Known(values, None),
                        )?;
                        *self.state__ = ContainerTypeContentDeserializerState::Done__(data);
                        ::xsd_parser::quick_xml::ElementHandlerOutput::Break { event, allow_any }
                    }
                    ::xsd_parser::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ = ContainerTypeContentDeserializerState::Known(
                            values,
                            Some(deserializer),
                        );
                        ::xsd_parser::quick_xml::ElementHandlerOutput::from_event_end(
                            event, allow_any,
                        )
                    }
                })
            }
            fn handle_any<'de, R>(
                &mut self,
                reader: &R,
                mut values: ::core::option::Option<AnyElement>,
                output: ::xsd_parser::quick_xml::DeserializerOutput<'de, AnyElement>,
                fallback: &mut ::core::option::Option<ContainerTypeContentDeserializerState>,
            ) -> ::core::result::Result<
                ::xsd_parser::quick_xml::ElementHandlerOutput<'de>,
                ::xsd_parser::quick_xml::Error,
            >
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                let ::xsd_parser::quick_xml::DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    *self.state__ = match fallback.take() {
                        None if values.is_none() => {
                            *self.state__ = ContainerTypeContentDeserializerState::Init__;
                            return Ok(::xsd_parser::quick_xml::ElementHandlerOutput::from_event(
                                event, allow_any,
                            ));
                        }
                        None => ContainerTypeContentDeserializerState::Any(values, None),
                        Some(ContainerTypeContentDeserializerState::Any(_, Some(deserializer))) => {
                            ContainerTypeContentDeserializerState::Any(values, Some(deserializer))
                        }
                        _ => unreachable!(),
                    };
                    return Ok(::xsd_parser::quick_xml::ElementHandlerOutput::break_(
                        event, allow_any,
                    ));
                }
                match fallback.take() {
                    None => (),
                    Some(ContainerTypeContentDeserializerState::Any(_, Some(deserializer))) => {
                        let data = deserializer.finish(reader)?;
                        Self::store_any(&mut values, data)?;
                    }
                    Some(_) => unreachable!(),
                }
                Ok(match artifact {
                    ::xsd_parser::quick_xml::DeserializerArtifact::None => unreachable!(),
                    ::xsd_parser::quick_xml::DeserializerArtifact::Data(data) => {
                        Self::store_any(&mut values, data)?;
                        let data = Self::finish_state(
                            reader,
                            ContainerTypeContentDeserializerState::Any(values, None),
                        )?;
                        *self.state__ = ContainerTypeContentDeserializerState::Done__(data);
                        ::xsd_parser::quick_xml::ElementHandlerOutput::Break { event, allow_any }
                    }
                    ::xsd_parser::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ =
                            ContainerTypeContentDeserializerState::Any(values, Some(deserializer));
                        ::xsd_parser::quick_xml::ElementHandlerOutput::from_event_end(
                            event, allow_any,
                        )
                    }
                })
            }
            fn handle_text<'de, R>(
                &mut self,
                reader: &R,
                mut values: ::core::option::Option<::xsd_parser::xml::Text>,
                output: ::xsd_parser::quick_xml::DeserializerOutput<'de, ::xsd_parser::xml::Text>,
                fallback: &mut ::core::option::Option<ContainerTypeContentDeserializerState>,
            ) -> ::core::result::Result<
                ::xsd_parser::quick_xml::ElementHandlerOutput<'de>,
                ::xsd_parser::quick_xml::Error,
            >
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                let ::xsd_parser::quick_xml::DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                } = output;
                if artifact.is_none() {
                    *self.state__ = match fallback.take() {
                        None if values.is_none() => {
                            *self.state__ = ContainerTypeContentDeserializerState::Init__;
                            return Ok(::xsd_parser::quick_xml::ElementHandlerOutput::from_event(
                                event, allow_any,
                            ));
                        }
                        None => ContainerTypeContentDeserializerState::Text(values, None),
                        Some(ContainerTypeContentDeserializerState::Text(
                            _,
                            Some(deserializer),
                        )) => {
                            ContainerTypeContentDeserializerState::Text(values, Some(deserializer))
                        }
                        _ => unreachable!(),
                    };
                    return Ok(::xsd_parser::quick_xml::ElementHandlerOutput::break_(
                        event, allow_any,
                    ));
                }
                match fallback.take() {
                    None => (),
                    Some(ContainerTypeContentDeserializerState::Text(_, Some(deserializer))) => {
                        let data = deserializer.finish(reader)?;
                        Self::store_text(&mut values, data)?;
                    }
                    Some(_) => unreachable!(),
                }
                Ok(match artifact {
                    ::xsd_parser::quick_xml::DeserializerArtifact::None => unreachable!(),
                    ::xsd_parser::quick_xml::DeserializerArtifact::Data(data) => {
                        Self::store_text(&mut values, data)?;
                        let data = Self::finish_state(
                            reader,
                            ContainerTypeContentDeserializerState::Text(values, None),
                        )?;
                        *self.state__ = ContainerTypeContentDeserializerState::Done__(data);
                        ::xsd_parser::quick_xml::ElementHandlerOutput::Break { event, allow_any }
                    }
                    ::xsd_parser::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                        *self.state__ =
                            ContainerTypeContentDeserializerState::Text(values, Some(deserializer));
                        ::xsd_parser::quick_xml::ElementHandlerOutput::from_event_end(
                            event, allow_any,
                        )
                    }
                })
            }
        }
        impl<'de> ::xsd_parser::quick_xml::Deserializer<'de, super::ContainerTypeContent>
            for ContainerTypeContentDeserializer
        {
            fn init<R>(
                reader: &R,
                event: ::xsd_parser::quick_xml::Event<'de>,
            ) -> ::xsd_parser::quick_xml::DeserializerResult<'de, super::ContainerTypeContent>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                let deserializer = Self {
                    state__: ::std::boxed::Box::new(ContainerTypeContentDeserializerState::Init__),
                };
                let mut output = deserializer.next(reader, event)?;
                output.artifact = match output.artifact {
                    ::xsd_parser::quick_xml::DeserializerArtifact::Deserializer(x)
                        if matches!(&*x.state__, ContainerTypeContentDeserializerState::Init__) =>
                    {
                        ::xsd_parser::quick_xml::DeserializerArtifact::None
                    }
                    artifact => artifact,
                };
                Ok(output)
            }
            fn next<R>(
                mut self,
                reader: &R,
                event: ::xsd_parser::quick_xml::Event<'de>,
            ) -> ::xsd_parser::quick_xml::DeserializerResult<'de, super::ContainerTypeContent>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                use ContainerTypeContentDeserializerState as S;
                let mut event = event;
                let mut fallback = None;
                let (event, allow_any) = loop {
                    let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                    event = match (state, event) {
                        (S::Unknown__, _) => unreachable!(),
                        (S::Known(values, Some(deserializer)), event) => {
                            let output = deserializer.next(reader, event)?;
                            match self.handle_known(reader, values, output, &mut fallback)? {
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Break {
                                    event,
                                    allow_any,
                                } => break (event, allow_any),
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                    event,
                                    ..
                                } => event,
                            }
                        }
                        (S::Any(values, Some(deserializer)), event) => {
                            let output = deserializer.next(reader, event)?;
                            match self.handle_any(reader, values, output, &mut fallback)? {
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Break {
                                    event,
                                    allow_any,
                                } => break (event, allow_any),
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                    event,
                                    ..
                                } => event,
                            }
                        }
                        (S::Text(values, Some(deserializer)), event) => {
                            let output = deserializer.next(reader, event)?;
                            match self.handle_text(reader, values, output, &mut fallback)? {
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Break {
                                    event,
                                    allow_any,
                                } => break (event, allow_any),
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                    event,
                                    ..
                                } => event,
                            }
                        }
                        (state, event @ ::xsd_parser::quick_xml::Event::End(_)) => {
                            return Ok(::xsd_parser::quick_xml::DeserializerOutput {
                                artifact: ::xsd_parser::quick_xml::DeserializerArtifact::Data(
                                    Self::finish_state(reader, state)?,
                                ),
                                event: ::xsd_parser::quick_xml::DeserializerEvent::Continue(event),
                                allow_any: false,
                            });
                        }
                        (S::Init__, event) => {
                            match self.find_suitable(reader, event, &mut fallback)? {
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Break {
                                    event,
                                    allow_any,
                                } => break (event, allow_any),
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                    event,
                                    ..
                                } => event,
                            }
                        }
                        (
                            S::Known(values, None),
                            event @ (::xsd_parser::quick_xml::Event::Start(_)
                            | ::xsd_parser::quick_xml::Event::Empty(_)),
                        ) => {
                            let output = reader.init_start_tag_deserializer(
                                event,
                                Some(&super::super::NS_TNS),
                                b"Known",
                                false,
                            )?;
                            match self.handle_known(reader, values, output, &mut fallback)? {
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Break {
                                    event,
                                    allow_any,
                                } => break (event, allow_any),
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                    event,
                                    ..
                                } => event,
                            }
                        }
                        (
                            S::Any(values, None),
                            event @ (::xsd_parser::quick_xml::Event::Start(_)
                            | ::xsd_parser::quick_xml::Event::Empty(_)),
                        ) => {
                            let output =
                                reader.init_start_tag_deserializer(event, None, b"any2", true)?;
                            match self.handle_any(reader, values, output, &mut fallback)? {
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Break {
                                    event,
                                    allow_any,
                                } => break (event, allow_any),
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                    event,
                                    ..
                                } => event,
                            }
                        }
                        (
                            S::Text(values, None),
                            event @ (::xsd_parser::quick_xml::Event::Start(_)
                            | ::xsd_parser::quick_xml::Event::Empty(_)),
                        ) => {
                            let output = < :: xsd_parser :: xml :: Text as :: xsd_parser :: quick_xml :: WithDeserializer > :: Deserializer :: init (reader , event) ? ;
                            match self.handle_text(reader, values, output, &mut fallback)? {
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Break {
                                    event,
                                    allow_any,
                                } => break (event, allow_any),
                                ::xsd_parser::quick_xml::ElementHandlerOutput::Continue {
                                    event,
                                    ..
                                } => event,
                            }
                        }
                        (s @ S::Done__(_), event) => {
                            *self.state__ = s;
                            break (
                                ::xsd_parser::quick_xml::DeserializerEvent::Continue(event),
                                false,
                            );
                        }
                        (
                            state,
                            ::xsd_parser::quick_xml::Event::Text(_)
                            | ::xsd_parser::quick_xml::Event::CData(_),
                        ) => {
                            *self.state__ = state;
                            break (::xsd_parser::quick_xml::DeserializerEvent::None, false);
                        }
                        (state, event) => {
                            *self.state__ = state;
                            break (
                                ::xsd_parser::quick_xml::DeserializerEvent::Break(event),
                                false,
                            );
                        }
                    }
                };
                let artifact = if matches!(&*self.state__, S::Done__(_)) {
                    ::xsd_parser::quick_xml::DeserializerArtifact::Data(self.finish(reader)?)
                } else {
                    ::xsd_parser::quick_xml::DeserializerArtifact::Deserializer(self)
                };
                Ok(::xsd_parser::quick_xml::DeserializerOutput {
                    artifact,
                    event,
                    allow_any,
                })
            }
            fn finish<R>(
                self,
                reader: &R,
            ) -> ::core::result::Result<super::ContainerTypeContent, ::xsd_parser::quick_xml::Error>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                Self::finish_state(reader, *self.state__)
            }
        }
        #[derive(Debug)]
        pub struct KnownTypeDeserializer {
            name: ::core::option::Option<::std::string::String>,
            state__: ::std::boxed::Box<KnownTypeDeserializerState>,
        }
        #[derive(Debug)]
        enum KnownTypeDeserializerState {
            Init__,
            Unknown__,
        }
        impl KnownTypeDeserializer {
            fn from_bytes_start<R>(
                reader: &R,
                bytes_start: &::xsd_parser::quick_xml::BytesStart<'_>,
            ) -> ::core::result::Result<Self, ::xsd_parser::quick_xml::Error>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                let mut name: ::core::option::Option<::std::string::String> = None;
                for attrib in ::xsd_parser::quick_xml::filter_xmlns_attributes(bytes_start) {
                    let attrib = attrib?;
                    if matches!(
                        reader.resolve_local_name(attrib.key, &super::super::NS_TNS),
                        Some(b"name")
                    ) {
                        reader.read_attrib(&mut name, b"name", &attrib.value)?;
                    } else {
                        reader.raise_unexpected_attrib_checked(attrib)?;
                    }
                }
                Ok(Self {
                    name: name,
                    state__: ::std::boxed::Box::new(KnownTypeDeserializerState::Init__),
                })
            }
            fn finish_state<R>(
                &mut self,
                reader: &R,
                state: KnownTypeDeserializerState,
            ) -> ::core::result::Result<(), ::xsd_parser::quick_xml::Error>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                Ok(())
            }
        }
        impl<'de> ::xsd_parser::quick_xml::Deserializer<'de, super::KnownType> for KnownTypeDeserializer {
            fn init<R>(
                reader: &R,
                event: ::xsd_parser::quick_xml::Event<'de>,
            ) -> ::xsd_parser::quick_xml::DeserializerResult<'de, super::KnownType>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
            }
            fn next<R>(
                mut self,
                reader: &R,
                event: ::xsd_parser::quick_xml::Event<'de>,
            ) -> ::xsd_parser::quick_xml::DeserializerResult<'de, super::KnownType>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                if let ::xsd_parser::quick_xml::Event::End(_) = &event {
                    Ok(::xsd_parser::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser::quick_xml::DeserializerArtifact::Data(
                            self.finish(reader)?,
                        ),
                        event: ::xsd_parser::quick_xml::DeserializerEvent::None,
                        allow_any: false,
                    })
                } else {
                    Ok(::xsd_parser::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser::quick_xml::DeserializerArtifact::Deserializer(self),
                        event: ::xsd_parser::quick_xml::DeserializerEvent::Break(event),
                        allow_any: false,
                    })
                }
            }
            fn finish<R>(
                mut self,
                reader: &R,
            ) -> ::core::result::Result<super::KnownType, ::xsd_parser::quick_xml::Error>
            where
                R: ::xsd_parser::quick_xml::DeserializeReader,
            {
                let state =
                    ::core::mem::replace(&mut *self.state__, KnownTypeDeserializerState::Unknown__);
                self.finish_state(reader, state)?;
                Ok(super::KnownType { name: self.name })
            }
        }
    }
    pub mod quick_xml_serialize {
        use xsd_parser::xml::AnyElement;
        #[derive(Debug)]
        pub struct RootTypeSerializer<'ser> {
            pub(super) value: &'ser super::RootType,
            pub(super) state: ::std::boxed::Box<RootTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum RootTypeSerializerState<'ser> {
            Init__,
            Container(
                <super::ContainerType as ::xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>,
            ),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> RootTypeSerializer<'ser> {
            fn next_event(
                &mut self,
            ) -> ::core::result::Result<
                ::core::option::Option<::xsd_parser::quick_xml::Event<'ser>>,
                ::xsd_parser::quick_xml::Error,
            > {
                loop {
                    match &mut *self.state {
                        RootTypeSerializerState::Init__ => {
                            *self.state = RootTypeSerializerState::Container(
                                ::xsd_parser::quick_xml::WithSerializer::serializer(
                                    &self.value.container,
                                    Some("tns:Container"),
                                    false,
                                )?,
                            );
                            let mut bytes = ::xsd_parser::quick_xml::BytesStart::new(self.name);
                            if self.is_root {
                                bytes
                                    .push_attribute((&b"xmlns:tns"[..], &super::super::NS_TNS[..]));
                                bytes.push_attribute((
                                    &b"xmlns:xsi"[..],
                                    &::xsd_parser::models::schema::Namespace::XSI[..],
                                ));
                            }
                            return Ok(Some(::xsd_parser::quick_xml::Event::Start(bytes)));
                        }
                        RootTypeSerializerState::Container(x) => match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = RootTypeSerializerState::End__,
                        },
                        RootTypeSerializerState::End__ => {
                            *self.state = RootTypeSerializerState::Done__;
                            return Ok(Some(::xsd_parser::quick_xml::Event::End(
                                ::xsd_parser::quick_xml::BytesEnd::new(self.name),
                            )));
                        }
                        RootTypeSerializerState::Done__ => return Ok(None),
                        RootTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> ::core::iter::Iterator for RootTypeSerializer<'ser> {
            type Item = ::core::result::Result<
                ::xsd_parser::quick_xml::Event<'ser>,
                ::xsd_parser::quick_xml::Error,
            >;
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                match self.next_event() {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = RootTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct ContainerTypeSerializer<'ser> {
            pub(super) value: &'ser super::ContainerType,
            pub(super) state: ::std::boxed::Box<ContainerTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum ContainerTypeSerializerState<'ser> {
            Init__,
            Content__(
                ::xsd_parser::quick_xml::IterSerializer<
                    'ser,
                    &'ser [super::ContainerTypeContent],
                    super::ContainerTypeContent,
                >,
            ),
            End__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> ContainerTypeSerializer<'ser> {
            fn next_event(
                &mut self,
            ) -> ::core::result::Result<
                ::core::option::Option<::xsd_parser::quick_xml::Event<'ser>>,
                ::xsd_parser::quick_xml::Error,
            > {
                loop {
                    match &mut *self.state {
                        ContainerTypeSerializerState::Init__ => {
                            *self.state = ContainerTypeSerializerState::Content__(
                                ::xsd_parser::quick_xml::IterSerializer::new(
                                    &self.value.content[..],
                                    None,
                                    false,
                                ),
                            );
                            let mut bytes = ::xsd_parser::quick_xml::BytesStart::new(self.name);
                            if self.is_root {
                                bytes
                                    .push_attribute((&b"xmlns:tns"[..], &super::super::NS_TNS[..]));
                                bytes.push_attribute((
                                    &b"xmlns:xsi"[..],
                                    &::xsd_parser::models::schema::Namespace::XSI[..],
                                ));
                            }
                            return Ok(Some(::xsd_parser::quick_xml::Event::Start(bytes)));
                        }
                        ContainerTypeSerializerState::Content__(x) => match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ContainerTypeSerializerState::End__,
                        },
                        ContainerTypeSerializerState::End__ => {
                            *self.state = ContainerTypeSerializerState::Done__;
                            return Ok(Some(::xsd_parser::quick_xml::Event::End(
                                ::xsd_parser::quick_xml::BytesEnd::new(self.name),
                            )));
                        }
                        ContainerTypeSerializerState::Done__ => return Ok(None),
                        ContainerTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> ::core::iter::Iterator for ContainerTypeSerializer<'ser> {
            type Item = ::core::result::Result<
                ::xsd_parser::quick_xml::Event<'ser>,
                ::xsd_parser::quick_xml::Error,
            >;
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                match self.next_event() {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = ContainerTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct ContainerTypeContentSerializer<'ser> {
            pub(super) value: &'ser super::ContainerTypeContent,
            pub(super) state: ::std::boxed::Box<ContainerTypeContentSerializerState<'ser>>,
        }
        #[derive(Debug)]
        pub(super) enum ContainerTypeContentSerializerState<'ser> {
            Init__,
            Known(<super::KnownType as ::xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
            Any(<AnyElement as ::xsd_parser::quick_xml::WithSerializer>::Serializer<'ser>),
            Text(
                <::xsd_parser::xml::Text as ::xsd_parser::quick_xml::WithSerializer>::Serializer<
                    'ser,
                >,
            ),
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> ContainerTypeContentSerializer<'ser> {
            fn next_event(
                &mut self,
            ) -> ::core::result::Result<
                ::core::option::Option<::xsd_parser::quick_xml::Event<'ser>>,
                ::xsd_parser::quick_xml::Error,
            > {
                loop {
                    match &mut *self.state {
                        ContainerTypeContentSerializerState::Init__ => match self.value {
                            super::ContainerTypeContent::Known(x) => {
                                *self.state = ContainerTypeContentSerializerState::Known(
                                    ::xsd_parser::quick_xml::WithSerializer::serializer(
                                        x,
                                        Some("tns:Known"),
                                        false,
                                    )?,
                                )
                            }
                            super::ContainerTypeContent::Any(x) => {
                                *self.state = ContainerTypeContentSerializerState::Any(
                                    ::xsd_parser::quick_xml::WithSerializer::serializer(
                                        x, None, false,
                                    )?,
                                )
                            }
                            super::ContainerTypeContent::Text(x) => {
                                *self.state = ContainerTypeContentSerializerState::Text(
                                    ::xsd_parser::quick_xml::WithSerializer::serializer(
                                        x,
                                        Some("text"),
                                        false,
                                    )?,
                                )
                            }
                        },
                        ContainerTypeContentSerializerState::Known(x) => {
                            match x.next().transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = ContainerTypeContentSerializerState::Done__,
                            }
                        }
                        ContainerTypeContentSerializerState::Any(x) => {
                            match x.next().transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = ContainerTypeContentSerializerState::Done__,
                            }
                        }
                        ContainerTypeContentSerializerState::Text(x) => {
                            match x.next().transpose()? {
                                Some(event) => return Ok(Some(event)),
                                None => *self.state = ContainerTypeContentSerializerState::Done__,
                            }
                        }
                        ContainerTypeContentSerializerState::Done__ => return Ok(None),
                        ContainerTypeContentSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> ::core::iter::Iterator for ContainerTypeContentSerializer<'ser> {
            type Item = ::core::result::Result<
                ::xsd_parser::quick_xml::Event<'ser>,
                ::xsd_parser::quick_xml::Error,
            >;
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                match self.next_event() {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = ContainerTypeContentSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
        #[derive(Debug)]
        pub struct KnownTypeSerializer<'ser> {
            pub(super) value: &'ser super::KnownType,
            pub(super) state: ::std::boxed::Box<KnownTypeSerializerState<'ser>>,
            pub(super) name: &'ser str,
            pub(super) is_root: bool,
        }
        #[derive(Debug)]
        pub(super) enum KnownTypeSerializerState<'ser> {
            Init__,
            Done__,
            Phantom__(&'ser ()),
        }
        impl<'ser> KnownTypeSerializer<'ser> {
            fn next_event(
                &mut self,
            ) -> ::core::result::Result<
                ::core::option::Option<::xsd_parser::quick_xml::Event<'ser>>,
                ::xsd_parser::quick_xml::Error,
            > {
                loop {
                    match &mut *self.state {
                        KnownTypeSerializerState::Init__ => {
                            *self.state = KnownTypeSerializerState::Done__;
                            let mut bytes = ::xsd_parser::quick_xml::BytesStart::new(self.name);
                            if self.is_root {
                                bytes
                                    .push_attribute((&b"xmlns:tns"[..], &super::super::NS_TNS[..]));
                                bytes.push_attribute((
                                    &b"xmlns:xsi"[..],
                                    &::xsd_parser::models::schema::Namespace::XSI[..],
                                ));
                            }
                            ::xsd_parser::quick_xml::write_attrib_opt(
                                &mut bytes,
                                "name",
                                &self.value.name,
                            )?;
                            return Ok(Some(::xsd_parser::quick_xml::Event::Empty(bytes)));
                        }
                        KnownTypeSerializerState::Done__ => return Ok(None),
                        KnownTypeSerializerState::Phantom__(_) => unreachable!(),
                    }
                }
            }
        }
        impl<'ser> ::core::iter::Iterator for KnownTypeSerializer<'ser> {
            type Item = ::core::result::Result<
                ::xsd_parser::quick_xml::Event<'ser>,
                ::xsd_parser::quick_xml::Error,
            >;
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                match self.next_event() {
                    Ok(Some(event)) => Some(Ok(event)),
                    Ok(None) => None,
                    Err(error) => {
                        *self.state = KnownTypeSerializerState::Done__;
                        Some(Err(error))
                    }
                }
            }
        }
    }
}
