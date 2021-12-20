/* This module contains BOT callendar future */
use crate::*;

#[allow(dead_code)]
pub async fn callendar(driver: &GenericWebDriver<ReqwestDriverAsync>)
{
    // Przechodzenie do kalenarza
    go_to_callendar(&driver).await;

    // Zczytywanie listy zbanowanych spotkań
    let banned_meeting_list: Vec<NoJoinToThisMeetings> = get_banned_meetings_list().await; // lista zbanowanych spotkań przeniesiona tutaj aby nie była zczytywana z pliku przy każdej iteracji pętli

    /*& Otrzymywanie daty i czasu */
    let time: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
    // --- Data - rok, miesiąc, data:
    let time_date: String = time.date().to_string();
    let time_date_split: Vec<&str> = time_date.split("+").collect::<Vec<&str>>()[0].split("-").collect();
    let _year_time_date: &str = time_date_split[0];
    let _month_time_date: &str = time_date_split[1];
    let date_time_date: &str = time_date_split[2];    

    //& Funkcja, która przechodzi do kalendarza z listą spotkań
    async fn go_to_callendar(driver: &GenericWebDriver<ReqwestDriverAsync>)
    {
        let callendar_button: WebElement = driver.find_element(By::Id("app-bar-ef56c0de-36fc-4ef8-b417-3d82ba9d073c")).await.expect("Program can't found callendar button!!!");
        callendar_button.click().await.expect("Program can't go to the callendar!!!"); // Szukanie lekcji w kalendarzy TODO: Ma zostać ona dodana do funkcji sprawdzającej rzeczy raz na jakiś czas tak samo jak metoda setInterval z JavaScriptu
        std::thread::sleep(std::time::Duration::from_secs(12)); // czekanie na przejście do kalendarza
    }

    //& Funkcja, która zwraca numer dnia z kalendarza teams, który jest równy dzisiejszemu dniu (jeżeli nie dopasuje dnia to zwraca wyliczenie Result)
    async fn get_meeting_day(driver: &GenericWebDriver<ReqwestDriverAsync>, _class: String, date_time_date: &str) -> Result<usize, &'static str>
    {
        /* Parametry:
            - driver - driver thirtyfour.rs,
            - class - podana klasa elementu,
            - date_time_date - dzisiejszy dzień
        */
        // Musi być tak zrobione aby uniknąć pojawiania się błędów
        let finded_days: Vec<WebElement> = driver.find_elements(By::ClassName("node_modules--msteams-bridges-components-calendar-grid-dist-es-src-renderers-calendar-multi-day-renderer-calendar-multi-day-renderer__gridColumn--1PTFP")).await.expect("Program can't find day container!!!");

        let mut result: usize = 1_000;
        for day_num in 0..finded_days.len()
        {
            let day_element_date_raw_text: String = finded_days[day_num].clone().find_element(By::ClassName("node_modules--msteams-bridges-components-calendar-grid-dist-es-src-renderers-calendar-multi-day-renderer-calendar-multi-day-renderer__ariaDateHeader--BFYlE")).await.expect("Program coudn't get day date element!!!").text().await.expect("Program codun't get element date text!!!"); // data dnia
            let day_element_date_ready_text: &str = day_element_date_raw_text.as_str().split(" ").collect::<Vec<&str>>()[0].trim();
            // jeżeli data tego dnia równa się dzisiejszej dzacie to ustawia jako result numer dnia iteracji i zakańcza pętlę za pomocą break;
            if day_element_date_ready_text == date_time_date
            {
                result = day_num;
                break;
            }
        };
        
        // Jeżeli Result nie jest równy 1000 z numerem dnia to zwraca Ok a jeżeli jest równy 1000 to zwraca Error
        if result != 1_000
        {
            Ok(result)
        }
        else
        {
            Err("Program coudn't find meeting day!!!")
        }
    }

    //& Funkcja, która zwraca listę spotkań
    async fn get_meetings_list(driver: &GenericWebDriver<ReqwestDriverAsync>, class: String, meeting_day_number: usize) -> Vec<TeamsCalendaryMeeting>
    {
        /* W momenicie gdy funkcja ta nie znajdzie spotkań to zwraca ona pusty Vector a obsługa go i wchodzenie na spotkania ma zostać wykonana poza tą funkcją */
        let meetings_container: WebElement = driver.find_elements(By::ClassName(class.as_str())).await.expect("Program can't found scheduled meetings container!!!")[meeting_day_number].clone();
        let meetings_list: Vec<WebElement> = meetings_container.find_elements(By::ClassName("node_modules--msteams-bridges-components-calendar-grid-dist-es-src-renderers-calendar-multi-day-renderer-calendar-multi-day-renderer__eventCard--3NBeS")).await.expect("Program can't found scheduled meetings!!!");
        let mut vector_with_meetings: Vec<TeamsCalendaryMeeting> = Vec::new();
        for meeting in meetings_list.iter()
        {
            let meeting_info_container: String = meeting.find_elements(By::Tag("div")).await.expect("Program coundn't found meeting info container!!!")[0].get_attribute("aria-label").await.expect("Program coundn't found container info attribute!!!").expect("Program coudn't read attribute text!!!");
            let split_comma_info: Vec<&str> = meeting_info_container.trim().split(",").collect::<Vec<&str>>();
            let split_time: Vec<&str> = 
            if split_comma_info.len() == 5 // jeżeli jest 5 elementów: nazwa spotkania, godzina spotkania , organizator i ten napis na chuja dodany
            {
                split_comma_info[1].split(" ").collect::<Vec<&str>>()
            }
            else // jeżeli są trzy elementy to oznacza to, że nie podano nazwy spotkania a więc indexem czasu będzie 1
            {
                split_comma_info[0].split(" ").collect::<Vec<&str>>()
            };

            // Dane do schemy
            let name: String = 
            if split_comma_info.len() == 5 // jeżeli jest 5 elementów: nazwa spotkania, godzina spotkania , organizator i ten napis na chuja dodany
            {
                split_comma_info[0].to_string()
            }
            else // gdy jest mniej niż 4 przycięte elementy to oznacza to że nie podano nazwy spotkania a więc jest ona równa "nil"
            {
                String::from("nil")
            };
            
            let organizator: String = 
            if split_comma_info.len() == 5 // jeżeli jest 5 elementów: nazwa spotkania, godzina spotkania , organizator i ten napis na chuja dodany
            {
                split_comma_info[2].split(":").collect::<Vec<&str>>()[1].trim().to_string()
            }
            else // gdy jest mniej niż 5 przyciętych elementów to oznacza to że nie podano nazwy spotkania a więc zmienia się kolejność
            {
                split_comma_info[1].split(":").collect::<Vec<&str>>()[1].trim().to_string()
            };

            let start_time: String = split_time[split_time.len() - 3].to_string();
            let end_time: String = split_time[split_time.len() - 1].to_string();

            let meeting_instance: TeamsCalendaryMeeting = TeamsCalendaryMeeting {
                name,
                start_time,
                end_time,
                organizator
            };
            vector_with_meetings.push(meeting_instance); // dodwanie instancji do listy
        };
        vector_with_meetings // zwracanie vectora z instancjami
    }

    //& Funkcja, która powoduje, że uzytkownik dołączył na spotkanie
    async fn join_to_meeting(driver: &GenericWebDriver<ReqwestDriverAsync>)
    {
        // Otrzymanie przełączników urządzeń: mikrofonu oraz kamerki
        let on_off_buttons: Vec<WebElement> = driver.find_element(By::ClassName("buttons-container")).await.expect("Program coudn't find devices buttons container!!!").find_elements(By::Tag("toggle-button")).await.expect("Program coudn't find on/off device buttons!!!"); // 0 - kamera, 1 - mikrofon

        // turn-off camera
        let camera_button: WebElement = on_off_buttons[0].clone();
        let camera_button_is_enabled: bool = camera_button.is_enabled().await.unwrap();
        let camera_button_on_off_state: String = camera_button.get_attribute("telemetry-summary").await.expect("Camera state attribute doesn't exists!!!").unwrap();

        if camera_button_is_enabled && camera_button_on_off_state == "Toggle camera OFF in meeting pre join screen"
        {
            camera_button.click().await.expect("Program coudn't click on camera button!!!");
        }

        // turn-off microphone
        let microphone_button: WebElement = on_off_buttons[1].clone();
        let microphone_button_is_enabled: bool = microphone_button.is_enabled().await.unwrap();
        let microphone_button_on_off_state: String = microphone_button.get_attribute("telemetry-summary").await.expect("Microphone state attribute doesn't exists!!!").unwrap();

        if microphone_button_is_enabled && microphone_button_on_off_state == "Toggle microphone OFF in meeting pre join screen"
        {
            microphone_button.click().await.expect("Program coudn't click on microphone button!!!");
        }

        // join to meeting
        std::thread::sleep(std::time::Duration::from_millis(300));
        driver.find_element(By::ClassName("join-btn")).await.expect("Program can't find join to meeting button!!!").click().await.expect("Program can't click on join to meeting button!!!");
        std::thread::sleep(std::time::Duration::from_secs(10)); // czekanie 15 sekund na dołączenie do spotkania: ma zapobiegać to błędom

        // send an invitation to another person to join the meeting handling which delte this element from user view :)
        let send_invite_element_find = driver.find_element(By::ClassName("ngdialog-content")).await;
        if send_invite_element_find.is_ok() // w momencie gdy ten element jest wyświetlany (element pytający czy zaprosić innych uzytkowników na spotkanie)
        {
            driver.action_chain().move_to(533, 358).click().perform().await.expect("Program coudn't do action which should remove fucking send invite to other users info!!!"); // wykonuje akcję
            std::thread::sleep(std::time::Duration::from_millis(10)); // czeka 10 milisekund na zamknięcie tego menu
            show_meeting_bar(&driver).await;
            show_meeting_bar(&driver).await;
        }
    }

    //&-- funkcja, która pokazuje bar spotkania w momencie gdy użytkownik znajduje się na spotkaniu
    async fn show_meeting_bar(driver: &GenericWebDriver<ReqwestDriverAsync>)
    {
        driver.action_chain().move_by_offset(15, 4).move_by_offset(-15, -4).click().perform().await.expect("Program coudn't move cursor for show meeting bar!!!");
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    //&-- Funkcja ta zwraca liczbę osób, które są na spotkaniu nie uwzględniając przy tym siebie (czyli numer osób minus 1)
    async fn how_much_users_is_in_the_meeting(driver: &GenericWebDriver<ReqwestDriverAsync>) -> String
    {
        // Otwieranie menu z listą uzytkowników jest wykonuwane w momencie gdy te menu nie jest już wyświetlane
        if driver.find_element(By::XPath("//*[@id=\"page-content-wrapper\"]/div[1]/div/calling-screen/div/div[2]/meeting-panel-components/calling-roster/div/right-pane-header/div/div/div/h2")).await.is_err() // gdy menu nie zostanie znalezione to je otwiera
        {  
            let meeting_bar_meeting_users_info_button: WebElement = driver.find_element(By::Id("roster-button")).await.expect("Program coudn't find users info button on the meeting!!!"); // przycisk otwierający listę użytkowników na barze
            let action_result = meeting_bar_meeting_users_info_button.click().await; // kliknięcie w przycisk z listą użytkowników na barze
            
            // w momencie gdy nie udało sie wyśweitlić menu z listą uczestników spotkania to próba jego otwarca zostaje ponowiona
            if action_result.is_err()
            {
                show_meeting_bar(&driver).await;
                let meeting_bar_meeting_users_info_button: WebElement = driver.find_element(By::Id("roster-button")).await.expect("Program coudn't find users info button on the meeting!!!"); // przycisk otwierający listę użytkowników na barze
                meeting_bar_meeting_users_info_button.click().await.expect("Program coudn't click on meeting info button!!!"); // kliknięcie w przycisk z listą użytkowników na barze; // kliknięcie w przycisk z listą użytkowników na barze
            }
            std::thread::sleep(std::time::Duration::from_millis(1500)); // czekanie na otworzenie się menu z listą użytkowników na spotkaniu
        }

        // liczba uczestników ma znajdować się tutaj po Xpath
        let meeting_user_count_full_text: String = driver.find_element(By::XPath("//*[@id=\"page-content-wrapper\"]/div[1]/div/calling-screen/div/div[2]/meeting-panel-components/calling-roster/div/div[3]/div/div[1]/accordion/div/accordion-section[2]/div/calling-roster-section/div/div[1]/button/span[3]")).await.expect("Program coudn't find element with user count in meeting!!!").text().await.expect("Program coudn't read text containing number represent user quantity in meeting!!!");


        let meeting_user_count_full_text_without_buckles: usize = meeting_user_count_full_text.replace("(", "").replace(")", "").parse::<usize>().unwrap(); // informacja z liczbą uczestników spotkania bez nawiasów
        return (meeting_user_count_full_text_without_buckles - 1).to_string(); // zwracanie ilośc użytkowników nie wliczając w to siebie
    }

    //&-- Funkcja ta zwraca ile czasu trwa spotkanie
    async fn how_long_the_meeting_lasts(driver: &GenericWebDriver<ReqwestDriverAsync>) -> (String, String, String)
    {
        show_meeting_bar(&driver).await;
        let meeting_time: String = driver.find_element(By::XPath("//*[@id=\"calling-duration\"]/div")).await.expect("Program coudn't find meeting time duration!!!").text().await.expect("Program coudn't read text containing meeting time duration!!!");
        let meeting_time_separate_units: Vec<&str> = meeting_time.trim().split(":").collect();
        let hour: String = if meeting_time_separate_units.len() >= 3 // godizny są zwracane tylko w momencie gdy ilośc elementów w czasie oddzielona na 
        {
            let hour_unit: &str = meeting_time_separate_units[meeting_time_separate_units.len() - 3];
            if hour_unit.starts_with("0")
            {
                let mut splitted = hour_unit.split("").collect::<Vec<&str>>();
                splitted.pop();
                splitted.reverse();
                splitted.pop();
                splitted.reverse();
                splitted[1].to_string()
            }
            else
            {
                hour_unit.to_string() 
            }
        }
        else
        {
            String::new()
        };

        let minute: String = if meeting_time_separate_units[meeting_time_separate_units.len() - 2].starts_with("0") // w momencie gdy zaczyna się od zera to nie zwraca liczbę bez 0
        {
            let minute_data = meeting_time_separate_units[meeting_time_separate_units.len() - 2].trim();
            let mut splitted = minute_data.split("").collect::<Vec<&str>>();
            splitted.pop();
            splitted.reverse();
            splitted.pop();
            splitted.reverse();
            splitted[1].to_string()
        }
        else
        {
            meeting_time_separate_units[meeting_time_separate_units.len() - 2].to_string()
        };

        let second: String = if meeting_time_separate_units[meeting_time_separate_units.len() - 1].starts_with("0") // w momencie gdy zaczyna się od zera to nie zwraca liczbę bez 0
        {
            let second_data = meeting_time_separate_units[meeting_time_separate_units.len() - 1].trim();
            let mut splitted = second_data.split("").collect::<Vec<&str>>();
            splitted.pop();
            splitted.reverse();
            splitted.pop();
            splitted.reverse();
            splitted[1].to_string()
        }
        else
        {
            meeting_time_separate_units[meeting_time_separate_units.len() - 1].to_string()
        };

        std::thread::sleep(std::time::Duration::from_secs(1)); // czeka na zamknięcie się bara
        (hour, minute, second)
    }

    //&-- Funkcja, która wychodzi ze spotkania
    async fn leaving_the_meeting(driver: &GenericWebDriver<ReqwestDriverAsync>)
    {
        //& --- Ta funkcja aktywuje wychodzenie
        async fn activate(driver: &GenericWebDriver<ReqwestDriverAsync>) 
        {
            show_meeting_bar(&driver).await;
            let leave_meeting_button: WebElement = driver.find_element(By::XPath("//*[@id=\"hangup-button\"]")).await.expect("Program coudn't find leave button!!!");
            let click_result_no = leave_meeting_button.click().await;
            if click_result_no.is_ok()
            {
                body(&driver).await;
            }
            else
            {
                show_meeting_bar(&driver).await;
                let leave_meeting_button: WebElement = driver.find_element(By::XPath("//*[@id=\"hangup-button\"]")).await.expect("Program coudn't find leave button!!!");
                let _ = leave_meeting_button.click().await.expect("Program coudn't click in leave meeting");
                body(&driver).await;
            }

            async fn body(driver: &GenericWebDriver<ReqwestDriverAsync>)
            {
                std::thread::sleep(std::time::Duration::from_secs(2)); // czekanie na pokazanie sie informacji o ewentualnym szybki opuszczeniu spotkania
            
                if driver.find_element(By::XPath("//*[@id=\"page-content-wrapper\"]/div[1]/div/calling-screen/div/div[2]/calling-quality-feedback/div/div[2]/button[2]")).await.is_ok() // w momencie gdy ten element został pokazany
                {
                    let opinion_discard: WebElement = driver.find_element(By::XPath("//*[@id=\"page-content-wrapper\"]/div[1]/div/calling-screen/div/div[2]/calling-quality-feedback/div/div[2]/button[2]")).await.unwrap();
                    opinion_discard.click().await.expect("Program coudn't click on opinion discard button!!!");
                }
                
                std::thread::sleep(std::time::Duration::from_secs(18)); // czekanie na wyjście ze spotkania
                
                // Obsługa gdy ze spotkania wrzuci nas w jakieś inne miejsce aplikacji niż
                if driver.current_url().await.expect("Program coudn't get current page url!!!").contains("https://teams.live.com/_#/conversations/") // gdy zostaliśmy przekierowani po wyjściu ze spotkania do chatów to przechodzi spowrotem do kalendarza
                {
                    go_to_callendar(&driver).await
                }
                println!("{}", "Program has left the meeting and is waiting for the next one!!!".bright_green());
            }
        }
        activate(&driver).await
        // w tym momencie jest finito i program może dalej działać
    }

    //& -- Funkcja, która zwraca listę zbanowanych spotkań
    async fn get_banned_meetings_list() -> Vec<NoJoinToThisMeetings>
    {
        let data_from_file = fs::read_to_string("./config/banned-meetings.json");
        if data_from_file.is_ok()
        {
            let to_json = serde_json::from_str::<BannedMeetingCollection>(data_from_file.unwrap().as_str());
            if to_json.is_ok() // w momencie gdy udało się przekonwertować dane z stringa do formatu json
            {
                to_json.unwrap().meet // zwraca listę spotkań!!!
            }
            else
            {
                println!("{}", format!("Program coudn't convert your banned meetings data to JSON format!!!").red());
                Vec::new()
            }
        }
        else // w moemencie gdy nie ma pliku
        {
            println!("{}", format!("Your banned meetings configuration is empty!!! You can create banned meeting configuration using command: config meetings/m, for have more information about banned meetings app module use command: config meetings/m help/h").red());
            Vec::new()
        }
    }

    //& -- Funkcja, która jest kamieniem węgielnym dla wchodzenia na spotkanie wgl!!!
    async fn meeting_handler(driver: &GenericWebDriver<ReqwestDriverAsync>, end_meeting_time_in_minutes: u64, now_hour_time: u64, now_minute_time: u64, now_sceond_time: u64, meeting: &TeamsCalendaryMeeting, meeting_day_number: usize, iteration: usize)
    {
        // proces dołączania do spotkania
        let search: String = format!("/html/body/div[1]/div[2]/div[2]/div[1]/div/div/calendar-bridge/div/div/div[4]/div[2]/div/div/div[1]/div/div[3]/div[{}]/div[3]/div[{}]", meeting_day_number + 1, iteration + 1); // + 1 o jebany xpath zbiera elementy od 1 a nie od 0 skurwiel jebany
        let meeting_box_check = driver.find_element(By::XPath(search.as_str())).await;
        
        //& -- Ta funkcja obsługuje dochodzenie użytkownika na spotkanie i w momencie gdy znajduje się on na spotkaniu
        async fn fucking_program_module_head(driver: &GenericWebDriver<ReqwestDriverAsync>, meeting_box: WebElement<'_>, end_meeting_time_in_minutes: u64, now_hour_time: u64, now_minute_time: u64 ,now_sceond_time: u64, meeting: &TeamsCalendaryMeeting)
        {
            meeting_box.click().await.expect("Program coudn't click in meeting box!!!");
            std::thread::sleep(std::time::Duration::from_secs(2)); // czekanie na animację, która władowuje element pozwaljący dołączyć do spotkania
            driver.find_element(By::ClassName("node_modules--msteams-bridges-components-calendar-grid-dist-es-src-renderers-peek-renderer-peek-meeting-header-peek-meeting-header__joinButton--3G-er")).await.expect("Program can't find joining to meeting button!!!").click().await.expect("Program can't click on meeting button!!!"); // dołączanie do spotkania
            std::thread::sleep(std::time::Duration::from_secs(5)); // czekanie na wejście do spotkania 

            // Poniżej: Dołączania uzytkownika na spotkanie
            join_to_meeting(&driver).await;

            // Poniżej obsługa użytkownika gdy znajduje się on na spotkaniu:
            let when_user_join_users_count: u64 = how_much_users_is_in_the_meeting(&driver).await.parse::<u64>().expect("Wtf!!! Program coudn't convert user at the meeting start count to the number!!!"); // liczba użytkowników na spotkaniu bez nas (czyli liczba osób na spotkaniu - 1)
            
            //& -- Zmienne pozwalające na wyswietlenie informacji o tym kiedy uzytkownik dołączył na spotkanie w bardziej przystempny sposób!!!                
            let display_now_hour_time: String = if now_hour_time < 10
            {
                format!("0{}", now_hour_time)
            }
            else
            {
                now_hour_time.to_string()
            };
            
            let display_now_minute_time: String  = if now_minute_time < 10
            {
                format!("0{}", now_minute_time)
            }
            else
            {
                now_minute_time.to_string()
            };

            let display_now_sceond_time: String = if now_sceond_time < 10
            {
                format!("0{}", now_sceond_time)
            }
            else
            {
                now_sceond_time.to_string()
            };
            
            println!("You're joined to meeting which name is: {}.\n\nMeeting data:\n\n\tmeeting-start-at: {}\n\tmeeting-end-at: {}\n\tyou-joined-the-meeting-at: {}:{}.{}\n\tusers-in-the-meeting-count: {}", meeting.name.bright_blue(), meeting.start_time.bright_blue(), meeting.end_time.bright_blue(), display_now_hour_time.bright_blue(), display_now_minute_time.bright_blue(), display_now_sceond_time.bright_blue(), format!("{}", when_user_join_users_count).bright_blue()); // user joined to meeting info CLI message
            
            let intreval_time_milliceonds: u64 = 5000;
            let mut last_iteration_user_in_meeting_count: u64 = 0; // liczba uzytkowników na spotkaniu aktualizowana po każdej iteracji
            // sprawdzanie danych spotkania raz na jakiś czas w tym wychodzenie z niego po upływie jego godziny końca
            for _ in 0.. // powtarzanie się czynności co jakiś czas ustalony na górze (stworzone za pomocą infinity loopa!!!)
            {
                show_meeting_bar(&driver).await; // pokazanie bara uzytkownika niezbędenego do opuszczenia spotkania
                // otrzymywanie aktualnego czasu
                let nd_meeting_belong_offset: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
                let nd_meeting_time_zone: String = nd_meeting_belong_offset.time().to_string();
                let nd_time_zone_point_split: Vec<&str> = nd_meeting_time_zone.split(".").collect::<Vec<&str>>()[0].split(":").collect();
                let nd_now_hour_time: u64 = nd_time_zone_point_split[0].parse::<u64>().expect("Wtf!!! Program can't convert actual hour to number");
                let nd_minute_time: u64 = nd_time_zone_point_split[1].parse::<u64>().expect("Wtf!!! Program can't convert actual minute to number");
                let _nd_sceond_time: u64 = nd_time_zone_point_split[2].parse::<u64>().expect("Wtf!!! Program can't convert actual second to number");
                let nd_time_in_minutes: u64 = nd_now_hour_time * 60 + nd_minute_time; // otrzymanie aktualnych minut z dnia czyli gdy jest godzina 17:50 to wynik tego to 1070 minut

                // Otrzymywanie liczby użytkowników znajujących się na spotkaniu teams
                let actual_user_in_meeting_count: u64 = how_much_users_is_in_the_meeting(driver).await.parse::<u64>().expect("Wtf!!! Program coudn't convert user at the meeting count to the number!!!");

                //& Akcja wychodzenia ze spotkania
                if nd_time_in_minutes >= end_meeting_time_in_minutes // opuszczanie spotkania gdy minął czas spotkania
                {
                    leaving_the_meeting(&driver).await;
                    // on_meeting = false; // umożliwianie ponownego dołączania na spotkanie
                    break; // zakończenie infinity loopa ponieważ użytkownik wyszedł ze spotkania
                }
                else if actual_user_in_meeting_count < last_iteration_user_in_meeting_count / 2 && last_iteration_user_in_meeting_count > 1  // wychodzi w momencie gdy liczba użytkowników na spotkaniu zmiejszyła się o połowę względem poprzedniej iteracji gdy jej liczba była większa 1
                {
                    leaving_the_meeting(&driver).await;
                    // on_meeting = false; // umożliwianie ponownego dołączania na spotkanie
                    break; // zakończenie infinity loopa ponieważ użytkownik wyszedł ze spotkania
                }
                else if last_iteration_user_in_meeting_count == 1 && actual_user_in_meeting_count == 0 // wychodzenie w momencie gdy na spotkaniu znajdowała się jedna osoba 
                {
                    leaving_the_meeting(&driver).await;
                    // on_meeting = false; // umożliwianie ponownego dołączania na spotkanie
                    break; // zakończenie infinity loopa ponieważ użytkownik wyszedł ze spotkania
                };

                last_iteration_user_in_meeting_count = actual_user_in_meeting_count; // updatowanie liczby użytkowników (umożliwia weryfikowanie czy liczba użytkowników nie spadła o połowę)
                std::thread::sleep(std::time::Duration::from_millis(intreval_time_milliceonds));
            };
        }

        if meeting_box_check.is_ok() // jeżeli można dołączyć na spotkanie --- dzieje się to najczęściej gdy znajuje się więcej niż jedno spotkanie przez co xpath potrafi go wyszukać za pomocą "[]"
        {
            let meeting_box: WebElement = meeting_box_check.expect("Program can't find meeting box!!!");
            fucking_program_module_head(&driver, meeting_box, end_meeting_time_in_minutes, now_hour_time, now_minute_time, now_sceond_time, meeting).await;
            // on_meeting = false;
            // break;
        }
        else // -- dzieje się tak najczęściej w momecnie gdy znajduje się tylko jedno spotkanie
        {
            let search: String = format!("/html/body/div[1]/div[2]/div[2]/div[1]/div/div/calendar-bridge/div/div/div[4]/div[2]/div/div/div[1]/div/div[3]/div[{}]/div[3]/div", meeting_day_number + 1);
            let meeting_box_check = driver.find_element(By::XPath(search.as_str())).await;
            if meeting_box_check.is_ok() // w momencie gdy program może dołączyć na spotkanie
            {
                let meeting_box: WebElement = meeting_box_check.expect("Program can't find meeting box!!!");

                fucking_program_module_head(&driver, meeting_box, end_meeting_time_in_minutes, now_hour_time, now_minute_time, now_sceond_time, meeting).await;
                // on_meeting = false;
                // break;
            }
            else
            {
                println!("{}", "Program coudn't joined to meeting with some reason!!!".red());
                // on_meeting = false; // ponowne szukanie spotkań!!!
                // break;
            }
        };
    }
    
    //& Odszukiwywanie dni w kalenarzu
    let mut date_matching: bool = false;
        // --- kontener kalendarza
    let container_with_days = driver.find_element(By::ClassName("node_modules--msteams-bridges-components-calendar-grid-dist-es-src-renderers-calendar-multi-day-renderer-calendar-multi-day-renderer__grid--xsw9f")).await;
    if let Ok(elements_from_container) = container_with_days
    {
        // --- kontener z dniami
        let days_meetings_containers: Vec<WebElement> = elements_from_container.find_elements(By::ClassName("node_modules--msteams-bridges-components-calendar-grid-dist-es-src-renderers-calendar-multi-day-renderer-calendar-multi-day-renderer__gridColumn--1PTFP")).await.expect("Program can't find day container!!!");
        
        // --- sprawdzanie czy dzień w kalendarzu jest równy naszemu dniu i dołączanie do spotkania
        for element in days_meetings_containers.iter() // przechodzenie po znalezionych elemntach
        {
            // --- kontener z datą określonego dnia
            let day_meeting_container_date_text: String = element.find_element(By::ClassName("node_modules--msteams-bridges-components-calendar-grid-dist-es-src-renderers-calendar-multi-day-renderer-calendar-multi-day-renderer__ariaDateHeader--BFYlE")).await.expect("Program can't find days meeting date!!!").text().await.expect("Program can't read element text");
            let day_meeting_container_date_number_text_basic: String = day_meeting_container_date_text.split(" ").collect::<Vec<&str>>()[0].trim().to_string(); // oddzielenie daty: 3 grudnia --- podstawowe
            let day_meeting_container_date_number_text_result: String = if day_meeting_container_date_number_text_basic.len() == 1
            {
                let value: String = format!("0{}", day_meeting_container_date_number_text_basic.trim());
                value
            }
            else
            {
                day_meeting_container_date_number_text_basic.trim().to_string()
            };

            if day_meeting_container_date_number_text_basic.len() > 0 // gdy ilość znaków w dacie jest większa od 0
            {
                if day_meeting_container_date_number_text_result == date_time_date // gdy data jest równa dzisiejszej dzacie
                {
                    date_matching = true; // określanie, że data spotkania równa dziejszej dacie jest równa
                    let mut on_meeting: bool = false; // w momencie gdy użytkownik znajduje się na spotkaniu to jest ona ustawiona na true i uniemożliwia ona dołączenie na spotkanie
                    let mut banned_meetings_collection = Vec::<usize>::new(); // kolekcja z numerami iteracji zbanowanych spotkań
                    let end_meeting_break_time: u64 = 15; // ile spotkanie ma być przed końcem aby na nie nie wszedł
                    let mut meeting_is_active_but_not_equal_requirements: bool = true; // (zabezpieczenie przed wielokrotnym wyświetlaniem komunikatów czemu program nie dołącza na spotkanie) w momencie gdy spotkanie jest aktywne i mógłby na nie dołączyć ale nie spełnione zostają wymogi aby mógł na nie dołączyć
                    let mut dis_display_finded_meetings_count_info: bool = true;

                    let element_class: String = element.clone().class_name().await.unwrap().unwrap();

                    // --- powtarzacz czaswoy, który raz na jakiś czas pobiera listę nowytch spotkań i dołącza na spotkania gdy uzytkownik nie znajduje się już na spotkaniu
                    let inf_loop_interval_secs: u64 = 5;
                    for _ in 0..
                    {
                        if !on_meeting // gdy użytkownik nie jest na spotkaniu to wyświetla, że program oczekuje na nowe spotkania 
                        {
                            /* driver.refresh().await.unwrap();
                            std::thread::sleep(std::time::Duration::from_secs(10)); */
                            println!("{}", "The program is waiting for new meetings...".bright_green());
                        }

                        // Numer dnia dzisiejszego spotkania
                        let meeting_day_number: usize = get_meeting_day(driver.clone(), element_class.clone(), date_time_date).await.unwrap();
                        // Lista spotkań w dzisiejszym dniu
                        let meetings_list: Vec<TeamsCalendaryMeeting> = get_meetings_list(driver.clone(), element_class.clone(), meeting_day_number).await;
                        // Dołączanie na spotkanie lub nie dołączanie na spotkanie
                        if meetings_list.len() > 0 
                        {
                            // wyświetlanie się ilości ze znalezionymi spotkaniami
                            if dis_display_finded_meetings_count_info
                            {
                                println!("{}", format!("Scheduled meetings found: {}", meetings_list.len()).bright_green());
                                dis_display_finded_meetings_count_info = false;
                            }
                            // Czas -> sekundy -> minuty -> godziny
                            let actual_time_zone: chrono::DateTime<chrono::Local> = chrono::offset::Local::now();
                            let actual_time_zone_point: String = actual_time_zone.time().to_string();
                            let time_zone_point_split: Vec<&str> = actual_time_zone_point.split(".").collect::<Vec<&str>>()[0].split(":").collect();
                            let now_hour_time: u64 = time_zone_point_split[0].parse::<u64>().expect("Wtf!!! Program can't convert actual hour to number");
                            let now_minute_time: u64 = time_zone_point_split[1].parse::<u64>().expect("Wtf!!! Program can't convert actual minute to number");
                            let now_sceond_time: u64 = time_zone_point_split[2].parse::<u64>().expect("Wtf!!! Program can't convert actual second to number");
                            let now_time_in_minutes: u64 = now_hour_time * 60 + now_minute_time; // otrzymanie aktualnych minut z dnia czyli gdy jest godzina 17:50 to wynik tego to 1070 minut
    
    
                            if on_meeting == false // jeżeli uzytkownik nie jest na spotkaniu
                            {
                                let mut iteration: usize = 0;
                                for meeting in meetings_list.iter() // przechodzenie przez spotkania i dołączanie do tego, którego data zgadza się
                                {
                                    // Czas rozpoczęcia spotkania
                                    let meeting_start_for_clone: String = meeting.start_time.clone();
                                    let meeting_time_start: Vec<&str> = meeting_start_for_clone.split(":").collect();
                                    let start_hour: u64 = meeting_time_start[0].parse::<u64>().expect("Wtf!!! Program can't convert meeting start-hour to number");
                                    let start_minute: u64 = meeting_time_start[1].parse::<u64>().expect("Wtf!!! Program can't convert meeting start-minute to number");
                                    let start_meeting_time_in_minutes: u64 = start_hour * 60 + start_minute; // otrzymanie minut końca spotkania czyli gdy jest godzina 17:30 to wynik tego to 1050 minut
                                    
                                    // Czas zakończenia spotkania
                                    let meeting_end_for_clone: String = meeting.end_time.clone();
                                    let meeting_time_end: Vec<&str> = meeting_end_for_clone.split(":").collect();
                                    let end_hour: u64 = meeting_time_end[0].parse::<u64>().expect("Wtf!!! Program can't convert meeting end-hour to number");
                                    let end_minute: u64 = meeting_time_end[1].parse::<u64>().expect("Wtf!!! Program can't convert meeting end-minute to number");
                                    let end_meeting_time_in_minutes: u64 = end_hour * 60 + end_minute; // otrzymanie minut końca spotkania czyli gdy jest godzina 17:50 to wynik tego to 1070 minut
    
                                    // Ile minut trwa spotkanie?
                                    let how_long_meeting_does_take: u64 = end_meeting_time_in_minutes - start_meeting_time_in_minutes;
    
                                    /* Opis działania:
                                        - Na spotkanie ma nie wchodzić 15 minut przed jego końcem,
                                        - Na spotkanie ma wchodzić gdy długość spotkania jest większa od lub równa czasowi nie wchodzenia przed końcem + 5 minut i gdy spotkanie się rozpoczeło
                                        - Wchodzi pod warunkiem, że spotkanie nie znajduje się na liście zbanowanych spotkań
                                    */
                                    if how_long_meeting_does_take >= end_meeting_break_time + 5 && now_time_in_minutes >= start_meeting_time_in_minutes && now_time_in_minutes < end_meeting_time_in_minutes - end_meeting_break_time && !banned_meetings_collection.contains(&iteration) // gdy czas spotkanie się zgadza oraz spotkanie nie zostało zakwalifikowane jako zbanowane!!! (Na spotkanie wchodzi w momencie gdy nie jest ono zbanowane oraz w momencie gdy zostało mniej niż 15 minut do końca spotkania)
                                    { //& Tutaj użytkownik dołącza na spotkanie ms.teams

                                        if banned_meeting_list.len() > 0 // jeżeli jakieś spotkania zosały wogóle zbanowane
                                        {
                                            let mut searched_meetings_count: usize = 0;
                                            for banned_meeting in banned_meeting_list.iter()
                                            {
                                                /* Opis: 
                                                    Zależności:
                                                        - meeting - instancja znalewzionego spotkania z kalendarza,
                                                        - banned_meeting - instancja zbanowanego spotkania,
                                                    Dokładny opis systemu: plik NOT_JOIN_TO_BANNED_MEETNG_SYSTEM.txt
                                                */
                                                let mut is_this_banned_meeting: bool = false;
                                                if !((meeting.name != "nil" && banned_meeting.name != "nil" && banned_meeting.author != "nil" && banned_meeting.start_time != "nil" && banned_meeting.end_time != "nil" &&banned_meeting.name == meeting.name && banned_meeting.author == meeting.organizator && banned_meeting.start_time == meeting.start_time && banned_meeting.end_time == meeting.end_time) || (meeting.name != "nil" && banned_meeting.name != "nil" && banned_meeting.author != "nil" && meeting.name == banned_meeting.name && meeting.organizator == banned_meeting.author) || (meeting.name != "nil" && banned_meeting.name != "nil" && banned_meeting.author == "nil" && meeting.name == banned_meeting.name) || (meeting.name =="nil" && banned_meeting.author != "nil" && banned_meeting.author == meeting.organizator) || (banned_meeting.name == "nil" && banned_meeting.author != "nil" && banned_meeting.author == meeting.organizator) || ((banned_meeting.end_time != "nil" && banned_meeting.start_time != "nil") && (banned_meeting.end_time == meeting.end_time) && banned_meeting.start_time == meeting.start_time) || (banned_meeting.end_time == "nil" && meeting.start_time == banned_meeting.start_time) || (banned_meeting.start_time == "nil" && meeting.end_time == banned_meeting.end_time))
                                                { // gdy spotkanie nie zostało zakwalifikowane jako zbanowane
                                                    searched_meetings_count += 1;
                                                }
                                                else
                                                {
                                                    is_this_banned_meeting = true;
                                                    println!("{}", format!("This meeting: name: {}, author: {}, time-start: {}, time-end: {} is banned by you in application banned meetings configuration!!!", meeting.name, meeting.organizator, meeting.start_time, meeting.end_time).on_black());
                                                }

                                                // Wrócę to dopieścić ten system i dodać wchodzenie na spotkanie gdy spotkanie nie zostało zbanowane (w ostatecznej formie dodać do jednego wyrażenia pisanego po prostu w 1. linii)
                                                if searched_meetings_count == banned_meeting_list.len() // jeżeli te wartości są sobie równe to oznacza to, że spotkanie nie zostało zakwalifikowane jako zbanowane i użytkownik może do niego dołączyć
                                                {
                                                    on_meeting = true; // ustawianie że użytkwonik znajduje się na spotkaniu co zapobiega przed dołączaniem na kolejne spotkania
                                                    meeting_is_active_but_not_equal_requirements = true; // umożliwianie ponownego wyświetlania komunikatu czemu nie dołaczył na spotkanie
                                                    meeting_handler(&driver, end_meeting_time_in_minutes, now_hour_time, now_minute_time, now_sceond_time, meeting, meeting_day_number, iteration).await;
                                                    on_meeting = false; // po tym jak uzytkownik wyszedł ze spotkania
                                                    break; // zatrzymywanie pętli przechodzącej przez spotkania
                                                }
                                                else // jeżeli liczby te nie są sobie równe to oznacza to, że zostało znalezione zbanowane spotkanie
                                                {
                                                    if !banned_meetings_collection.contains(&iteration) && is_this_banned_meeting // jeżeli to spotkanie nie zostało jeszcze dodane
                                                    {
                                                        banned_meetings_collection.push(iteration); // dodwanie numeru iteracji zbanowanego sptokania w celu zlokalizowania go i nie próbowania dołączyć na nie następnym razem
                                                        break;
                                                    };
                                                }
                                            }
                                        }
                                        else
                                        {
                                            on_meeting = true; // ustawianie że użytkwonik znajduje się na spotkaniu co zapobiega przed dołączaniem na kolejne spotkania
                                            meeting_is_active_but_not_equal_requirements = true; // umożliwianie ponownego wyświetlania komunikatu czemu nie dołaczył na spotkanie
                                            meeting_handler(&driver, end_meeting_time_in_minutes, now_hour_time, now_minute_time, now_sceond_time, meeting, meeting_day_number, iteration).await;
                                            on_meeting = false; // po tym jak uzytkownik wyszedł ze spotkania
                                            break; // zatrzymywanie pętli przechodzącej przez spotkania
                                        };
                                    }
                                    else
                                    {

                                        if meeting_is_active_but_not_equal_requirements // jeżeli nie zostały wyświetlone już komunikaty o tym, że program nie może dołączyć na spotkanie
                                        {
                                            if how_long_meeting_does_take <= end_meeting_break_time + 5 && now_time_in_minutes >= start_meeting_time_in_minutes && now_time_in_minutes <= end_meeting_time_in_minutes // jeżeli spotkanie trawa mniej niż 20 minut oraz się ono zaczęło oraz się ono jeszcze nie skończyło
                                            {
                                                println!("{}", format!("In order for the program to join the meeting, it must last {} minutes or more!!!", end_meeting_break_time + 5).red());
                                                meeting_is_active_but_not_equal_requirements = false;
                                                println!("Meeting name: {}", meeting.name);
                                            }
                                            else if now_time_in_minutes >= start_meeting_time_in_minutes && now_time_in_minutes >= end_meeting_time_in_minutes - end_meeting_break_time && now_time_in_minutes <= end_meeting_time_in_minutes // jeżeli spotkanie trwa lub się rozpoczęło, ale zostało mniej lub 15 minut do jego końca oraz spotkanie się nie skończyło
                                            {
                                                println!("{}", format!("Program coudn't join the meeting {} minutes before the end of the meeting!!!", end_meeting_break_time).red());
                                                meeting_is_active_but_not_equal_requirements = false;
                                                println!("Meeting name: {}", meeting.name);
                                            };
                                            
                                        }
                                    };
                                    iteration += 1; // powiększanie liczby iteracji
                                }
                            }
                        }
                        else
                        {
                            println!("{}", "No scheduled meetings were found on this day!!!".bright_yellow());
                        };

                        std::thread::sleep(std::time::Duration::from_secs(inf_loop_interval_secs));  // sprawdza czy nowe spotkanie odbywa sie co x sekund
                    }

                    break; // zatrzymywanie pętli ponieważ doapsowano datę
                }
            }
            else
            {
                panic!("Program can't read date from callendar day!!!");
            }
        }
    }
    else
    {
        panic!("Program can't read calendar schema!!!");
    }

    // Gdy pętla się zakończyła
    if date_matching == false // gdy nie znaleziono daty równej dziesiejszej dacie
    {
        println!("{}", "match dates equal to today's date!!!".bright_yellow());
        std::process::exit(0);
    }
}