use crate::BCDConversionError;

#[derive(Debug)]
pub struct BCD<const BYTES: usize> {
    data: [u8;BYTES]
}

pub trait Convertible<T> {
    fn convert(self) -> T;
}

impl<const BYTES_OG: usize, const BYTES_DST: usize> Convertible<BCD<BYTES_DST>> for BCD<BYTES_OG> {
    fn convert(self) -> BCD<BYTES_DST> {
        if BYTES_OG >= BYTES_DST {
            BCD {
                data: self.data.into_iter().rev().take(BYTES_DST).rev().collect::<Vec<u8>>().try_into().unwrap()
            }
        }
        else {
            let mut new_data = vec![0;BYTES_DST-BYTES_OG];
            new_data.append(&mut Vec::from(self.data));
            BCD {
                data: new_data.try_into().unwrap()
            }
        }
    }
}

impl<const BYTES: usize> IntoIterator for BCD<BYTES> {
    type Item = u8;

    type IntoIter = std::array::IntoIter<Self::Item,BYTES>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<const BYTES_OG: usize, const BYTES_DST: usize> From<[u8;BYTES_OG]> for BCD<BYTES_DST> {
    fn from(data: [u8;BYTES_OG]) -> Self {
        if BYTES_OG >= BYTES_DST {
            Self {
                data: data[BYTES_OG-BYTES_DST..].try_into().unwrap()
            }
        }
        else {
            let mut new_data = vec![0;BYTES_DST-BYTES_OG];
            new_data.append(&mut Vec::from(data));
            Self {
                data: new_data.try_into().unwrap()
            }
        }
    }
}

impl<const BYTES_OG: usize, const BYTES_DST: usize> From<BCD<BYTES_OG>> for [u8;BYTES_DST] {
    fn from(data: BCD<BYTES_OG>) -> Self {
        if BYTES_OG >= BYTES_DST {
            data.data.into_iter().rev().take(BYTES_DST).rev().collect::<Vec<u8>>().try_into().unwrap()
        }
        else {
            let mut new_data = vec![0;BYTES_DST-BYTES_OG];
            new_data.append(&mut Vec::from(data.data));
            new_data.try_into().unwrap()
        }
    }
}

impl TryFrom<u8> for BCD<1> {
    type Error = BCDConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = vec![();n_bytes].into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u8", value, max_val)); }
        let high = value/10;
        let low = value%10;
        Ok(Self{data: [high<<4|low]})
    }
}

impl From<BCD<1>> for u8 {
    fn from(val: BCD<1>) -> Self {
        let high = (val.data[0] & 0xf0) >> 4;
        let low  = val.data[0] & 0x0f;
        high*10+low
    }
}

impl TryFrom<u16> for BCD<2> {
    type Error = BCDConversionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = vec![();n_bytes].into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u16", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = value%100;
            buffer.push((current as u8).try_into()?);
            value /= 100;
        }
        while buffer.len() < n_bytes {
            buffer.push(BCD{ data: [0] });
        }
        let buffer: [u8; 2] = buffer.into_iter()
            .rev()
            .map(|item| item.data)
            .flatten()
            .collect::<Vec<u8>>()
            .try_into().unwrap();
        Ok(Self::from(buffer))
    }
}

impl From<BCD<2>> for u16 {
    fn from(value: BCD<2>) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100+(x as Self))
    }
}

impl TryFrom<u32> for BCD<4> {
    type Error = BCDConversionError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = vec![();n_bytes].into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u32", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = value%100;
            buffer.push((current as u8).try_into()?);
            value /= 100;
        }
        while buffer.len() < n_bytes {
            buffer.push(BCD{ data: [0] });
        }
        let buffer: [u8; 2] = buffer.into_iter()
            .rev()
            .map(|item| item.data)
            .flatten()
            .collect::<Vec<u8>>()
            .try_into().unwrap();
        Ok(Self::from(buffer))
    }
}

impl From<BCD<4>> for u32 {
    fn from(value: BCD<4>) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100 + (x as Self))
    }
}

impl TryFrom<u64> for BCD<8> {
    type Error = BCDConversionError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = vec![();n_bytes].into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u32", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = value%100;
            buffer.push((current as u8).try_into()?);
            value /= 100;
        }
        while buffer.len() < n_bytes {
            buffer.push(BCD{ data: [0] });
        }
        let buffer: [u8; 2] = buffer.into_iter()
            .rev()
            .map(|item| item.data)
            .flatten()
            .collect::<Vec<u8>>()
            .try_into().unwrap();
        Ok(Self::from(buffer))
    }
}

impl From<BCD<8>> for u64 {
    fn from(value: BCD<8>) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100 + (x as Self))
    }
}

impl TryFrom<u128> for BCD<16> {
    type Error = BCDConversionError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        let n_bytes = value.to_ne_bytes().into_iter().count();
        let max_val = vec![();n_bytes].into_iter().fold(0, |acc, _| acc*100+99);
        if value > max_val { return Err(Self::Error::new_with_template_description("u32", value, max_val)); }
        let mut value = value;
        let mut buffer: Vec<BCD<1>> = vec![];
        while value != 0 {
            let current = value%100;
            buffer.push((current as u8).try_into()?);
            value /= 100;
        }
        while buffer.len() < n_bytes {
            buffer.push(BCD{ data: [0] });
        }
        let buffer: [u8; 2] = buffer.into_iter()
            .rev()
            .map(|item| item.data)
            .flatten()
            .collect::<Vec<u8>>()
            .try_into().unwrap();
        Ok(Self::from(buffer))
    }
}

impl From<BCD<16>> for u128 {
    fn from(value: BCD<16>) -> Self {
        value.data.into_iter()
            .fold(0, |acc, x| acc*100 + (x as Self))
    }
}