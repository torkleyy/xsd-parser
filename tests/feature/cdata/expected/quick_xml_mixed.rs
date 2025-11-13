use xsd_parser::{
    models::schema::Namespace,
    quick_xml::{Error, WithDeserializer, WithSerializer},
};
pub const NS_XS: Namespace = Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: Namespace = Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_TNS: Namespace = Namespace::new_const(b"http://example.com/cdata");
pub type MixedContent = MixedContentType;
#[derive(Debug)]
pub struct MixedContentType {
    pub description: String,
}
impl WithSerializer for MixedContentType {
    type Serializer<'x> = quick_xml_serialize::MixedContentTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: Option<&'ser str>,
        is_root: bool,
    ) -> Result<Self::Serializer<'ser>, Error> {
        Ok(quick_xml_serialize::MixedContentTypeSerializer {
            value: self,
            state: Box::new(quick_xml_serialize::MixedContentTypeSerializerState::Init__),
            name: name.unwrap_or("tns:MixedContent"),
            is_root,
        })
    }
}
impl WithDeserializer for MixedContentType {
    type Deserializer = quick_xml_deserialize::MixedContentTypeDeserializer;
}
pub mod quick_xml_deserialize {
    use core::mem::replace;
    use xsd_parser::quick_xml::{
        filter_xmlns_attributes, BytesStart, DeserializeReader, Deserializer, DeserializerArtifact,
        DeserializerEvent, DeserializerOutput, DeserializerResult, ElementHandlerOutput, Error,
        ErrorKind, Event, RawByteStr, WithDeserializer,
    };
    #[derive(Debug)]
    pub struct MixedContentTypeDeserializer {
        description: Option<String>,
        state__: Box<MixedContentTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum MixedContentTypeDeserializerState {
        Init__,
        Description(Option<<String as WithDeserializer>::Deserializer>),
        Done__,
        Unknown__,
    }
    impl MixedContentTypeDeserializer {
        fn from_bytes_start<R>(reader: &R, bytes_start: &BytesStart<'_>) -> Result<Self, Error>
        where
            R: DeserializeReader,
        {
            for attrib in filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                reader.raise_unexpected_attrib_checked(attrib)?;
            }
            Ok(Self {
                description: None,
                state__: Box::new(MixedContentTypeDeserializerState::Init__),
            })
        }
        fn finish_state<R>(
            &mut self,
            reader: &R,
            state: MixedContentTypeDeserializerState,
        ) -> Result<(), Error>
        where
            R: DeserializeReader,
        {
            use MixedContentTypeDeserializerState as S;
            match state {
                S::Description(Some(deserializer)) => {
                    self.store_description(deserializer.finish(reader)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_description(&mut self, value: String) -> Result<(), Error> {
            if self.description.is_some() {
                Err(ErrorKind::DuplicateElement(RawByteStr::from_slice(
                    b"Description",
                )))?;
            }
            self.description = Some(value);
            Ok(())
        }
        fn handle_description<'de, R>(
            &mut self,
            reader: &R,
            output: DeserializerOutput<'de, String>,
            fallback: &mut Option<MixedContentTypeDeserializerState>,
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
                if self.description.is_some() {
                    fallback.get_or_insert(MixedContentTypeDeserializerState::Description(None));
                    *self.state__ = MixedContentTypeDeserializerState::Done__;
                    return Ok(ElementHandlerOutput::from_event(event, allow_any));
                } else {
                    *self.state__ = MixedContentTypeDeserializerState::Description(None);
                    return Ok(ElementHandlerOutput::break_(event, allow_any));
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(reader, fallback)?;
            }
            Ok(match artifact {
                DeserializerArtifact::None => unreachable!(),
                DeserializerArtifact::Data(data) => {
                    self.store_description(data)?;
                    *self.state__ = MixedContentTypeDeserializerState::Done__;
                    ElementHandlerOutput::from_event(event, allow_any)
                }
                DeserializerArtifact::Deserializer(deserializer) => {
                    let ret = ElementHandlerOutput::from_event(event, allow_any);
                    match &ret {
                        ElementHandlerOutput::Continue { .. } => {
                            fallback.get_or_insert(MixedContentTypeDeserializerState::Description(
                                Some(deserializer),
                            ));
                            *self.state__ = MixedContentTypeDeserializerState::Done__;
                        }
                        ElementHandlerOutput::Break { .. } => {
                            *self.state__ =
                                MixedContentTypeDeserializerState::Description(Some(deserializer));
                        }
                    }
                    ret
                }
            })
        }
    }
    impl<'de> Deserializer<'de, super::MixedContentType> for MixedContentTypeDeserializer {
        fn init<R>(
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedContentType>
        where
            R: DeserializeReader,
        {
            reader.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next<R>(
            mut self,
            reader: &R,
            event: Event<'de>,
        ) -> DeserializerResult<'de, super::MixedContentType>
        where
            R: DeserializeReader,
        {
            use MixedContentTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Description(Some(deserializer)), event) => {
                        let output = deserializer.next(reader, event)?;
                        match self.handle_description(reader, output, &mut fallback)? {
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
                        *self.state__ = MixedContentTypeDeserializerState::Description(None);
                        event
                    }
                    (S::Description(None), event @ (Event::Start(_) | Event::Empty(_))) => {
                        let output = reader.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_TNS),
                            b"Description",
                            false,
                        )?;
                        match self.handle_description(reader, output, &mut fallback)? {
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
        fn finish<R>(mut self, reader: &R) -> Result<super::MixedContentType, Error>
        where
            R: DeserializeReader,
        {
            let state = replace(
                &mut *self.state__,
                MixedContentTypeDeserializerState::Unknown__,
            );
            self.finish_state(reader, state)?;
            Ok(super::MixedContentType {
                description: self
                    .description
                    .ok_or_else(|| ErrorKind::MissingElement("Description".into()))?,
            })
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser::quick_xml::{BytesEnd, BytesStart, Error, Event, WithSerializer};
    #[derive(Debug)]
    pub struct MixedContentTypeSerializer<'ser> {
        pub(super) value: &'ser super::MixedContentType,
        pub(super) state: Box<MixedContentTypeSerializerState<'ser>>,
        pub(super) name: &'ser str,
        pub(super) is_root: bool,
    }
    #[derive(Debug)]
    pub(super) enum MixedContentTypeSerializerState<'ser> {
        Init__,
        Description(<String as WithSerializer>::Serializer<'ser>),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> MixedContentTypeSerializer<'ser> {
        fn next_event(&mut self) -> Result<Option<Event<'ser>>, Error> {
            loop {
                match &mut *self.state {
                    MixedContentTypeSerializerState::Init__ => {
                        *self.state = MixedContentTypeSerializerState::Description(
                            WithSerializer::serializer(
                                &self.value.description,
                                Some("tns:Description"),
                                false,
                            )?,
                        );
                        let mut bytes = BytesStart::new(self.name);
                        if self.is_root {
                            bytes.push_attribute((&b"xmlns:tns"[..], &super::NS_TNS[..]));
                        }
                        return Ok(Some(Event::Start(bytes)));
                    }
                    MixedContentTypeSerializerState::Description(x) => {
                        match x.next().transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = MixedContentTypeSerializerState::End__,
                        }
                    }
                    MixedContentTypeSerializerState::End__ => {
                        *self.state = MixedContentTypeSerializerState::Done__;
                        return Ok(Some(Event::End(BytesEnd::new(self.name))));
                    }
                    MixedContentTypeSerializerState::Done__ => return Ok(None),
                    MixedContentTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> Iterator for MixedContentTypeSerializer<'ser> {
        type Item = Result<Event<'ser>, Error>;
        fn next(&mut self) -> Option<Self::Item> {
            match self.next_event() {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = MixedContentTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
}
