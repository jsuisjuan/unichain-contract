#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod unichain_contract {
    use ink::prelude::string::String;
    use ink::storage::Mapping;
    use ink::storage::traits::StorageLayout;
   
    #[derive(Debug, PartialEq, Default)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub enum FileType {
        Pdf,
        Docx,
        Xls,
        Txt,
        Csv,
        Pptx,
        Jpg,
        Png,
        #[default]
        Unknown,
    }

    #[derive(Debug, PartialEq)]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    pub struct File {
        id: u64,
        name: String,
        file_type: FileType,
        size: u64,
        description: String,
        timestamp: u64,
        owner: AccountId,
    }

    #[ink(storage)]
    pub struct FileManagerContract {
        files: Mapping<u64, File>,
        next_file_id: u64,
    }

    impl Default for FileManagerContract {
        fn default() -> Self {
            Self::new()
        }
    }

    impl FileManagerContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                files: Mapping::default(),
                next_file_id: 0,
            }
        }

        fn increment_file_id(&mut self) -> u64 {
            let next_id = self.next_file_id;
            self.next_file_id = next_id
                .checked_add(1)
                .expect("Overflow ao incrementar next_file_id");
            next_id
        }

        #[ink(message)]
        pub fn add_file(
            &mut self, 
            name: String, 
            size: u64,
            description: String, 
            file_type: FileType
        ) -> u64 {
            let caller = self.env().caller();
            let file_id = self.increment_file_id();
            let file = File { 
                id: file_id, 
                name, 
                file_type, 
                size,
                description,
                timestamp: self.env().block_timestamp(), 
                owner: caller
            };
            self.files.insert(file_id, &file);
            file_id
        }

        #[ink(message)]
        pub fn get_file(&self, file_id: u64) -> Option<File> {
            self.files.get(file_id)
        }

        #[ink(message)]
        pub fn update_file(
            &mut self, 
            file_id: u64, 
            new_name: String,
            new_size: u64,
            new_description: String, 
            new_file_type: FileType
        ) -> bool {
            self.files.get(file_id)
                .filter(|file| self.env().caller() == file.owner)
                .map(|mut file| {
                    file.name = new_name;
                    file.size = new_size;
                    file.description = new_description;
                    file.file_type = new_file_type;
                    self.files.insert(file_id, &file);
                    true 
                })
                .unwrap_or(false)
        }

        #[ink(message)]
        pub fn delete_file(&mut self, file_id: u64) -> bool {
            self.files.get(file_id)
                .filter(|file| self.env().caller() == file.owner)
                .map(|_| { self.files.remove(file_id); true })
                .unwrap_or(false)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn constructor_works() {
            let contract = FileManagerContract::new();
            assert_eq!(contract.next_file_id, 0);
        }

        #[ink::test]
        fn add_file_works() {
            let mut contract = FileManagerContract::new();
            let file_id = contract.add_file(
                "test_file".to_string(), 
                200, 
                "little file description".to_string(), 
                FileType::Pdf
            );
            assert_eq!(file_id, 0);
            let file = contract.get_file(file_id).unwrap();
            assert_eq!(file.name, "test_file");
            assert_eq!(file.file_type, FileType::Pdf);
            assert_eq!(file.id, 0);
        }

        #[ink::test]
        fn get_file_works() {
            let mut contract = FileManagerContract::new();
            let file_id = contract.add_file(
                "example".to_string(), 
                200, 
                "little file description".to_string(), 
                FileType::Pdf
            );
            let file = contract.get_file(file_id).unwrap();
            assert_eq!(file.name, "example");
            assert_eq!(file.file_type, FileType::Pdf);
        }

        #[ink::test]
        fn update_file_works() {
            let mut contract = FileManagerContract::new();
            let file_id = contract.add_file(
                "old_name".to_string(), 
                200, 
                "little file description".to_string(),  
                FileType::Docx
            );
            assert!(contract.update_file(
                file_id, 
                "new_name".to_string(), 
                500, 
                "new file description".to_string(), 
                FileType::Pdf)
            );
            let updated_file = contract.get_file(file_id).unwrap();
            assert_eq!(updated_file.name, "new_name");
            assert_eq!(updated_file.file_type, FileType::Pdf);
        }

        #[ink::test]
        fn delete_file_works() {
            let mut contract = FileManagerContract::new();
            let file_id = contract.add_file(
                "file_to_delete".to_string(), 
                200, 
                "little file description".to_string(),  
                FileType::Pdf
            );
            assert!(contract.delete_file(file_id));
            assert!(contract.get_file(file_id).is_none());
        }
    }

}
