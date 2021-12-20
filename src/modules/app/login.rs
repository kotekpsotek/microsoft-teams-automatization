/* This module contains BOT login feauture */
use enigo::{ Enigo, Key, KeyboardControllable };

use crate::*;

pub async fn login(driver: &GenericWebDriver<ReqwestDriverAsync>)
{
    let pass_data: MainConfigFile = read_data_from_conf_file(); // dane do logowania takie jak: hasło - passwd, email - email

    // Logowanie się do usługi office 365:
    let input_email_selector: By = By::Name("loginfmt");
    let input_password_selector: By = By::Name("passwd");
    let accept_email_buttons_selector: By = By::Id("idSIButton9");
    let accept_password_button_selector: By = By::Id("idSIButton9");

    // -- Podawanie adresu e-mail i jego akceptowanie
    let inptu_email_element: WebElement = driver.find_element(input_email_selector).await.expect(add_newline_characters("The field to enter the e-mail field could not be found!!!", 2, 2, "err").as_str());
    let button_accept_desctibed_email: WebElement = driver.find_element(accept_email_buttons_selector).await.expect(add_newline_characters("The button to accept the given e-mail address could not be found!!!", 2, 2, "err").as_str());

    inptu_email_element.send_keys(pass_data.email).await.expect(add_newline_characters("The e-mail address to the field intended for this could not be entered!!!", 2, 2, "err").as_str()); // wpisywanie hasła
    std::thread::sleep(std::time::Duration::from_millis(500)); // czekanie na podanie danych do pola <input type="email" name="loginfmt">
    button_accept_desctibed_email.click().await.expect(add_newline_characters("The e-mail address provided could not be validated!!!", 2, 2, "err").as_str()); // akceptowanie hasła
    
    // czekanie na dokonanie się akcji sprwadzenia poprwaności adresu e-mail
    std::thread::sleep(std::time::Duration::from_secs(7)); // czekanie na zmienienie się sekcji do podawania adres e-mail na sekcję do podawania 

    // Sprawdzanie czy element sugerujący że wystąpił błąd w zalogowywaniu suę istnieje
    if driver.find_element(By::Id("usernameError")).await.is_ok()
    {
        panic!("{}", add_newline_characters("You entered the wrong e-mail address for logging in.\nUpdate the e-mail address specified for the program", 2, 2, "err"));
    }
    
    // let _qte = driver.quit();
    // -- Podawanie hasła i jego akceptowanie
    let input_password_element = driver.find_element(input_password_selector).await.expect(add_newline_characters("The password field could not be found!!!", 2, 2, "err").as_str());
    let button_accept_described_password = driver.find_element(accept_password_button_selector).await.expect(add_newline_characters("Failed to find the button to validate the given password!!!", 2,2, "err").as_str());

    input_password_element.send_keys(pass_data.passwd).await.expect(add_newline_characters("The password for the password field could not be entered!!!", 2, 2, "err").as_str());
    std::thread::sleep(std::time::Duration::from_millis(500)); // czekanie na podanie danych do pola <input type="password" name="passwd">
    button_accept_described_password.click().await.expect(add_newline_characters("The user password you provided could not be validated!!!", 2, 2, "err").as_str());

    // Gdy pokazała się informacja, że zostało podane nieprwdiłowe hasło to program zostaje zakończony błędem nienaprwialnym panic!
    if driver.find_element(By::Id("passwordError")).await.is_ok()
    {
        panic!("{}", add_newline_characters("You entered a wrong password!!!\nUpdate the password specified for the program using command: config", 2, 2, "err"));
    }

    std::thread::sleep(std::time::Duration::from_millis(500)); // czekanie na zalogowanie się użytkownika
    
    // Jeżeli strona internetowa pyta nas czy chcemy zapmaiętywać adres e-mail to klika żeby nie zapamiętywała naszego e-maila oraz naszych danych osobistych
    if driver.find_element(By::Id("lightbox")).await.is_ok()
    {
        let no_display_that_menu_in_the_future_button: WebElement = driver.find_element(By::Id("KmsiCheckboxField")).await.expect(add_newline_characters("Unable to find a suitable item!!!", 2, 2, "err").as_str());
        let no_remember_user_button: WebElement = driver.find_element(By::Id("idBtn_Back")).await.expect(add_newline_characters("Unable to find a suitable item!!!", 2, 2, "err").as_str());
        no_display_that_menu_in_the_future_button.click().await.unwrap();
        no_remember_user_button.click().await.unwrap();
    }

    //& -- Czekanie na zamknięcie elementu, który pyta nas czy nie chcemy otworzyć aplikacji zamiast używać wersji przeglądarkowej
    std::thread::sleep(std::time::Duration::from_secs(10));
    let mut key_emitter: Enigo = Enigo::new();
    key_emitter.key_down(Key::Escape);

    //& -- Obłsuga kominikatu pytającego nas czy nie chcemy pobrać aplikacji ms.teams
    if driver.find_element(By::XPath("//*[@id=\"download-desktop-page\"]/div/a")).await.is_ok() 
    {
        let use_web_app_button: WebElement = driver.find_element(By::ClassName("use-app-lnk")).await.expect("Program coudn't found use web-app button!!!");
        use_web_app_button.click().await.expect("Program coudn't click on use web-app button!!!");
    }

    std::thread::sleep(std::time::Duration::from_secs(5)); // czeka 3 szekund na zalogowanie się na przejście użytkownika do elementu i zamyka element, który pyta czy użytkownik chce otrzymywać powiadmoenia na puplicie

    //& -- Usuwanie powiadmoneia "Trzymaj rękę na pulsie". Wyłącz powiadomienia na pulpicie
    if driver.find_element(By::XPath("//*[@id=\"toast-container\"]/div/div")).await.is_ok()
    {
        let try_turn_off_desktop_communications_button = driver.find_element(By::XPath("//*[@id=\"toast-container\"]/div/div/div[2]/div/button[2]/div")).await;
        if try_turn_off_desktop_communications_button.is_ok()
        {
            let turn_off_desktop_communications_button: WebElement = try_turn_off_desktop_communications_button.expect("Wtf!!! Program cound't find turn off desktop communications button!!!");
            turn_off_desktop_communications_button.click().await.expect("Program coudn't click on turn off desktop notifications button!!!");
        }
    }
}