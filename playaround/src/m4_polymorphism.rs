use ethers::types::Address;
use std::str::FromStr;

trait EthereumAddress{
    fn convert_addresss(&self) -> Result<Address, &'static str>;
}

impl EthereumAddress for &str{
    fn convert_addresss(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) =>  Ok(address),
            Err(_) => Err("Invaloid Ethereum Addtres String")
        }
    }
}

impl EthereumAddress for Address {
    fn convert_addresss(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_etherenum_data<T: EthereumAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_addresss().unwrap();
    converted_address
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_poly() {
        let addr = Address::from_str("0x4B3883cCE3313Ed4445a897355032c273fd87A6f")
        .unwrap();

        let new_addr: Address = get_etherenum_data(addr);
        assert_eq!(new_addr, Address::from_str("0x4B3883cCE3313Ed4445a897355032c273fd87A6f").unwrap());
    
        let new_addr: Address = get_etherenum_data("0x4B3883cCE3313Ed4445a897355032c273fd87A6f");
        assert_eq!(new_addr, Address::from_str("0x4B3883cCE3313Ed4445a897355032c273fd87A6f").unwrap());
    }
}

