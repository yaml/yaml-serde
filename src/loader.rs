use crate::de::{Event, Progress};
use crate::error::{self, Error, ErrorImpl, Result};
use crate::libyaml::error::Mark;
use crate::libyaml::parser::{Event as YamlEvent, Parser};
use alloc::borrow::Cow;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;

pub(crate) struct Loader<'input> {
    parser: Option<Parser<'input>>,
    document_count: usize,
}

pub(crate) struct Document<'input> {
    pub events: Vec<(Event<'input>, Mark)>,
    pub error: Option<Arc<ErrorImpl>>,
    /// Map from alias id to index in events.
    pub aliases: BTreeMap<usize, usize>,
}

impl<'input> Loader<'input> {
    pub fn new(progress: Progress<'input>) -> Result<Self> {
        let input = match progress {
            Progress::Str(s) => Cow::Borrowed(s.as_bytes()),
            Progress::Slice(bytes) => Cow::Borrowed(bytes),
            #[cfg(feature = "std")]
            Progress::Read(mut rdr) => {
                let mut buffer = Vec::new();
                if let Err(io_error) = rdr.read_to_end(&mut buffer) {
                    return Err(error::new(ErrorImpl::Io(io_error)));
                }
                Cow::Owned(buffer)
            }
            Progress::Iterable(_) | Progress::Document(_) => unreachable!(),
            Progress::Fail(err) => return Err(error::shared(err)),
        };

        Ok(Loader {
            parser: Some(Parser::new(input)),
            document_count: 0,
        })
    }

    pub fn next_document(&mut self) -> Option<Document<'input>> {
        let Some(parser) = &mut self.parser else {
            return None;
        };

        let first = self.document_count == 0;
        self.document_count += 1;

        let mut anchors = BTreeMap::new();
        let mut document = Document {
            events: Vec::new(),
            error: None,
            aliases: BTreeMap::new(),
        };

        loop {
            let (event, mark) = match parser.next() {
                Ok((event, mark)) => (event, mark),
                Err(err) => {
                    // The parser latches into an error state, so any further
                    // call would yield the same error forever. Drop it so the
                    // next `next_document` returns None and iteration ends.
                    self.parser = None;
                    document.error = Some(Error::from(err).shared());
                    return Some(document);
                }
            };
            let event = match event {
                YamlEvent::StreamStart => continue,
                YamlEvent::StreamEnd => {
                    self.parser = None;
                    return if first {
                        if document.events.is_empty() {
                            document.events.push((Event::Void, mark));
                        }
                        Some(document)
                    } else {
                        None
                    };
                }
                YamlEvent::DocumentStart => continue,
                YamlEvent::DocumentEnd => return Some(document),
                YamlEvent::Alias(alias) => match anchors.get(&alias) {
                    Some(id) => Event::Alias(*id),
                    None => {
                        document.error = Some(error::new(ErrorImpl::UnknownAnchor(mark)).shared());
                        return Some(document);
                    }
                },
                YamlEvent::Scalar(mut scalar) => {
                    if let Some(anchor) = scalar.anchor.take() {
                        let id = anchors.len();
                        anchors.insert(anchor, id);
                        document.aliases.insert(id, document.events.len());
                    }
                    Event::Scalar(scalar)
                }
                YamlEvent::SequenceStart(mut sequence_start) => {
                    if let Some(anchor) = sequence_start.anchor.take() {
                        let id = anchors.len();
                        anchors.insert(anchor, id);
                        document.aliases.insert(id, document.events.len());
                    }
                    Event::SequenceStart(sequence_start)
                }
                YamlEvent::SequenceEnd => Event::SequenceEnd,
                YamlEvent::MappingStart(mut mapping_start) => {
                    if let Some(anchor) = mapping_start.anchor.take() {
                        let id = anchors.len();
                        anchors.insert(anchor, id);
                        document.aliases.insert(id, document.events.len());
                    }
                    Event::MappingStart(mapping_start)
                }
                YamlEvent::MappingEnd => Event::MappingEnd,
            };
            document.events.push((event, mark));
        }
    }
}
