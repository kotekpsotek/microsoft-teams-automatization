const CONGIG_PATH: &str = "./config";

// Application modules
mod modules {
    pub mod encryption;
    pub mod config;
    pub mod app {
        pub mod calendar;
        pub mod dependencies;
        pub mod login;
    }
}

use std::{ path::Path, fs::{ self, File }, process::Command };
use { serde, serde_json };
use serde::{ Deserialize, Serialize };
use tokio;
use clap::{ App, Arg, SubCommand };
use question::{ Question, Answer };
use indicatif::ProgressBar;
use thirtyfour::{GenericWebDriver, WebDriverCommands, WebElement, http::reqwest_async::ReqwestDriverAsync, prelude::{ WebDriver, DesiredCapabilities, By } };
use colored::Colorize;
use ctrlc;
use chrono;
use modules::{ app::{ calendar::callendar, dependencies::{ TeamsCalendaryMeeting, NoJoinToThisMeetings, BannedMeetingCollection, BannedMeetingsConfig, BannedMeetingsConfigTime, BannedMeetingsConfigTrybe }, login::login }, encryption::{ EncryptedData, DecryptedData, encrypt_data, decrypt_data } };


// struktóra do zapisywania i odczytywania danych konfiguracyjnych
#[derive(Debug, Serialize, Deserialize)]
struct MainConfigFile {
    passwd: String,
    email: String,
    encrypted: bool
}

// HITWS: Funkcja ta dodaje do podanego stringa znaki newline i zwraca ten string
fn add_newline_characters(r#type: &str, how_patch_add_to_start: u8, how_patch_add_to_end: u8, colored: &str) -> String
{
    let rplac: String = r#type.trim().to_string();
    let mut add_to_end: Vec<&str> = rplac.split("").collect::<Vec<&str>>();
    // dodawnaie new line charcters
        // Dodawanie znaków na koniec vectora ze stringa
    for _ in 0..how_patch_add_to_end
    {
        add_to_end.push("\n");
    }
        // Odwracanie vectora w celu 
    add_to_end.reverse();
        // Dodawanie znaku newline na początek vectora ze stringa
    for _ in 0..how_patch_add_to_start
    {
        add_to_end.push("\n");
    }
        // Przywracanie odpowiedniej kolejności Vectora
    add_to_end.reverse();
        // Konwertowanie utworzonego vectora spowrottem do postaci stringa
    let mut fn_result: String = add_to_end.join("");
    
    // ustawianie odpowiedniego koloru
    if colored == "suc"
    {
        fn_result = fn_result.green().bold().to_string();
        fn_result
    }
    else if colored == "err"
    {
        fn_result = fn_result.red().bold().to_string();
        fn_result
    }
    else if colored == "none"
    {
        fn_result
    }
    else
    {
        panic!("{}", add_newline_characters("Something went wrong!!!", 2, 2, "err"));
    }
}

// Odczytywanie danych z pliku konfiguracyjnego: hasło/adres e-mail itp...
fn read_data_from_conf_file() -> MainConfigFile
{
    // tworzy plik i zwraca odpowiednią strukturę
    fn create_conf_file(main_conf_file_with_path: &String) -> MainConfigFile
    {
        let _created_file: File = File::create(main_conf_file_with_path).expect(add_newline_characters("The configuration file could not be created!!!", 2, 2, "err").as_str());
        MainConfigFile {
            passwd: "nil".to_string(),
            email: "nil".to_string(),
            encrypted: false
        }
    }

    let main_conf_file_with_path: String = format!("{}/main.config.json", CONGIG_PATH);
    // jeżeli folder na plik konfiguracyjny istnieje
    if Path::new(CONGIG_PATH).exists()
    {
        // Jeżeli główny plik konfiguracyjny istnieje
        if Path::new(&main_conf_file_with_path).exists()
        {
            let file_data_str: String = fs::read_to_string(&main_conf_file_with_path).expect(add_newline_characters("Failed to read data from the file!!!", 2, 2, "err").as_str()).trim().to_string();
            let json_file_data: MainConfigFile = serde_json::from_str(file_data_str.as_str()).expect(add_newline_characters("The contents of the configuration file could not be read!!!", 2, 2, "err").as_str());
            
            if json_file_data.encrypted == false // w momencie gdy dane nie zostały zaszyfrowane
            {
                json_file_data
            }
            else if json_file_data.encrypted == true // w momencie gdy dane zostały zaszyfrowane
            {
                // Rozszyfrowywanie i zwracanie rozszyfrowanych danych
                let DecryptedData { dec_email, dec_password } = decrypt_data(json_file_data.email, json_file_data.passwd);
                MainConfigFile {
                    passwd: dec_password,
                    email: dec_email,
                    encrypted: true
                }
            }
            else // gdy została podana jakaś inna wartość to usuwa plik konfiguracyjny/tworzy nowy z pustymi danymi
            {
                let empty_scheam: MainConfigFile = MainConfigFile {
                    passwd: "nil".to_string(),
                    email: "nil".to_string(),
                    encrypted: false
                };
                let to_json_string: String = serde_json::to_string(&empty_scheam).expect(add_newline_characters("Program can't serialize added config data. Please try again!!!", 2, 2, "err").as_str());
                fs::write(main_conf_file_with_path, to_json_string).expect(add_newline_characters("Program can't save configuration data into file!!!", 2, 2, "err").as_str());
                println!("{}", add_newline_characters("Failed to read data from the configuration file!!!\nConfigure the application again using the command: config", 2, 2, "err"));
                std::process::exit(0);
            }
        }
        else
        {
            create_conf_file(&main_conf_file_with_path)
        }
    }
    else
    {
        fs::create_dir(Path::new(CONGIG_PATH)).expect(add_newline_characters("The library for the configuration files could not be created!!!", 2, 2, "err").as_str());
        create_conf_file(&main_conf_file_with_path)
    }
}

// tutaj znajduje się ciało drivera = w celu odpalenia aplikacji trzeba wywołać tą funkcję
async fn application_main()
{
    // funkcja ta odpala sterownik dla przeglądarki i czeka jakąś ilośc czasu na jego odpalenie
    async fn run_driver()
    {
        // Odpalanie sterownika dla przegladarki
        let mut run_chromedriver = Command::new("powershell");
        run_chromedriver.args(&["&", r".\drivers\chromedriver.exe", "--port=4444"]);
        run_chromedriver.spawn().expect(add_newline_characters("Failed to read the file...", 2, 2, "err").as_str());
        // println!("{}", add_newline_characters("\n\n\nOczekiwanie na odpalenie sterownika dla przeglądarki!!!\n\n\n", 2, 2));
        std::thread::sleep(std::time::Duration::from_millis(500));
        
        // Progress bar informujący uzytkownika o tym, że jest właśnie odpalany sterownik do aplikacji
        let bar = ProgressBar::new(10);
        bar.println(add_newline_characters("Waiting for the browser driver to fire...", 2, 2, "none")); // wiadomośc nad barem o tym że aplikacja została włączona
        // Powiększanie paska postepu na barze
        for _  in 0..10
        {
            bar.inc(1);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        bar.finish_and_clear();
    }

    // Czekanie na odpalenie webdrivera dla aplikacji
    run_driver().await; 
    println!("{}", add_newline_characters("The driver was fired!!!", 2, 2, "suc"));

    // Odpalanie części aplikacji związanej ze stroną internetową
    println!("{}", add_newline_characters("The application has been launched...", 2, 2, "suc")); // wyświetlanie informacji
    let mut capabilities = DesiredCapabilities::chrome();
    capabilities.add_chrome_arg("--disable-default-apps").expect("Program coudn't add \"disable-default-apps\" argument to launched application!!!");
    capabilities.add_chrome_arg("use-fake-ui-for-media-stream").expect("Program coudn't add an \"fake-ui\" argument!!!");
    capabilities.add_chrome_arg("start-maximized").expect("Program coudn't add an \"window-maximized\" argument!!!");
    let driver = WebDriver::new("http://localhost:4444", &capabilities).await.unwrap();

    // Łączenie się ze stroną logowania do pakietu office 365:
    driver.get("https://go.microsoft.com/fwlink/p/?LinkID=873020&culture=en-us&country=WW&lm=deeplink&lmsrc=homePageWeb&cmpid=WebSignIn").await.expect(add_newline_characters("Could not connect to ms.teams!!! -- Reason: Bad request URL address", 2, 2, "err").as_str());
    std::thread::sleep(std::time::Duration::from_secs(10)); // czekanie określoną ilośc czasu na zaladowanie strony internetowej
    login(&driver).await; // akcja logowania sie użytkownika
    std::thread::sleep(std::time::Duration::from_secs(15)); // czekanie na zalogowanie się użytkownika time: 15 sekund (20 sekund - 5 sekund czekania na przejście do panelu głownego ms.teams i zamknięcie zapytania czy użytkownik chce otrzymywać powiadomienia na pulpicie)
    callendar(&driver).await; // logowanie się na lekcje z wykorzystaniem kalendarza
}

#[tokio::main]
async fn main()
{
    // Działa tylko w momencie gdy aplikacja jest odpalana na systemach z rodziny Windows
    if cfg!(target_os = "windows")
    {
        // Linia wiersza poleceń w celu odpalenia aplikacji
        let app = App::new("Teams Automatization Tool")
        .version("1.0")
        .author("SiematuMichael@protonmail.com")
        .about("This application is the automatization tool for login in Microsoft Teams Web Application and joining to the Microsoft Teams meeting without user interference, but user can turn on/off camera, mike and using other ms.teams feautures when he is in meettings")
        .arg(Arg::with_name("run") // argument odpalajacy program
                    .long("run")
                    .short("r")
                    .help("Use this argument to run the program")
                    .long_help("This command run the application. If you haven't provided configuration data, the program will end with an error. Use config command to configure your application if you don't do that ever or if you would like to change data")
        )
        .subcommand(SubCommand::with_name("run")
                                        .version("1.0")
                                        .help("Use this command to run the application")
                                        .help_message("Use this command to run the application")
                                        .usage("teams-automatization run")
        )
        .subcommand(SubCommand::with_name("config")
                                        .version("1.0")
                                        .help("Use this command to configure the application. How to use this command: config [a1] [a2]\n\nDescription:\n\n[a1] - this is the argument with first order config subfunction. You can use in this place keywords like: [meetings/m] - use that for configure banned meetings (banned meetings are the meetings where bot can't join)\n\n[a2] - this is the additional function which work with [a1] first order argument. This feature only work when [a1] is defined for command and [a2] functions are defined for [a1]. Use command: command [a1] list to see what commands you have for your disposition to usage in [a2] argument place!!!\n\n\nCommands:\n\nmeetings | m - this command allow you to configure your banned meetings list!!! Enter: [config m help] to show banned meetings configuration commands!!!")
                                        .usage("teams-automatization config")
                                        .arg(Arg::with_name("meet")
                                            /* .index(1)
                                            .required(true) */
                                            .help("Use this argument to execute additional functions of config command!!!")
                                            .long_help("Use this argument to execute additional functions of config command!!!")
                                        ) // -- dodwanie spotkań na, które ma nie dołączać bot
                                        .arg(Arg::with_name("type")
                                            .help("Use this argument to execute additional functions of meet command argument!!!")
                                            .long_help("Use this argument to execute additional functions of meet command argument!!!")
                                        ) // -- Dodawanie dodatkowych opcji dla spotkań np: wyświwtlanie konfiguracji zbanowanych spotkań, wyrzucanie określonych spotkań ze zbanowanych, 
        )
        .get_matches();

        /*& Powiadomienia dla uytkownika o tym w jaki sposób ma korzystać z aplikacji oraz czego nie może robić aby działała ona poprawnie */
        if !Path::new(CONGIG_PATH).exists()
        {
            println!("{}", add_newline_characters("The application cannot be in a folder other than the main folder with the configuration directory and drivers for the browser!!!", 2, 2, "err"));
            std::process::exit(0);
        };

        /*& Nasłuchiwanie gdy użytkownik klika w odpowiednie przycisk */
        ctrlc::set_handler(|| { // zamykanie aplikcaji po kliknięciu w przycisk ctrl + c
            println!("{}", add_newline_characters("The application is closed...", 4, 0, "err"));
            std::thread::sleep(std::time::Duration::from_millis(1500));
            std::process::exit(0);
        })
        .expect(add_newline_characters("Program can't listen CTRL + C window close event!!!", 2, 2, "err").as_str());


        if app.is_present("run") || app.subcommand_matches("run").is_some() // odpalanie aplikacji
        {
            let application_was_not_configured_message: String = add_newline_characters("Before starting the application, you need to configure it!!!\nFor this purpose, use:\nCommand: teams-automatization.exe config", 2, 2, "err");
            // W momencie gdy plik konfiguracyujny istnieje
            if Path::new(&format!("{}/main.config.json", CONGIG_PATH)).exists()
            {
                let file_data: MainConfigFile = serde_json::from_str(fs::read_to_string(format!("{}/main.config.json", CONGIG_PATH)).expect(format!("Failed to read the configuration data\n{}", application_was_not_configured_message).as_str()).as_str()).expect(format!("Failed to read the configuration data\n{}", application_was_not_configured_message).as_str());
                
                // W momencie gdy dane pól hasła i emaila zostały podane w pliku konifguracyjnym
                if file_data.email != "nil".to_string() && file_data.passwd != "nil".to_string()
                {
                    println!("{}", add_newline_characters("App was started...", 2, 2, "suc"));
                    application_main().await; // odpalanie aplikacji
                }
                else
                {
                    panic!("{}", application_was_not_configured_message);
                }
            }
            else // gdy nie znaleziono pliku konfiguracyjnego lub gdy nie znaleziono folderu z plikami konfiguracyjnymi
            {
                panic!("{}", application_was_not_configured_message);
            }
        }
        else if app.is_present("config") || app.subcommand_matches("config").is_some() // konfigurowanie aplikacji
        {
            //& -- Funkcja, która zajmuje się wyświetleniem konfiguracji zbanowanych spotkań dla bota
            fn show_user_set_settings(vector: &Vec<NoJoinToThisMeetings>)
            {
                // W moemencie gdy ilość elementów jest większa od zera to zwraca spotkania znajudjące się w pliku konfiguracyjnym,
                // W moemcnie gdy ilość elementów jest równa zeru to zostaje wyświetlony komunikat, że spotkania nie zostały skonfigutrowane i gdy chcemy to zrobić to powinniśmy wykorzystać do tego celu komendę config meetings lub config m
                if vector.len() > 0
                {
                    println!("\n\n{}\n", "Your banned meetings configuration:".bright_green());
                    let mut num_it: u32 = 1;
                    for meet_obj in vector.iter()
                    {
                        println!("{}.", num_it);
                        println!("\tmeeting name: {}", meet_obj.name);
                        println!("\tmeeting author: {}", meet_obj.author);
                        println!("\tmeeting start time: {}", meet_obj.start_time);
                        println!("\tmeeting end time: {}", meet_obj.end_time);
                        println!("\n\n");
                        num_it += 1;
                    }
                }
                else
                {
                    println!("\n\n{}\n\n", "You banned meetings configuration is empty!!! Use command: teams-automatization.exe config meetings OR teams-automatization.exe config m if you would like add some banned meetings!!!".red())
                };
            }

            if app.subcommand_matches("config").unwrap().is_present("meet") // obsługa sub argumentów komendy
            {
                let value_of_subcommand_arg: &str = app.subcommand_matches("config").unwrap().value_of("meet").unwrap();
                if value_of_subcommand_arg == "meetings" || value_of_subcommand_arg == "m" // konfigurowanie spotkań na, które ma niedołączać
                {
                    if app.subcommand_matches("config").unwrap().is_present("type") // obsługa dodatkowych opcji konfiguracji
                    {
                        //& -- Funkcja, która wyświetla obecną konfigurację zbanowanych spotkań
                        fn show_configuration() -> Vec<NoJoinToThisMeetings>
                        {
                            let data_from_file: String = fs::read_to_string("./config/banned-meetings.json").expect("Program coudn't read data about banned meetings from file: banned-meetings.json which is into library: config!!!");
                            let vec_from_struct: Vec<NoJoinToThisMeetings> = serde_json::from_str::<BannedMeetingCollection>(data_from_file.as_str()).expect("Program coudn't convert data from file to JSON format!!!").meet;
                            show_user_set_settings(&vec_from_struct);
                            vec_from_struct
                        }
                        
                        let value_of_subcommand_type_arg: &str = app.subcommand_matches("config").unwrap().value_of("type").unwrap();
                        
                        //& -- Obsługa dodatkowych akcji na komendzie config
                        if value_of_subcommand_type_arg == "display" || value_of_subcommand_type_arg == "dis" // wyświetlanie informacji o ustawionej konfiguracji zbanowanych spotkań
                        {
                            show_configuration();
                        }
                        else if value_of_subcommand_type_arg == "delete" || value_of_subcommand_type_arg == "del"
                        {
                            //& -- Funkcja, która zadaje pytanie uzytkownikowi, które zbanowane spotkania usunąć
                            fn answer()
                            {
                                let which_meeting_delete_question = Question::new("Tell me meetings numbers which you want to delete: ").ask().unwrap();
                                if let Answer::RESPONSE(meetings_num_string) = which_meeting_delete_question
                                {
                                    if meetings_num_string.len() > 0
                                    {
                                        let splited = meetings_num_string.split(" ").collect::<Vec<&str>>(); // numery spotkań do usunięcia
                                        let data_from_file: String = fs::read_to_string("./config/banned-meetings.json").expect("Program coudn't read data about banned meetings from file: banned-meetings.json which is into library: config!!!");
                                        let mut meetings_list: Vec<NoJoinToThisMeetings> = serde_json::from_str::<BannedMeetingCollection>(data_from_file.as_str()).expect("Program coudn't convert data from file to JSON format!!!").meet; // lista spotkań z pliku
                                        let mut deleted_meeting_indexes: Vec<usize> = Vec::new(); // lista indexów do usunięcia spotkania

                                        //& -- Dodawanie odpowiednio przygotowanych indexów, które program ma usunąć
                                        let mut deleted_meeting_list: Vec<NoJoinToThisMeetings> = Vec::new(); // lista usuniętych spotkań
                                        for num in splited.iter()
                                        {
                                            let number_delete_parse = num.parse::<usize>();
                                            match number_delete_parse // sprawdzanie czy konwertowanie elementu do postaci numeru się powiodło
                                            {
                                                Ok(mut number_delete) => 
                                                {
                                                    number_delete = if number_delete == 0 // pomniejszanie o 1 w związku z tym, że spotkania są podane w kolejności
                                                    {
                                                        0
                                                    }
                                                    else
                                                    {
                                                        number_delete - 1
                                                    };
    
                                                    if number_delete < meetings_list.len() // sprawdzanie czy podana liczba spełnia kryteria liczby spotkań 
                                                    {
                                                        deleted_meeting_indexes.push(number_delete);
                                                    }
                                                    else
                                                    {
                                                        println!("{}", "The number of banned meetings to be removed must be greater than or equal to 1 and less than or equal to the number of banned meetings!!!".red());
                                                        answer();
                                                    }
                                                },
                                                Err(_) => 
                                                {
                                                    println!("{}", "Program coudn't convert your added number to number!!! You must add only numbers of banned meeting to delete it. You can't add other characters then numbers and this numbers must be greater then 1 and no greater then banned meeting count!!!".red());
                                                    answer();
                                                }
                                            }
                                        }
    
                                        //& -- Usuwanie indexów
                                        deleted_meeting_indexes.sort();
                                        deleted_meeting_indexes.reverse();
                            
                                        for index in deleted_meeting_indexes.iter()
                                        {
                                            let delted_meeting: NoJoinToThisMeetings = meetings_list.remove(*index);
                                            deleted_meeting_list.push(delted_meeting);
                                        }
                                        // println!("{:?}", deleted_meeting_indexes);
    
                                        //& -- Zapisywanie pliku bez usuniętych danych
                                        let to_json_format: String = serde_json::to_string(&BannedMeetingCollection {
                                            meet: meetings_list
                                        })
                                        .expect("Program coudn't convert result data to JSON file format!!!"); // konwertowanie danych do postaci stylu json
                                        fs::write("./config/banned-meetings.json", to_json_format).expect("Program coudn't save configuration result into JSON file!!!"); // zapisanie danych bez usuniętych danych1
    
                                        println!("{}", format!("Program deleted: {} banned meetings", deleted_meeting_list.len()).bright_green());
                                    }
                                    else
                                    {
                                        println!("\n{}\n", "You must enter the meeting numbers which must be deleted. You should based on this schema: meeting1 meeting2 meeting3 e.t.c".red());
                                        answer();
                                    }
                                }
                                else
                                {
                                    panic!("Something went wrong while the program tried to read your answer!!!")
                                };   
                            }

                            show_configuration(); // wyświetlanie obecnej konfiguracji zbanowanych spotkań
                            answer(); // pytanie użytkownika o to jakie spotkania chce usunąć
                        }
                        else if value_of_subcommand_type_arg == "update" || value_of_subcommand_type_arg == "up" // aktualizowanie zbanowanego spotkania z konfiguracji
                        {
                            let start_value_banned_meetings_list: Vec<NoJoinToThisMeetings> = show_configuration(); // wyświetlanie listy spotkań
                            if start_value_banned_meetings_list.len() > 0 // jeżeli w konfiguracji są jakieś spotkania!!!
                            {
                                // tutaj ponawiane jest pytanie !!!!
                                fn action()
                                {
                                    let question: Answer = Question::new("Which meeting do you want to update (only one meeting):").ask().unwrap();
                                    if let Answer::RESPONSE(data) = question
                                    {
                                        let converted: &str = data.trim(); // usunięto whitespace z końca i początka odpowiedzi
                                        if converted.len() >= 1 // jeżeli podano treść odpowiedzi
                                        {
                                            let number_list = converted.split(" ").collect::<Vec<&str>>();
                                            if number_list.len() == 1
                                            {
                                                let try_to_number = number_list[0].parse::<usize>(); // próba przekonwertowania podanych znaków do postaci liczby
                                                if try_to_number.is_ok() // w momencie gdy udało się przekonwertować do postaci numeru
                                                {
                                                    let to_number: usize = if try_to_number.clone().unwrap().clone() <= 1 // podany przez uzytkownika numer spotkania do usunięcia
                                                    {
                                                        0
                                                    }
                                                    else if try_to_number.clone().unwrap() > 1
                                                    {
                                                        try_to_number.unwrap() - 1
                                                    }
                                                    else
                                                    {
                                                        panic!("Something goes wrong when program converted number!!!")
                                                    };
                                                    
                                                    let text_from_file: String = fs::read_to_string("./config/banned-meetings.json").expect("Program coudn't read data about banned meetings from file: banned-meetings.json which is into library: config!!!");
                                                    let mut to_json_meetings_list: Vec<NoJoinToThisMeetings> = serde_json::from_str::<BannedMeetingCollection>(text_from_file.as_str()).expect("Program coudn't convert data from file to JSON format!!!").meet; // lista spotkań
                                                    if to_json_meetings_list.len() > 0 // sprawdzanie czy lista spotkań z pliku jest pełna
                                                    {
                                                        if to_number < to_json_meetings_list.len() // jeżeli podany numer mieści się w zakresie spotkań. When: to_number = Numer aktualizowanego sptokania
                                                        {
                                                            let old_configuration: NoJoinToThisMeetings = to_json_meetings_list.remove(to_number); // usunięcie z vectora ze spotkaniami starego spotkania i otrzymanie go
                                                            println!("\n\n{}\n\n\n", "Update meeting data has been started...\nIf you want not save update result click keybord keys: CTRL + C".bright_green());
    
                                                            // WHITS: zadawanie pytań w celu aktualizacji danych spotkania
                                                            print!("\n\n");
                                                                
                                                                // -- Podawanie nazwy spotkania
                                                            let q_m_name: String = BannedMeetingsConfig.meeting_name(BannedMeetingsConfigTrybe::Update);
                                                            let m_name = if q_m_name != String::from("nil") // gdy wynikiem odpowiedzi na pytanie nie było nil
                                                            {
                                                                println!("Meeting name has been changed from {old} to {new}", old = old_configuration.name, new = q_m_name);
                                                                q_m_name
                                                            }
                                                            else // gdy wynikiem odpowiedzi na pytanie było nil
                                                            {
                                                                println!("Meeting name hasn't been changed!!!");
                                                                old_configuration.name.clone()
                                                            };
                                                                
                                                                // -- Podawanie nazwy autora spotkania
                                                            let q_m_author: String = BannedMeetingsConfig.meeting_author(BannedMeetingsConfigTrybe::Update);
                                                            let m_author: String = if q_m_author != String::from("nil") // gdy wynikiem odpowiedzi na pytanie nie było nil
                                                            {
                                                                println!("Meeting author has been changed from {old} to {new}", old = old_configuration.author, new = q_m_author);
                                                                q_m_author
                                                            }
                                                            else // gdy wynikiem odpowiedzi na pytanie było nil
                                                            {
                                                                println!("Meeting author time hasn't been changed!!!");
                                                                old_configuration.author.clone()
                                                            };
    
                                                                // -- Podawanie godziny rozpoczęcia spotkania
                                                            let q_m_start_time: String = BannedMeetingsConfig.meeting_time(BannedMeetingsConfigTime::Start, BannedMeetingsConfigTrybe::Update);
                                                            let m_start_time: String = if q_m_start_time != String::from("nil") // gdy wynikiem odpowiedzi na pytanie nie było nil
                                                            {
                                                                println!("Meeting start time has been changed from {old} to {new}", old = old_configuration.start_time, new = q_m_start_time);
                                                                q_m_start_time
                                                            }
                                                            else // gdy wynikiem odpowiedzi na pytanie było nil
                                                            {
                                                                println!("Meeting start time hasn't been changed!!!");
                                                                old_configuration.start_time.clone()
                                                            };
    
                                                                // -- Podawanie godziny końca spotkania
                                                            let q_m_end_time: String = BannedMeetingsConfig.meeting_time(BannedMeetingsConfigTime::End, BannedMeetingsConfigTrybe::Update);
                                                            let m_end_time: String = if q_m_end_time != String::from("nil") // gdy wynikiem odpowiedzi na pytanie nie było nil
                                                            {
                                                                println!("Meeting end time has been changed from {old} to {new}", old = old_configuration.end_time, new = q_m_end_time);
                                                                q_m_end_time
                                                            }
                                                            else // gdy wynikiem odpowiedzi na pytanie było nil
                                                            {
                                                                println!("Meeting end time hasn't been changed!!!");
                                                                old_configuration.end_time.clone()
                                                            };

                                                            //& -- Process zlicznaia liczby zaktualizownaych pól spotkania
                                                            let mut meeting_updated_fileds_count: usize = 0;
                                                            if m_name != old_configuration.name 
                                                            {
                                                                meeting_updated_fileds_count += 1;
                                                            };

                                                            if m_author != old_configuration.author 
                                                            {
                                                                meeting_updated_fileds_count += 1;
                                                            };

                                                            if m_start_time != old_configuration.start_time
                                                            {
                                                                meeting_updated_fileds_count += 1;
                                                            };

                                                            if m_end_time != old_configuration.end_time
                                                            {
                                                                meeting_updated_fileds_count += 1;
                                                            };
    
                                                            //& -- Funkcja zadająca finalne pytanie
                                                            fn final_question(to_json_meetings_list: &Vec<NoJoinToThisMeetings>)
                                                            {
                                                                    // -- Zadawanie pytania kończącego
                                                                let q_final = Question::new("\n\nWould you like to update other meeting data or save this data [con - continue update other meeting, sav - save update result into app banned meetings configuration]?")
                                                                .ask()
                                                                .unwrap();

                                                                // -- Funkcja zapisująca dane w pliku .json
                                                                fn save(to_json_meetings_list: &Vec<NoJoinToThisMeetings>)
                                                                {
                                                                    let full_instance: BannedMeetingCollection = BannedMeetingCollection {
                                                                        meet: to_json_meetings_list.to_vec()
                                                                    };
                                                                    let convert_to_json_format: String = serde_json::to_string(&full_instance).expect("Program coudn't convert data added durning configuration process to JSON file format!!!");
                                                                    fs::write("./config/banned-meetings.json", convert_to_json_format).expect("Program coudn't save configuration result into JSON file!!!");
                                                                }
                                                        
                                                                if let Answer::RESPONSE(data) = q_final // jeżeli została udzielona odpowiedź słowna
                                                                {
                                                                    let up_data = data.trim();
                                                                    if up_data == "con" // pozwala na aktualizowanie kolejnego spotkania
                                                                    {
                                                                        println!("\n\n{}\n\n\n", "Updating another meeting has been started...".bright_green());
                                                                        save(&to_json_meetings_list);
                                                                        // -- Wyświetlanie listy obecnych spotkań
                                                                        show_user_set_settings(&to_json_meetings_list); // wyświetlanie obecnych spotkań
                                                                        action(); // kontynuowanie updatowanie
                                                                    }
                                                                    else if up_data == "sav" // zapisuje spotkanie bez aktualizacji kolejnego
                                                                    {
                                                                        save(&to_json_meetings_list);
                                                                        show_user_set_settings(&to_json_meetings_list); // wyświetlanie obecnych spotkań
                                                                    }
                                                                    else
                                                                    {
                                                                        println!("\n{}", "You must answer with: \"con\" or \"save\"!!!".red());
                                                                        final_question(&to_json_meetings_list); // ponawianie pytania
                                                                    }
                                                                }
                                                                else
                                                                {
                                                                    panic!("Oh no!!! Something went wrong while setting up the meeting author name");
                                                                };
                                                            }
                                                            
                                                            //& -- System sprawdzania czy, które z pól spotkania zostało zmienione a jeżeli żadne z nich nie zosyało zmienione to oznacza to że spotkanie nie zostało tak naprwdę zaktualizowane | WAŻNE: To tutaj inicjowana jest akcja zadawania ostatniego pytania w celu zapisania aktualizowanych danych w pliku .json
                                                            if meeting_updated_fileds_count != 0 // gdy spotkanie zostało aktualizowane
                                                            {
                                                                    //& -- Konfiguracja zapisana w posaci struktury
                                                                let configuration: NoJoinToThisMeetings = NoJoinToThisMeetings {
                                                                    name: m_name,
                                                                    author: m_author,
                                                                    start_time: m_start_time,
                                                                    end_time: m_end_time
                                                                };
                                                                to_json_meetings_list.push(configuration); // -- Dodawanie do vectora nowej konfiguracji // nowa konfiguracja zostaje zapisana dla każdej opcji z (con, sav) nie zależnie od podanej w ostatnim zapytaniu -- vector zostaje dodany do listy spotkań tylko pod warunkiem, że, któreś z pól spotkania zostało zaktualizowane
                                                                    //& -- Zadawanie ostatniego pytania
                                                                final_question(&to_json_meetings_list);
                                                            }
                                                            else // gdy spotkanie nie zostało aktualizowane
                                                            {
                                                                println!("{}", "No one updated meeting key has been chnaged!!! If you would like save updated meeting you must change any meeting field!\n".red());
                                                                show_configuration(); // ponowen wyświetlanie listy spotkań symulując powtórzenie się tej samej akcji -- show_configuration() zamiast show_user_set_settings() poniważ podwany do tej drugiej vector ma usuwane dane w momenie w celu pozyskania starej instancji!!!
                                                                action(); // ponawianie próby zaktualizowania spotkania po przez ponowienie tej samej funkcji
                                                            };
                                                        }
                                                        else
                                                        {
                                                            println!("{}", format!("You entered the wrong meeting number!!! The encounter is to be greater than or equal to {start_num} and less than or equal to {end_num}", start_num = 1, end_num = to_json_meetings_list.len()).red());
                                                            action();
                                                        }
                                                    }
                                                    else
                                                    {
                                                        println!("{}", "There are no saved banned meetings in your banned meetings app configuration!!! You must have any configured banned meeting to update them!!!".red());
                                                        action();
                                                    }
                                                }
                                                else
                                                {
                                                    println!("{}", "You can't add other characters then number to update the meeting!!!".red());
                                                    action();
                                                }
                                            }
                                            else
                                            {
                                                println!("{}", "You must add only one number meeting to update in one time!!! Program can update only one meeting in one time!!!".red());
                                                action();
                                            } 
                                        }
                                        else
                                        {
                                            println!("{}", "You must add only one number meeting to update in one time!!! Program can update only one meeting in one time!!!".red());
                                            action();                                        }
                                    }
                                    else
                                    {
                                        panic!("Something went wrong while the program tried to read your answer!!!")
                                    };
                                }
                                action();
                            }
                        }
                        else if value_of_subcommand_type_arg == "help" || value_of_subcommand_type_arg == "h"
                        {
                            println!("{intro}\n\n{display} - this command display your actual banned meetings configuration.\n\n\t{how_to_use}\n\t\t1. Enter: config meetings/m display/dis,\n\t\t2. After typing this command, the bot will show you the current banned meetings configuration,\n\n{delete} - this command is for delete banned meetings from your saved configuration.\n\n\t{how_to_use}\n\t\t1. Enter: [config meetings/m del/delete],\n\t\t2. After that command bot show you banned meetings list with number of specific meeting and display below question \"Tell me meetings numbers which you want to delete:\" or display a message stating that you do not have any saved banned meetings in your configuration,\n\t\t3. If meetings list is displayed you should enter number of specific displayed meeting/meetings to delete this meeting/these meetings from your application configuration!!!. This process look like that: \"Tell me meetings numbers which you want to delete:\" meeting_number1 meeting_number2 meeting_number3,\n\t\t4. If you want close delete banned meetings panel you should use keyboard keys CTRL + C\n\n{update} - this command update your actual banned meetings configuration.\n\n\t{how_to_use}\n\n\t\t1. Enter: [config meetings/m up/update].\n\t\t2. After that bot show you banned meetings list and ask you: \"Which meeting do you want to update (only one meeting)\",\n\t\t3. In response, add only one number of the meeting that you want to update", intro = "Commnad List for [config meetings [command]]:".bright_green(), display = "display | dis".bright_blue(), delete = "delete | del".bright_blue(), update = "update | up".bright_blue(), how_to_use = "How to use this command:".on_black())
                        }
                        else // gdy użytkownik poda nie obsługiwaną komendę!!!
                        {
                            println!("{}", "This command is not supported!!! Use command: [config -help] - if you want see availeble options for this command!!!".red());
                        }
                    }
                    else // konfigurowanie zbanowanych spotkań aplikacji (są to takie spotkanie na, które bot nie dołącza!!!)
                    {
                        println!("{intro}:\n\n\t{update} - this commnad updating your meetings configuration,\n\t{delete} - this command delete meeting/meetings from your application banned meetings configuration\n\t{display} - this command showing you your application banned meetings configuration\n\n", intro = "If you would like update or delete any meeting from configuration you should save your config first using [sav - in response for question] and later use commands".bright_green(), update = "config meeting/m update/up".bright_blue(), delete = "config meeting/m delete/del".bright_blue(), display = "config meeting/m dis/display".bright_blue());
                        let mut meetings_collection: Vec<NoJoinToThisMeetings> = Vec::new(); // vektor z listą spotkań na, które ma dołączać
                        //& -- Funkcja wyświetlająca pytania konfiguracyjne dla aplikacji
                        fn meeting_questions(mut collecion: &mut Vec<NoJoinToThisMeetings>)
                        {
                            // -- parametry dla spotkania
                            let meeting_name: String;
                            let meeting_author: String;
                            #[allow(unused_assignments)] // żeby nie pojawiał się tutaj ten głupi warning
                            let meeting_start_time: String;
                            #[allow(unused_assignments)] // żeby nie pojawiał się tutaj ten głupi warning
                            let meeting_end_time: String;
    
                                // -- Podawanie nazwy spotkania
                            meeting_name = BannedMeetingsConfig.meeting_name(BannedMeetingsConfigTrybe::Create);
    
                                // -- Podawanie nazwy autora spotkania
                            meeting_author = BannedMeetingsConfig.meeting_author(BannedMeetingsConfigTrybe::Create);
    
                                // -- Podawanie godziny rozpoczęcia spotkania
                            meeting_start_time = BannedMeetingsConfig.meeting_time(BannedMeetingsConfigTime::Start, BannedMeetingsConfigTrybe::Create);
    
                                // -- Podawanie godziny końca spotkania
                            meeting_end_time = BannedMeetingsConfig.meeting_time(BannedMeetingsConfigTime::End, BannedMeetingsConfigTrybe::Create);
    
                                // -- Dodawanie stworzonych danych do vectora
                            let instance: NoJoinToThisMeetings = NoJoinToThisMeetings 
                            {
                                name: meeting_name,
                                author: meeting_author,
                                start_time: meeting_start_time,
                                end_time: meeting_end_time
                            };

                                // -- Liczba wartości "nil" znajdujących się w elemencie
                            let mut nil_count: usize = 0;

                            if instance.name == "nil".to_string()
                            {
                                nil_count += 1;
                            }
                            
                            if instance.author == "nil".to_string()
                            {
                                nil_count += 1;
                            }
                            
                            if instance.end_time == "nil".to_string()
                            {
                                nil_count += 1;
                            }
                            
                            if instance.start_time == "nil".to_string()
                            {
                                nil_count += 1;
                            };
    
                                // -- Zapisywanie wyników konfiguracji lub rozpoczynanie konfigurowania kolejnego spotkania
                            fn finall_answer(mut vector: &mut Vec<NoJoinToThisMeetings>)
                            {
                                show_user_set_settings(&vector); // wyświwtlanie konfiguracji, która została do tej pory ustawiona
                                let save_or_continue_vonf_question = Question::new("Would you like continue configuration or save it result? [con - continue configuration, sav - save configuration result]: ").ask().unwrap();
    
                                if let Answer::RESPONSE(res) = save_or_continue_vonf_question
                                {
                                    if res.as_str() == "con" //& -- kontynuacja konfiguracji
                                    {
                                        println!("\n\n{}\n\n", "Another stage of banned meetings configuration has been started!!!".bright_green());
                                        meeting_questions(&mut vector);
                                    }
                                    else if res.as_str() == "sav" //& -- zapisywanie konfiguracji
                                    {
                                        //& -- Funkcja, która zastępuje stare dane w pliku nowymi danymi lub po prostu zapisuje wprowadzone dane w pliku (jest uzywana w momencie gdy plik konfiguracyjny nie istnieje w celu zapisania danych w dopiro co stworzonym pliku oraz do zastepowania starych danych konfiguracyjnych nowymi danymi)
                                        fn write_data_to_file(vector: &mut Vec<NoJoinToThisMeetings>)
                                        {
                                            let to_saved: BannedMeetingCollection = BannedMeetingCollection {
                                                meet: vector.to_vec()
                                            };
                                            let json_file: String = serde_json::to_string(&to_saved).expect("Program coudn't convert data added durning configuration process to JSON file format!!!");
                                            fs::write("./config/banned-meetings.json", json_file).expect("Program coudn't save configuration result into JSON file!!!");
                                        }
    
                                        if Path::new("./config/banned-meetings.json").exists() // gdy plik ze spotkaniami już istnieje
                                        {
                                            // Odczytuje dane z pliku i zapisuje je wraz z nowymi danymi jeżeli plik istnieje
                                            let question_wdlike_to_replace_data = Question::new("Would you like to replace old configuration data by new configuration data? [yes/no]")
                                            .yes_no()
                                            .clarification(&format!("Please enter either 'yes' or 'no'\n").red())
                                            .confirm();
    
                                            if let Answer::YES = question_wdlike_to_replace_data // zastąpywanie starych danych konfiguracyjnych nowymi
                                            {
                                                write_data_to_file(&mut vector); // zastąpywanie starych danych nowymi danymi
                                                println!("{}", "Configuration for banned meetings has been saved!!!".bright_green());
                                            }
                                            else // dodawanie do starych danych konfiguracyjnych nowych danych konfiguracyjnych
                                            {
                                                let bytes_from_file: String = fs::read_to_string("./config/banned-meetings.json").expect("Program coudn't read data from banned meetings configuration JSON file!!!");
                                                let to_struct = serde_json::from_str::<BannedMeetingCollection>(bytes_from_file.as_str()).expect("Program coudn't convert data from file to JSON format!!!");
                                                let mut config_data: Vec<NoJoinToThisMeetings> = to_struct.meet;
                                                config_data.append(&mut vector); // dodanie do starego wektora z danymi konfiguracyjnymi nowych danych konfiguracyjnych
                                                write_data_to_file(&mut config_data); // zapisywanie danych
                                                println!("{}", "Configuration for banned meetings has been saved!!!".bright_green());
                                            }
    
                                        }
                                        else // gdy plik ze spotkaniami nie istnieje
                                        {
                                            let _ = fs::File::create("./config/banned-meetings.json").expect("Program coudn't create config file with banned meetings!!!");
                                            write_data_to_file(&mut vector); // zastąpywanie starych danych nowymi
                                            println!("{}", "Configuration for banned meetings has been saved!!!".bright_green());
                                        }
                                    }
                                    else
                                    {
                                        println!("{}", "You must answer with: \"con\" or \"sav\"!!!".red());
                                        finall_answer(&mut vector)
                                    }
                                }
                                else
                                {
                                    panic!("Oh no!!! Something went wrong while asking final configuration step question");
                                };  
                            }

                            //& -- Zabezpiecznie przed stworzeniem doukemntu, który zawiera tylko pola z samymi wartościami nil
                            if nil_count != 4 
                            {
                                collecion.push(instance); // dodawanie do kolekcji nowego spotkania tylko w momencie w, którym liczba wartości nil kluczy nie jest równa liczbie wszystkich kluczy
                                finall_answer(&mut collecion);
                            }
                            else
                            {
                                println!("{}", "Meeting fields coudn't have got only \"nil\" values!!!\nTry again adding a value other than \"nil\" to some meeting key!\n".red());
                                meeting_questions(collecion);
                            }
                        }   
                        meeting_questions(&mut meetings_collection); // funkcja zwróci pożyczoną referencję   
                    }
                }
                else
                {
                    println!("{}", "You provided an unsupported configuration argument. Use commnad with another argument like: teams-automatization.exe config meetings".red())
                }
            }
            else // Konfigurowanie danych do logowania dla aplikacji
            {
                let added_email: String;
                let added_password: String;
                println!("{}", add_newline_characters("Application configuration started...", 2, 2, "suc"));
                
                // Zadawanie pytania adres e-mail konta użytkownika
                let question_email: Answer = Question::new("Add account e-mail address:")
                .default(Answer::RESPONSE("nil".to_string()))
                .ask()
                .expect( add_newline_characters("The response could not be read!!!", 0, 0, "err").as_str());
                
                if let Answer::RESPONSE(data) = question_email
                {
                    added_email = data;
                }
                else
                {
                    added_email = "nil".to_string();
                };
            
                // Zadawanie pytania o hasło użytkownika
                let question_password: Answer = Question::new("Add account password:")
                .default(Answer::RESPONSE("nil".to_string()))
                .ask()
                .expect(add_newline_characters("The response could not be read!!!", 0, 0, "err").as_str());
            
                if let Answer::RESPONSE(data) = question_password
                {
                    added_password = data;
                }
                else
                {
                    added_password = "nil".to_string();
                }
            
                // Zadawanie pytania czy dane podane w konfiguracji mają zostać zaszyfrowane w celu ich ochrony przed przeczytaniem
                let question_encrypt_added_data: Answer = Question::new("Would you like encrypt added data (this should prevent your data from being accidentally shared)? [yes/no]")
                .yes_no()
                .clarification(&format!("Please enter either 'yes' or 'no'\n").red())
                .confirm();
            
                // Zadwanie pytania czy użytkownik chce odpalić aplikację wraz z udzieleniem na nie odpowiedzi i wykonanie akcji adekwatnej do podanej odpowiedzi przez uzytkownika odpowiedzi
                async fn run_application_question()
                {
                    let question_run_the_application: Answer = Question::new("The application has been set up! Would you like to start it? [yes/no]")
                    .yes_no()
                    .clarification(&format!("Please enter either 'yes' or 'no'\n").red())
                    .confirm();
                
                    // W momencie gdy użytkownik chce odpalić aplikacje = włączanie apliakcji
                    if let Answer::YES = question_run_the_application
                    {
                        application_main().await
                    }
                    else // w momencie gdy uzytkownik nie chce odpalać aplikacji = wyłączanie aplikacji
                    {
                        println!("{}", "Program has been turned off!!!".red());
                        std::process::exit(0);
                    }
                }
            
                /*& PYTANIE O TO CZY UŻYTKOWNIK CHCE ZASZYFROWAĆ PODANE DANE KONFIGURACYJNE */
                if let Answer::YES = question_encrypt_added_data // w momencie gdy użytkownik chce zaszyfrować podane dane
                {
                    // Progress bar informujący uzytkownika o tym, że jego dane są szyfrowane

                    let bar = ProgressBar::new(10);
                    bar.println(add_newline_characters("Encryption of the provided data...", 2, 2, "none")); // wiadomośc nad barem o tym że aplikacja została włączona
                    for _  in 0..10
                    {
                        bar.inc(1);
                        std::thread::sleep(std::time::Duration::from_millis(300));
                    }
                    bar.finish_and_clear();

                    // --- ciało szyforowania danych

                    let EncryptedData { enc_password, enc_email } = encrypt_data(added_email, added_password); // zwracanie zaszyfrowanych danych
                    let config_data_struct: MainConfigFile = MainConfigFile {
                        passwd: enc_password,
                        email: enc_email,
                        encrypted: true
                    };
                    let config_data_struct_json: String = serde_json::to_string(&config_data_struct).expect(add_newline_characters("Program coudn't serialize added config data. Please try again!!!", 2, 2, "err").as_str());
                    fs::write(format!("{}/main.config.json", CONGIG_PATH), config_data_struct_json).expect(add_newline_characters("Program can't save added configuration data. Please try again!!!", 2, 2, "err").as_str());
                    run_application_question().await;
                }
                else // w momencie gdy użytkownik nie chce zaszyfrowywac podanych danych
                {
                    let config_data_struct: MainConfigFile = MainConfigFile {
                        passwd: added_password,
                        email: added_email,
                        encrypted: false
                    };
                    let config_data_struct_json: String = serde_json::to_string(&config_data_struct).expect(add_newline_characters("Program can't serialize added config data. Please try again!!!", 2, 2, "err").as_str());
                    fs::write(format!("{}/main.config.json", CONGIG_PATH), config_data_struct_json).expect(add_newline_characters("Program can't save added configuration data. Please try again!!!", 2, 2, "err").as_str());
                    run_application_question().await;
                }
            }
                
        }
        else
        {
            println!("Open Console (cmd/powershell) and Type \"teams-automatization-bot.exe -h\" or \"teams-automatization-bot.exe --help\" or \"teams-automatization-bot.exe help\" to get more informations about program and you should also read attached program documentation (DOCUMENTATION.md file)!!!\nMore informations how to use windows shell is here: https://www.makeuseof.com/tag/a-beginners-guide-to-the-windows-command-line/");
            std::thread::sleep(std::time::Duration::from_secs(60 * 5)); // program czeka 2 sekundy przed zamknięciem go
        }
    }
    else // W momencie gdy aplikacja została odpalona na innym systemie operacyjmnym np: linux
    {
        println!("{}", add_newline_characters("This version of the application only works on Windows operating systems (WINDOWS_NT family)", 2, 2, "err"))
    };
}

#[cfg(test)]
mod tests {
    use colored::Colorize;

    use crate::modules;

    use super::application_main;
    use super::{ read_data_from_conf_file, add_newline_characters };
    use super::modules::config::Config;

    #[test]
    fn default()
    {
        // tetsowanie czy bibliteka driver została odpowiednio stworzona
        let config_data: Config = Config::new();
        let driver_dir: String = config_data.driver_directory;
        println!("{}", std::path::Path::new(&driver_dir).exists());
    }

    #[test]
    fn file_reader()
    {
        println!("test is.. runnded");
        println!("{:#?}", read_data_from_conf_file());
    }

    #[test]
    fn key()
    {
        // let data: String = crate::modules::encryption::generate_save_and_resturn_key();
        // println!("{} : {}", data, data.len())
    }

    #[test]
    fn parser()
    {
        println!("os");
        println!("{}", add_newline_characters("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\nthis is a smart plan for how data should be parsed.\nNew data !!!", 2, 2, "none"));
        println!("os");
    }

    #[test]
    fn progress_bar()
    {
        #[allow(unused)]
        use indicatif::{ ProgressStyle, ProgressBar };
        
        let bar = ProgressBar::new(10);
        bar.println("Waiting for the browser driver to fire ..."); // wiadomośc nad barem o tym że aplikacja została włączona

        // odpalanie
        for _  in 0..10
        {
            bar.inc(1);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        bar.finish_and_clear();
        println!("{}", add_newline_characters("The driver was fired !!!", 2, 2, "none"));
        


        // println!("\nBar data:\nposition: {},\nlength: {},\nduration: {}\n, time: {},\nupdated_per_sec: {}", bar.position(), bar.length(), bar.duration().as_secs(), bar.elapsed().as_secs(), bar.per_sec());
    }

    #[test]
    fn collored()
    {
        // kolor gdy sukces
        println!("{}", "success_test".green().bold().to_string());
        // kolor w momencie gdy wystąpił błąd
        println!("{}", "error_test".red().bold().to_string());

        // działanie z funkcją:
        // funkcja przyjmuje dwa typy zwracanych kolorów - suc = zielony gdzy coś przeszło pomyślnie, err - gdy wystąpił błąd
    }

    #[tokio::test]
    async fn run_app()
    {
        application_main().await;
    }

/*     #[test]
    fn time()
    {
        let chrono = chrono::offset::Local::now();
        println!("{} {}", chrono.time(), chrono.date().to_string().split("+").collect::<Vec<&str>>()[0].split(":").collect::<Vec<&str>>())
    } */
}