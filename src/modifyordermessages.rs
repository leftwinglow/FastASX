use crate::types::{BinaryMessageLength, Parse, ParseError};
use crate::{helpers::byte_to_bool, messageheader::MessageHeader};
use byteorder::{BigEndian, ByteOrder};

#[cfg(test)]
use crate::types::{EnumTestHelpers, GenerateBinaryExample};
#[cfg(test)]
use fastrand::Rng;

#[derive(Debug, PartialEq)]
pub struct OrderExecuted {
    header: MessageHeader,
    order_reference_number: u64,
    executed_shares: u32,
    match_number: u64, // matches trade break message
}

impl Parse for OrderExecuted {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(OrderExecuted {
            header: MessageHeader::parse(&input[..10]),
            order_reference_number: BigEndian::read_u64(&input[10..18]),
            executed_shares: BigEndian::read_u32(&input[18..22]),
            match_number: BigEndian::read_u64(&input[22..30]),
        })
    }
}

impl BinaryMessageLength for OrderExecuted {
    const LENGTH: usize = 30;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for OrderExecuted {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_example_message();
        let order_reference_number = rng.u64(..).to_be_bytes();
        let executed_shares = rng.u32(..).to_be_bytes();
        let match_number = rng.u64(..).to_be_bytes();

        let mut message = [0; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&order_reference_number);
        message[18..22].copy_from_slice(&executed_shares);
        message[22..30].copy_from_slice(&match_number);

        message
    }
}

#[derive(Debug, PartialEq)]
pub struct OrderExecutedWithPrice {
    order_executed_message: OrderExecuted,
    printable: bool,
    exec_price: u32,
}

impl Parse for OrderExecutedWithPrice {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(OrderExecutedWithPrice {
            order_executed_message: OrderExecuted::parse(&input[..30])
                .expect("Failed to parse OrderExecutedWithPrice: Invalid order_executed header."),
            printable: byte_to_bool(input[30]),
            exec_price: BigEndian::read_u32(&input[31..35]),
        })
    }
}

impl BinaryMessageLength for OrderExecutedWithPrice {
    const LENGTH: usize = 35;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for OrderExecutedWithPrice {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let order_executed_message = OrderExecuted::generate_example_message();
        let printable = b'Y';
        let exec_price = rng.u32(..).to_be_bytes();

        let mut message = [0; Self::LENGTH];
        message[..30].copy_from_slice(&order_executed_message);
        message[30] = printable;
        message[31..35].copy_from_slice(&exec_price);

        message
    }
}

#[derive(Debug, PartialEq)]
pub struct OrderCancel {
    header: MessageHeader,
    order_reference_number: u64,
    canceled_shares: u32,
}

// Byte layout:
// 0-9: Header (10 bytes)
// 10-17: Order Reference Number (8 bytes)
// 18-21: Canceled Shares (4 bytes)
// Total: 23 bytes
impl Parse for OrderCancel {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(OrderCancel {
            header: MessageHeader::parse(&input[..10]),
            order_reference_number: BigEndian::read_u64(&input[10..18]),
            canceled_shares: BigEndian::read_u32(&input[18..22]),
        })
    }
}

impl BinaryMessageLength for OrderCancel {
    const LENGTH: usize = 22;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for OrderCancel {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_example_message();
        let order_reference_number = rng.u64(..).to_be_bytes();
        let canceled_shares = rng.u32(..).to_be_bytes();

        let mut message = [0; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&order_reference_number);
        message[18..22].copy_from_slice(&canceled_shares);

        message
    }
}

#[derive(Debug, PartialEq)]
pub struct OrderDelete {
    header: MessageHeader,
    order_reference_number: u64,
}

impl Parse for OrderDelete {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(OrderDelete {
            header: MessageHeader::parse(&input[..10]),
            order_reference_number: BigEndian::read_u64(&input[10..18]),
        })
    }
}

impl BinaryMessageLength for OrderDelete {
    const LENGTH: usize = 18;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for OrderDelete {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_example_message();
        let order_reference_number = rng.u64(..).to_be_bytes();

        let mut message = [0; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&order_reference_number);

        message
    }
}

#[derive(Debug, PartialEq)]
pub struct OrderReplace {
    header: MessageHeader,
    original_order_reference_number: u64,
    new_order_reference_number: u64, // Assert old order dropped?
    shares: u32,
    price: u32,
}

impl Parse for OrderReplace {
    fn parse(input: &[u8]) -> Result<Self, ParseError> {
        if input.len() != Self::LENGTH {
            return Err(ParseError::IncompleteMessage {
                expected: Self::LENGTH,
            });
        }

        Ok(OrderReplace {
            header: MessageHeader::parse(&input[..10]),
            original_order_reference_number: BigEndian::read_u64(&input[10..18]),
            new_order_reference_number: BigEndian::read_u64(&input[18..26]),
            shares: BigEndian::read_u32(&input[26..30]),
            price: BigEndian::read_u32(&input[30..34]),
        })
    }
}

impl BinaryMessageLength for OrderReplace {
    const LENGTH: usize = 34;
}

#[cfg(test)]
impl GenerateBinaryExample<{ Self::LENGTH }> for OrderReplace {
    fn generate_example_message() -> [u8; Self::LENGTH] {
        let mut rng = Rng::new();

        let header = MessageHeader::generate_example_message();
        let original_order_reference_number = rng.u64(..).to_be_bytes();
        let new_order_reference_number = rng.u64(..).to_be_bytes();
        let shares = rng.u32(..).to_be_bytes();
        let price = rng.u32(..).to_be_bytes();

        let mut message = [0; Self::LENGTH];
        message[..10].copy_from_slice(&header);
        message[10..18].copy_from_slice(&original_order_reference_number);
        message[18..26].copy_from_slice(&new_order_reference_number);
        message[26..30].copy_from_slice(&shares);
        message[30..34].copy_from_slice(&price);

        message
    }
}
