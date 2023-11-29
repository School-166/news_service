use super::Subject;
use std::str::FromStr;

impl FromStr for Subject {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Mathematics" => Self::Mathematics,
            "Physics" => Self::Physics,
            "Chemistry" => Self::Chemistry,
            "Biology" => Self::Biology,
            "Uzbek" => Self::Uzbek,
            "Russian" => Self::Russian,
            "English" => Self::English,
            "History" => Self::History,
            "Geography" => Self::Geography,
            "Literature" => Self::Literature,
            "PhysicalEducation" => Self::PhysicalEducation,
            "ComputerScience" => Self::ComputerScience,
            "Economics" => Self::Economics,
            "Law" => Self::Law,
            "Education" => Self::Education,
            _ => return Err(()),
        })
    }
}

impl ToString for Subject {
    fn to_string(&self) -> String {
        match self {
            Subject::Mathematics => "Mathematics",
            Subject::Physics => "Physics",
            Subject::Chemistry => "Chemistry",
            Subject::Biology => "Biology",
            Subject::Uzbek => "Uzbek",
            Subject::Russian => "Russian",
            Subject::English => "English",
            Subject::History => "History",
            Subject::Geography => "Geography",
            Subject::Literature => "Literature",
            Subject::PhysicalEducation => "Physical Education",
            Subject::ComputerScience => "Computer Science",
            Subject::Economics => "Economics",
            Subject::Law => "Law",
            Subject::Education => "Education",
        }
        .to_string()
    }
}
