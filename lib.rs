#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod layout {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Layout {
        value_0_32: u32,
        value_0_128: u128,
        value_0_arr: [u8; 32],
        map_0_string: Mapping<String, u32>,
        map_0_u32: Mapping<u32, u32>,
        data: Data,
    }

    #[derive(Debug)]
    #[ink::storage_item]
    pub struct Data {
        value_1_32: u32,
        value_1_128: u128,
        value_1_arr: [u8; 32],
        map_1_string: Mapping<String, u32>,
        map_1_u32: Mapping<u32, u32>,
        pub _reserved: Option<()>,
    }

    impl Layout {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                value_0_32: 11,
                value_0_128: 11,
                value_0_arr: [1; 32],
                map_0_string: Default::default(),
                map_0_u32: Default::default(),
                data: Data {
                    value_1_32: 22,
                    value_1_128: 22,
                    value_1_arr: [2; 32],
                    map_1_string: Default::default(),
                    map_1_u32: Default::default(),
                    _reserved: None,
                },
            }
        }

        #[ink(message)]
        pub fn init(&mut self) {
            self.map_0_string.insert(
                "abc",
                &11,
            );
            self.map_0_string.insert(
                "aaaaaaaaa",
                &111,
            );
            self.map_0_u32.insert(111, &11);
            self.data
                .map_1_string
                .insert("bbbbbbbbbbbbbbbbbbbbbbbbbbbb", &22);
            self.data.map_1_u32.insert(222, &22);
        }
    }
}
