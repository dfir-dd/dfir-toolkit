use evtx::SerializedEvtxRecord;

pub (crate) struct Unfiltered<'a, V> {
    pub (crate) inner: Box<dyn Iterator<Item = evtx::err::Result<SerializedEvtxRecord<V>>> + 'a>,
}

impl<'a, V> Iterator for Unfiltered<'a, V> {
    type Item = evtx::err::Result<SerializedEvtxRecord<V>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}