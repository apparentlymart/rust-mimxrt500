use super::*;
use super::pins::PinSet;

pub struct FlexcommUart<FC: WithUsart, PS: PinSet<FC>> {
    _fc: FC,
    _ps: PS,
    tx: PS::TxdSclMisoFramePin,
    rx: PS::RxdSdaMosiDataPin,
}

impl<FC: WithUsart, PS: PinSet<FC>> FlexcommUart<FC, PS> {
    pub fn new(flexcomm: FC, pin_set: PS, tx: PS::TxdSclMisoFramePin, rx: PS::RxdSdaMosiDataPin) -> Self {
        Self {
            _fc: flexcomm,
            _ps: pin_set,
            tx, rx,
        }
    }
}
