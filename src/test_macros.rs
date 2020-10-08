use crate::{SerializeResult, Serializer};
#[cfg(test)]
#[macro_export]
macro_rules! packet_test_cases {
    ($pnam: ident, $varnam: ident, $bodnam: ident, $testnam: ident, $benchnams: ident, $benchnamd: ident) => {
        #[test]
        fn $testnam() {
            for _ in 0..10 {
                let packet = $pnam::$varnam($bodnam::test_gen_random());
                let mut out = BytesSerializer::default();
                packet.mc_serialize(&mut out).expect("serialize succeeds");
                let bytes = out.into_bytes();

                let raw_packet = RawPacket {
                    id: packet.id(),
                    data: bytes.as_slice(),
                };

                let deserialized =
                    <$pnam>::mc_deserialize(raw_packet).expect("deserialize succeeds");
                assert_eq!(packet, deserialized);
            }
        }

        #[bench]
        fn $benchnams(b: &mut Bencher) {
            let packet = $pnam::$varnam($bodnam::test_gen_random());
            let mut serializer = BenchSerializer::default();
            packet
                .mc_serialize(&mut serializer)
                .expect("serialize succeeds");
            b.bytes = serializer.len() as u64;
            serializer.reset();

            b.iter(|| {
                packet
                    .mc_serialize(&mut serializer)
                    .expect("serialize succeeds");
                serializer.reset();
            })
        }

        #[bench]
        fn $benchnamd(b: &mut Bencher) {
            let packet = $pnam::$varnam($bodnam::test_gen_random());
            let mut serializer = BytesSerializer::default();
            packet
                .mc_serialize(&mut serializer)
                .expect("serialize succeeds");

            let bytes = serializer.into_bytes();
            b.bytes = bytes.len() as u64;
            let raw_packet = RawPacket {
                id: packet.id(),
                data: bytes.as_slice(),
            };
            b.iter(|| {
                $pnam::mc_deserialize(raw_packet.clone()).expect("deserialize succeeds");
            })
        }
    };
}

#[cfg(test)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BenchSerializer {
    data: Vec<u8>,
}

#[cfg(test)]
impl Serializer for BenchSerializer {
    fn serialize_bytes(&mut self, data: &[u8]) -> SerializeResult {
        self.data.extend_from_slice(data);
        Ok(())
    }
}

#[cfg(test)]
impl BenchSerializer {
    pub fn reset(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}