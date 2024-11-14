use sha3::{
    digest::{core_api::XofReaderCoreWrapper, ExtendableOutput, Update},
    Shake256, Shake256ReaderCore,
};

pub struct DRNG {
    reader: XofReaderCoreWrapper<Shake256ReaderCore>,
}

impl DRNG {
    pub fn new(entropy: [u8; 64]) -> Self {
        let mut hasher = Shake256::default();
        hasher.update(&entropy);
        DRNG {
            reader: hasher.finalize_xof(),
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) {
        sha3::digest::XofReader::read(&mut self.reader, buffer);
    }
}
