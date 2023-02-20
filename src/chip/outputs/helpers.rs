use crate::{
    chip::{ChipBuilder, ChipRunner, ChipType, Pin, PinType},
    generate_chip, State,
};

/// # 7-Segment decoder
///
/// Takes a nibble as input (4-bits defined by DCBA)
/// Outputs 7-segments format (abcdefg)
/// BI is for blanking the input (active low)
///
/// # Diagram
/// ```txt
///        ---__---
///    B --|1   14|-- VCC
///    C --|2   13|-- f
///  !BI --|3   12|-- g
///    D --|4   11|-- a
///    A --|5   10|-- b
///    e --|6    9|-- c
///  GND --|7    8|-- d
///        --------
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SevenSegmentsDecoder {
    pub vcc: Pin,
    pub gnd: Pin,
    pub bi: Pin,
    pub ia: Pin,
    pub ib: Pin,
    pub ic: Pin,
    pub id: Pin,
    pub oa: Pin,
    pub ob: Pin,
    pub oc: Pin,
    pub od: Pin,
    pub oe: Pin,
    pub of: Pin,
    pub og: Pin,
}

impl SevenSegmentsDecoder {
    pub const VCC: usize = 14;
    pub const GND: usize = 7;
    pub const BI: usize = 3;
    pub const IA: usize = 5;
    pub const IB: usize = 1;
    pub const IC: usize = 2;
    pub const ID: usize = 4;
    pub const OA: usize = 11;
    pub const OB: usize = 10;
    pub const OC: usize = 9;
    pub const OD: usize = 8;
    pub const OE: usize = 6;
    pub const OF: usize = 13;
    pub const OG: usize = 12;
}

generate_chip!(
    SevenSegmentsDecoder,
    vcc: SevenSegmentsDecoder::VCC,
    gnd: SevenSegmentsDecoder::GND,
    bi: SevenSegmentsDecoder::BI,
    ia: SevenSegmentsDecoder::IA,
    ib: SevenSegmentsDecoder::IB,
    ic: SevenSegmentsDecoder::IC,
    id: SevenSegmentsDecoder::ID,
    oa: SevenSegmentsDecoder::OA,
    ob: SevenSegmentsDecoder::OB,
    oc: SevenSegmentsDecoder::OC,
    od: SevenSegmentsDecoder::OD,
    oe: SevenSegmentsDecoder::OE,
    of: SevenSegmentsDecoder::OF,
    og: SevenSegmentsDecoder::OG
);

impl ChipBuilder<ChipType> for SevenSegmentsDecoder {
    fn build() -> ChipType {
        ChipType::SevenSegmentDecoder(SevenSegmentsDecoder {
            vcc: Pin::from(PinType::Input),
            gnd: Pin::from(PinType::Output),
            bi: Pin::from(PinType::Input),
            ia: Pin::from(PinType::Input),
            ib: Pin::from(PinType::Input),
            ic: Pin::from(PinType::Input),
            id: Pin::from(PinType::Input),
            oa: Pin::from(PinType::Output),
            ob: Pin::from(PinType::Output),
            oc: Pin::from(PinType::Output),
            od: Pin::from(PinType::Output),
            oe: Pin::from(PinType::Output),
            of: Pin::from(PinType::Output),
            og: Pin::from(PinType::Output),
        })
    }
}

/// 4-bit (DCBA) to 7-bit (abcdefg) decoding lookup table
const SEG_DECODER_LUT: [u8; 16] = [
    0b1111110, 0b0110000, 0b1101101, 0b1111001, 0b0110011, 0b1011011, 0b1011111, 0b1110000,
    0b1111111, 0b1111011, 0b1110111, 0b0011111, 0b0001101, 0b0111101, 0b1001111, 0b1000111,
];

impl ChipRunner for SevenSegmentsDecoder {
    fn run(&mut self, _: std::time::Duration) {
        if self.vcc.state.as_logic(3.3).into() {
            self.gnd.state = State::Low;

            let output = if self.bi.state.as_logic(3.3).into() {
                let data = Pin::read_threshold(&[&self.ia, &self.ib, &self.ic, &self.id], 3.3);
                SEG_DECODER_LUT[data & 0xF]
            } else {
                0
            };

            Pin::write(
                &mut [
                    &mut self.og,
                    &mut self.of,
                    &mut self.oe,
                    &mut self.od,
                    &mut self.oc,
                    &mut self.ob,
                    &mut self.oa,
                ],
                output as usize,
            );
        }
    }
}
