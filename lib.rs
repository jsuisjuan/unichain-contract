#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod unichain_contract {
    use ink::{prelude::string::String, storage::traits::StorageLayout};
    use ink::storage::Mapping;
    use parity_scale_codec::{Decode, Encode};
    use scale_info::TypeInfo;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, Default, TypeInfo, StorageLayout)]
    pub enum FileType {
        Pdf,
        Docx,
        #[default]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo, StorageLayout)]
    pub struct File {
        id: u64,
        name: String,
        file_type: FileType,
        owner: AccountId,
    }

    #[ink(storage)]
    pub struct FileManagerContract {
        files: Mapping<u64, File>,
        next_file_id: u64,
    }

    impl FileManagerContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                files: Mapping::default(),
                next_file_id: 0,
            }
        }

        #[ink(message)]
        pub fn add_file(&mut self, name: String, file_type: FileType) -> u64 {
            let caller = self.env().caller();
            let file_id = self.next_file_id;
            let file = File {id: file_id, name, file_type, owner: caller};
            self.files.insert(&file_id, &file);
            self.next_file_id += 1;
            file_id
        }

        #[ink(message)]
        pub fn get_file(&self, file_id: u64) -> Option<File> {
            self.files.get(&file_id)
        }

        #[ink(message)]
        pub fn update_file(&mut self, file_id: u64, new_name: String, new_file_type: FileType) -> bool {
            self.files.get(&file_id)
                .filter(|file| self.env().caller() == file.owner)
                .map(|mut file| {
                    file.name = new_name;
                    file.file_type = new_file_type;
                    self.files.insert(&file_id, &file);
                    true })
                .unwrap_or(false)
        }

        #[ink(message)]
        pub fn delete_file(&mut self, file_id: u64) -> bool {
            self.files.get(&file_id)
                .filter(|file| self.env().caller() == file.owner)
                .map(|_| { self.files.remove(&file_id); true })
                .unwrap_or(false)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_env::test::set_caller;
        use ink_env::AccountId;

        #[ink::test]
        fn constructor_works() {
            let contract = FileManagerContract::new();
            assert_eq!(contract.next_file_id, 0);
        }

        #[ink::test]
        fn add_file_works() {
            let mut contract = FileManagerContract::new();
            let file_id = contract.add_file("test_file".to_string(), FileType::Pdf);
            assert_eq!(file_id, 0);
            let file = contract.get_file(file_id).unwrap();
            assert_eq!(file.name, "test_file");
            assert_eq!(file.file_type, FileType::Pdf);
            assert_eq!(file.id, 0);
        }

        #[ink::test]
        fn get_file_works() {
            let mut contract = FileManagerContract::new();
            let file_id = contract.add_file("example".to_string(), FileType::Pdf);
            let file = contract.get_file(file_id).unwrap();
            assert_eq!(file.name, "example");
            assert_eq!(file.file_type, FileType::Pdf);
        }

        #[ink::test]
        fn update_file_works() {
            let mut contract = FileManagerContract::new();
            let file_id = contract.add_file("old_name".to_string(), FileType::Docx);

            assert!(contract.update_file(file_id, "new_name".to_string(), FileType::Pdf));
            let updated_file = contract.get_file(file_id).unwrap();
            assert_eq!(updated_file.name, "new_name");
            assert_eq!(updated_file.file_type, FileType::Pdf);

            let new_caller = AccountId::from([0x02; 32]);
            set_caller::<ink_env::DefaultEnvironment>(new_caller);
            assert!(!contract.update_file(file_id, "hacker_name".to_string(), FileType::Docx));
        }

        #[ink::test]
        fn delete_file_works() {
            let mut contract = FileManagerContract::new();
            let file_id = contract.add_file("file_to_delete".to_string(), FileType::Pdf);

            assert!(contract.delete_file(file_id));
            assert!(contract.get_file(file_id).is_none());

            let new_file_id = contract.add_file("another_file".to_string(), FileType::Docx);
            set_caller::<ink_env::DefaultEnvironment>(AccountId::from([0x02; 32]));
            assert!(!contract.delete_file(new_file_id));
        }
    }

}
