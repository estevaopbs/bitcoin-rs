use crate::finite_field::FieldElement;
use bnum::types::I512;

#[allow(dead_code)]
pub fn s256_field_element(num: I512) -> Result<FieldElement, String> {
    let prime: I512 = (I512::from(2)).pow(256) - (I512::from(2)).pow(32) - I512::from(977);
    FieldElement::new(num, prime)
}
