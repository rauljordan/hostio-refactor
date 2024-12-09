#![no_std]

mod hostio;

use hostio::wrap_hostio;

wrap_hostio! {
    ink_price INK_PRICE tx_ink_price u32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ink_price() {
        assert_eq!(ink_price(), 5u32);
        INK_PRICE.set(10u32);
        assert_eq!(ink_price(), 10u32);
    }
}
