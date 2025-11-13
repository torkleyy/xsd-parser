use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com/cdata");
pub type Document = DocumentType;
#[derive(Debug)]
pub struct DocumentType {
    pub title: String,
    pub content: String,
    pub code: String,
}
impl WithSerializer for DocumentType {
    type Serializer<'x> = quick_xml_serialize::DocumentTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::DocumentTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::DocumentTypeSerializerState::Init__),
            name: name.unwrap_or("tns:Document"),
            is_root,
        })
    }
}
impl WithDeserializer for DocumentType {
    type Deserializer = quick_xml_deserialize::DocumentTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct DocumentTypeDeserializer {
        title: Option<String>,
        content: Option<String>,
        code: Option<String>,
        state__: Box<DocumentTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum DocumentTypeDeserializerState {
        Init__,
        Title(Option<<String as WithDeserializer>::Deserializer>),
        Content(Option<<String as WithDeserializer>::Deserializer>),
        Code(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl DocumentTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                title: None,
                content: None,
                code: None,
                state__: Box::new(DocumentTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: DocumentTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use DocumentTypeDeserializerState as S;
            match state {
                S::Title(Some(deserializer)) => self.store_title(deserializer.finish(reader)?)?,
                S::Content(Some(deserializer)) => {
                    self.store_content(deserializer.finish(reader)?)?
                }
                S::Code(Some(deserializer)) => self.store_code(deserializer.finish(reader)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_title(&mut self, value: String) -> Result<(), Error> {
            if self.title.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Title",
                )))?;
            }
            self.title = Some(value);
            Ok(())
        }
        fn store_content(&mut self, value: String) -> Result<(), Error> {
            if self.content.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Content",
                )))?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn store_code(&mut self, value: String) -> Result<(), Error> {
            if self.code.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(b"Code")))?;
            }
            self.code = Some(value);
            Ok(())
        }
        fn handle_title<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.title.is_some() {
                    fallback.get_or_insert(DocumentTypeDeserializerState::Title(None));
                    *self.state__ = DocumentTypeDeserializerState::Content(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = DocumentTypeDeserializerState::Title(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_title(data)?;
                    *self.state__ = DocumentTypeDeserializerState::Content(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DocumentTypeDeserializerState::Title(Some(
                                deserializer,
                            )));
                            *self.state__ = DocumentTypeDeserializerState::Content(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                DocumentTypeDeserializerState::Title(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_content<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.content.is_some() {
                    fallback.get_or_insert(DocumentTypeDeserializerState::Content(None));
                    *self.state__ = DocumentTypeDeserializerState::Code(None);
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = DocumentTypeDeserializerState::Content(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    *self.state__ = DocumentTypeDeserializerState::Code(None);
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DocumentTypeDeserializerState::Content(Some(
                                deserializer,
                            )));
                            *self.state__ = DocumentTypeDeserializerState::Code(None);
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                DocumentTypeDeserializerState::Content(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
        fn handle_code<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<DocumentTypeDeserializerState>,
        ) -> Result<ElementHandlerOutput<'de>, Error>
        where
            R: DeserializeReader,
        {
            let DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if self.code.is_some() {
                    fallback.get_or_insert(DocumentTypeDeserializerState::Code(None));
                    *self.state__ = DocumentTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = DocumentTypeDeserializerState::Code(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_code(data)?;
                    *self.state__ = DocumentTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(DocumentTypeDeserializerState::Code(Some(
                                deserializer,
                            )));
                            *self.state__ = DocumentTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ = DocumentTypeDeserializerState::Code(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::DocumentType> for DocumentTypeDeserializer {
        fn init<R>(reader: &R, event: Event<'de>) -> DeserializerResult<'de, super::DocumentType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::DocumentType>
        where
            R: DeserializeReader,
        {
            use DocumentTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Title(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_title(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Content(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Code(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (_, Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(reader, fallback)?;
                        }
                        return Ok(DeserializerOutput {
                            artifact: DeserializerArtifact::Data(self.finish(reader)?),
                            event: DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = DocumentTypeDeserializerState::Title(None);
                        event
                    }
                    (S::Title(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Title",
                            false,
                        )?;
                        match self.handle_title(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Content(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Content",
                            false,
                        )?;
                        match self.handle_content(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Code(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Code",
                            false,
                        )?;
                        match self.handle_code(reader, output, &mut fallback)? {
                            ElementHandlerOutput::Continue { event, allow_any } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ElementHandlerOutput::Break { event, allow_any } => {
                                break (event, allow_any)
                            }
                        }
                    }
                    (S::Done__, event) => {
                        fallback.get_or_insert(S::Done__);
                        break (DeserializerEvent::Continue(event), allow_any_element);
                    }
                    (S::Unknown__, _) => unreachable!(),
                    (state, event) => {
                        *self.state__ = state;
                        break (DeserializerEvent::Break(event), false);
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(DeserializerOutput {
                artifact: DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish<R>(mut self, reader: &R) -> Result<super::DocumentType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(&mut *self.state__, DocumentTypeDeserializerState::Unknown__);
            self.finish_state(reader, state)?;
            Ok(super::DocumentType {
                title: self
                    .title
                    .ok_or_else(|| ErrorKind::MissingElement("Title".into()))?,
                content: self
                    .content
                    .ok_or_else(|| ErrorKind::MissingElement("Content".into()))?,
                code: self
                    .code
                    .ok_or_else(|| ErrorKind::MissingElement("Code".into()))?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, WithSerializer};
    #[derive(Debug)]
    pub struct DocumentTypeSerializer<'ser> {
        pub(super) value: &'ser super::DocumentType,
        pub(super) state: Box<DocumentTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum DocumentTypeSerializerState<'ser> {
        Init__,
        Title(<String as WithSerializer>::Serializer<'ser>),
        Content(<String as WithSerializer>::Serializer<'ser>),
        Code(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> DocumentTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    DocumentTypeSerializerState::Init__ => {
                        *self.state =
                            DocumentTypeSerializerState::Title(WithSerializer::serializer(
                                &self.value.title,
                                Some("tns:Title"),
                                false,
                            )?);
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    DocumentTypeSerializerState::Title(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                DocumentTypeSerializerState::Content(WithSerializer::serializer(
                                    &self.value.content,
                                    Some("tns:Content"),
                                    false,
                                )?)
                        }
                    },
                    DocumentTypeSerializerState::Content(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state =
                                DocumentTypeSerializerState::Code(WithSerializer::serializer(
                                    &self.value.code,
                                    Some("tns:Code"),
                                    false,
                                )?)
                        }
                    },
                    DocumentTypeSerializerState::Code(x) => match x.next().transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = DocumentTypeSerializerState::End__,
                    },
                    DocumentTypeSerializerState::End__ => {
                        *self.state = DocumentTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    DocumentTypeSerializerState::Done__ => return Ok(None),
                    DocumentTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for DocumentTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = DocumentTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
