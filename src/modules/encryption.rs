use rand::{self, Rng };
use crate::add_newline_characters;

use super::config::Config;
use std::path::Path;
use std::fs; // odczyty i zapis plików
use crypto:: { // tworzenie silnych kluczy oraz szyfrowanie danych (używany tylko do generowania klucza)
    self,
    pbkdf2::pbkdf2_simple,
};
use magic_crypt::{ new_magic_crypt, MagicCrypt256, MagicCryptTrait };

#[derive(Debug)]
pub struct EncryptedData {
    pub enc_password: String,
    pub enc_email: String,
}

#[derive(Debug)]
pub struct DecryptedData {
    pub dec_password: String,
    pub dec_email: String,
}

// tworzenie i zapisywanie klucza do szyfrowania danych w pliku komputera -> zwraca ona zaszyfrowany klucz
fn generate_save_and_resturn_key() -> String
{
    /*& ZMIENNE ZAWIERAJĄCE INFORMACJE KONFIGURACYJNE O MIEJSCU GDZIE MA ZOSTAĆ ZAPISANY FOLDER */
    let config_paths: Config = Config::new();
    let save_key_main_folder: String = config_paths.program_files_folder;
    let save_key_path: String = config_paths.this_program_folder;

    /*& FUNKCJA, KTÓREJ ZADANIEM JEST WYGENEROWANIE NOWEGO KLUCZA */
    fn generate_new_key() -> String
    {
        /*& GENEROWANIE HASŁA: */
        // baza znaków do utworzenia hasła dla klucza
        let charcter_base: Vec<&str> = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890-=[';!-=~`@#$%^&*()".split("").collect::<Vec<&str>>();
        // vector z gotowymi znakami, które mają utworzyć hasło
        let mut ready_password: Vec<&str> = Vec::new();
        for _ in 0..40
        {
            // Randomowy Numer generowany w celu wyciągnięcia jakiegoś znaku z bazy znaków (0..ilość_znaków_w_bazie)
            let random_number: usize = rand::thread_rng().gen_range(0..charcter_base.len());
            // Wyłanianie randomowego zaznaczonego znaku z bazy znaków
            let selected_character: &str = charcter_base[random_number];
            // Dodawanie zaznaczonego znaku do bazdy znaków z, których ma być utworzone hasło
            ready_password.push(selected_character);
        };
        // Tworzenie stringa z zaznaczonego hasła
        let password: String = ready_password.join("");

        /*& TWORZENIE KLUCZA */
        let cipher_key: String = pbkdf2_simple(password.as_str(), 10_000).expect(add_newline_characters("Program can't generate new key!!!", 2, 2, "err").as_str());
        cipher_key
    }

    fn return_key(save_key_main_folder: String, save_key_path: String) -> String
    {
        if Path::new(&save_key_main_folder).exists() // gdy folder /Programs już istnieje
        {
            if Path::new(&save_key_path).exists() // gdy folder /Programs/teams-automatization już istnieje
            {
                let key_file_name: &str = "secret_key__.key";
                let file_path: String = format!("{}/{}", save_key_path, key_file_name); // paczka do pliku z kluczem
                if Path::new(&file_path).exists() // gdy plik z kluczem już istnieje
                {
                    // OCZYTYWANIE KLUCZA Z PLIKU I SPRAWDZANIE CZY ZOSTAŁ ON PODANY
                    let key_from_file: String = fs::read_to_string(&file_path).expect(add_newline_characters("Program can't read file with encryption key. Please try again!!!", 2, 2, "err").as_str());
                    
                    // SPRAWDZANIE CZY ODCZYTANY KLUCZ JEST POPRWANY: jeżeli jest to go zwraca
                    if key_from_file.len() > 30 // gdy odczytany klucz ma odpowiednią wielkość
                    {
                        key_from_file
                    }
                    else // Gdy klucz nie ma odpowiedniej wielkości generowany jest nowy klucz
                    {
                        // Nowy klucz
                        let new_key: String = generate_new_key();
                        // Zapisywanie nowego klucza
                        fs::write(file_path, &new_key).expect(add_newline_characters("Program can't save new generated key!!!", 2, 2, "err").as_str());
                        new_key
                    }
                }
                else
                {
                    /*& GENEROWANIE NOWEGO KLUCZA I ZAPISYWANIE GO W PLIKU Z KLUCZEM */
                    // Nowy klucz
                    let new_key: String = generate_new_key();
                    // Zapisywanie nowego klucza
                    fs::write(file_path, &new_key).expect(add_newline_characters("Program can't save new generated key!!!", 2, 2, "err").as_str());
                    new_key
                }
            }
            else
            {
                fs::create_dir(&save_key_path).expect(add_newline_characters("Program can't create required directory", 2, 2, "err").as_str()); // tworzenie biblioteki /Programs/teams-automatization
                return_key(save_key_main_folder, save_key_path)
            }
        }
        else
        {
            fs::create_dir(&save_key_main_folder).expect(add_newline_characters("Program can't create required directory", 2, 2, "err").as_str()); // tworzenie biblioteki /Programs
            return_key(save_key_main_folder, save_key_path)
        }
    }

    return_key(save_key_main_folder, save_key_path)
}

// szyfrowanie podanych danych: zwraca (zaszyfrowany_email, zaszyfrowane_hasło)
pub fn encrypt_data(email: String, password: String) -> EncryptedData
{
    let key: String = generate_save_and_resturn_key();
    let encryption_b: MagicCrypt256 = new_magic_crypt!(key, 256);

    let enc_email: String = encryption_b.encrypt_str_to_base64(email);
    let enc_password: String = encryption_b.encrypt_str_to_base64(password);

    EncryptedData {
        enc_email,
        enc_password
    }
}

// deszyfrowanie podanych danych
pub fn decrypt_data(email: String, password: String) -> DecryptedData
{
    let key: String = generate_save_and_resturn_key();
    let encryption_b: MagicCrypt256 = new_magic_crypt!(key, 256);

    let dec_email: String = encryption_b.decrypt_base64_to_string(email).expect(add_newline_characters("Program can't decrypt email. Please setup your application again using command: setup", 2, 2, "err").as_str());
    let dec_password: String = encryption_b.decrypt_base64_to_string(password).expect(add_newline_characters("Program can't decrypt password. Please setup your application again using command: setup", 2, 2, "err").as_str());

    DecryptedData {
        dec_email,
        dec_password
    }
}