/* This module contains dependencies for other files */
use crate::*;


#[derive(Debug)]
pub struct TeamsCalendaryMeeting {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
    pub organizator: String
}

/* lista ze spotkaniami na, które bot ma niedołączać */
#[derive(Debug, Serialize, Deserialize)]
pub struct BannedMeetingCollection {
    pub meet: Vec<NoJoinToThisMeetings>
}

/* struktóra pojdedynczego spotkania na, które bot ma nie dołączać */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoJoinToThisMeetings {
    pub name: String,
    pub author: String,
    pub start_time: String,
    pub end_time: String
}

/* struktóra z funkcjami zadającymi pytania w celu skonfigurowania zbanowanych spotkań aplikacji */
pub struct BannedMeetingsConfig;

pub enum BannedMeetingsConfigTime { /* typ czasu spotkania */
    Start,
    End
}

#[allow(dead_code)]
pub enum BannedMeetingsConfigTrybe { /* tryb updatowania zbanowanego sptotkania */
    Create,
    Update
}

impl BannedMeetingsConfig { /* zawiera wszystkie pytania z wyjątkiem tych końcowych */
    pub fn meeting_name(self, q: BannedMeetingsConfigTrybe) -> String // konfigurowanie nazwy spotkania
    {
        let meeting_name_q = Question::new("Enter a meeting name:").ask().unwrap();
        if let Answer::RESPONSE(m_name) = meeting_name_q
        {
            if m_name.len() > 0
            {
                if let BannedMeetingsConfigTrybe::Create = q
                {
                    println!("Meeting name has been set as: {}", m_name);
                }
                m_name
            }
            else
            {
                if let BannedMeetingsConfigTrybe::Create = q
                {
                    println!("You didn't provide meeting name so meeting name has been set to \"nil\"!!!");
                }
                String::from("nil")
            }
        }
        else
        {
            panic!("Oh no!!! Something went wrong while setting up the meeting name")
        }
    }

    pub fn meeting_author(self, q: BannedMeetingsConfigTrybe) -> String // konfigurowanie nazwy autora spotkania
    {
        let meeting_author_q = Question::new("Enter a meeting author:").ask().unwrap();
        if let Answer::RESPONSE(m_author) = meeting_author_q
        {
            if m_author.len() > 0
            {
                if let BannedMeetingsConfigTrybe::Create = q
                {
                    println!("Meeting author has been set as: {}", m_author);
                }
                m_author
            }
            else
            {
                if let BannedMeetingsConfigTrybe::Create = q
                {
                    println!("You didn't provide meeting author name so meeting author name has been set to \"nil\"!!!");
                }
                String::from("nil")
            }
        }
        else
        {
            panic!("Oh no!!! Something went wrong while setting up the meeting author name")
        }
    }

    pub fn meeting_time(self, t: BannedMeetingsConfigTime, q: BannedMeetingsConfigTrybe) -> String // konfigurowanie czasu spotkania
    {
        // -- określanie czy 
        let hr_type: &str = 
        if let BannedMeetingsConfigTime::End = t
        {
            "end time"
        }
        else if let BannedMeetingsConfigTime::Start = t
        {
            "start time"
        }
        else
        {
            panic!("Something went wrongs!!!")
        };

        let meeting_start_time_q = Question::new(&format!("Enter a meeting {}:", hr_type)).ask().unwrap();
        if let Answer::RESPONSE(start_time) = meeting_start_time_q
        {
            if start_time.len() > 0 // w momencie gdy podano wogóle datę spotkania
            {
                if start_time.contains(":") // sprawdzanie popewaności 
                {
                    let splited = start_time.split(":").collect::<Vec<&str>>();
                    if splited.len() == 2 // w momencie gdy podamo dwa argumenty to oznacza to że schamet daty został dobrze podany
                    {
                        let parsed_hour = splited[0].parse::<u8>();
                        let parsed_minute = splited[1].parse::<u8>();
                        if parsed_hour.is_ok() && parsed_minute.is_ok()
                        {
                            if parsed_hour.unwrap() < 24 && parsed_minute.unwrap() < 60
                            {   
                                if let BannedMeetingsConfigTrybe::Create = q
                                {
                                    println!("Meeting {} has been set as: {}", hr_type, start_time);
                                }
                                start_time
                            }
                            else
                            {
                                println!("{}", "Minute must by less then 60 so it must be in range 0-59, hour must be less then 24 so it must be in range 0-23!!!".red());
                                self.meeting_time(t, q)
                            }
                        }
                        else
                        {
                            if parsed_hour.is_err() // w momencie gdy nie udało się parsowanie podanej godziny spotkania
                            {
                                println!("{}", "The meeting hour must be builded only with numbers!!!".red());
                            }
                            else if parsed_minute.is_err() // w momencie gdy nie udało się parsowanie minuty rozpoczęcia spotaknia do postaci numeru
                            {
                                println!("{}", "The meeting minute must be builded only with numbers!!!".red());
                            };
                            self.meeting_time(t, q)
                        }
                    }
                    else
                    {
                        if splited.len() > 2 // powiadomienie że podano za dużo argumentów
                        {
                            println!("{}", format!("You must add only 2 arguments based on this schema: date:hour but you add: {} arguments. Wtf Dude!!!", splited.len()));
                        } 
                        else // powiadmienie że podano za mało argumentów
                        {
                            println!("{}", format!("You must add 2 arguments based on this schema: date:hour but you add: {} arguments. Wtf Dude!!!", splited.len()));
                        };
                        self.meeting_time(t, q)
                    }
                }
                else
                {
                    println!("{}", "The date must be created by this schema: hour:minute".red());
                    self.meeting_time(t, q)
                }
            }
            else
            {
                if let BannedMeetingsConfigTrybe::Create = q
                {
                    println!("You didn't provide meeting {a} so meeting {a} has been set to \"nil\"!!!", a = hr_type);
                };
                String::from("nil")
            }
        
        }
        else
        {
            panic!("Oh no!!! Something went wrong while setting up the meeting {}. Try again configure your application banned meetings list!!!", hr_type)
        }
    }

    /* Zapisywanie spotkań w innym typie musi zostać obsłużone w miejscy gdzie są wywoływane te funkcje i ze względu na swoją specyfikę mogą się od siebie znacznie różnić więc tworzenie funkcji do zapsisywania tych danych było by ciężkie do ujednolicenia i kosztowało by to stworzeniem złożonego, skoplikowany kodu, którego było by ciężko odczytać */
}