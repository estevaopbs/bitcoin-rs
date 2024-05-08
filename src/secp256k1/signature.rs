use crate::secp256k1::FieldElement;
use bnum::types::U256;

#[derive(Debug)]
pub struct Signature {
    r: FieldElement,
    s: FieldElement,
}

impl Signature {
    #[allow(dead_code)]
    pub fn new(r: U256, s: U256) -> Self {
        Self::from_field_elements(FieldElement::new(r), FieldElement::new(s))
    }

    pub fn from_field_elements(r: FieldElement, s: FieldElement) -> Self {
        Self { r, s }
    }

    pub fn r(&self) -> FieldElement {
        self.r
    }

    pub fn s(&self) -> FieldElement {
        self.s
    }
}
