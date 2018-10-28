use std::fmt;

use chrono;
use yansi::{Color, Paint};

pub const BASE_URL: &str = "https://launchlibrary.net/1.4";

/// Launch collection: <http://launchlibrary.net/docs/1.4/api.html#launch>
#[derive(Deserialize, Debug)]
pub struct Launches {
    offset: u32,
    count: u32,
    total: u32,
    launches: Vec<Launch>,
}

impl IntoIterator for Launches {
    type Item = Launch;
    type IntoIter = ::std::vec::IntoIter<Launch>;
    fn into_iter(self) -> Self::IntoIter {
        self.launches.into_iter()
    }
}

/// Launch endpoint: <http://launchlibrary.net/docs/1.4/api.html#launch>
#[derive(Deserialize, Debug)]
pub struct Launch {
    id: u32,
    name: String,
    net: String,
    #[serde(with = "iso_date_fmt")]
    isostart: chrono::DateTime<chrono::Utc>,
    #[serde(with = "iso_date_fmt")]
    isoend: chrono::DateTime<chrono::Utc>,
    #[serde(with = "iso_date_fmt")]
    isonet: chrono::DateTime<chrono::Utc>,
    tbddate: u32,
    tbdtime: u32,
    #[serde(rename = "vidURLs")]
    vid_urls: Vec<String>,
    rocket: Rocket,
    missions: Vec<Mission>,
}

impl fmt::Display for Launch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {}", Paint::yellow("Launch"), self.name)?;
        write!(
            f,
            "{} {}",
            Paint::cyan("Liftoff:"),
            self.isonet.with_timezone(&chrono::Local).to_rfc2822()
        )?;
        if self.isostart == self.isoend {
            write!(f, " instantaneous");
        } else {
            write!(
                f,
                " {}m window",
                (self.isoend - self.isostart).num_minutes()
            );
        }
        write!(
            f,
            " {}",
            Paint::green(duration_display(chrono::Utc::now() - self.isonet))
        )?;
        if self.tbdtime == 1 {
            writeln!(f, " {}", Paint::black("TBD").bg(Color::Yellow))?;
        } else {
            writeln!(f)?;
        }
        if self.vid_urls.is_empty() {
            writeln!(f, "{} TBD / Unavailable", Paint::cyan("Broadcasts:"));
        } else {
            write!(f, "{}", Paint::cyan("Broadcasts:"))?;
            for url in &self.vid_urls {
                write!(f, " {}", url);
            }
            writeln!(f);
        }
        if f.alternate() {
            writeln!(f, "{} {}", Paint::cyan("Rocket: "), self.rocket.name)?;
            writeln!(f, "{}", Paint::cyan("Missions: "))?;
            for (i, mission) in self.missions.iter().enumerate() {
                writeln!(
                    f,
                    "{} {}",
                    Paint::yellow(format!("{}) [{}]", i + 1, mission.type_name)),
                    mission.description
                )?;
            }
        }

        Ok(())
    }
}

/// Rocket endpoint: <http://launchlibrary.net/docs/1.4/api.html#rocket>
#[derive(Deserialize, Debug)]
struct Rocket {
    id: u32,
    name: String,
    configuration: String,
}

/// Mission endpoint: <http://launchlibrary.net/docs/1.4/api.html#mission>
#[derive(Deserialize, Debug)]
struct Mission {
    id: u32,
    name: String,
    description: String,
    #[serde(rename = "typeName")]
    type_name: String,
}

/// Deserialize time format as per <http://launchlibrary.net/docs/1.4/api.html#launch>
mod iso_date_fmt {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y%m%dT%H%M%SZ";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

fn duration_display(d: chrono::Duration) -> String {
    if d.num_days() != 0 {
        // Show sign of days since it is non-zero
        format!(
            "D{:+} {:02}:{:02}",
            d.num_days(),
            (d.num_hours() % 24).abs(),
            (d.num_minutes() % 60).abs()
        )
    } else {
        // Extract sign from seconds since hours may be zero
        let sign = if d.num_seconds() < 0 { "-" } else { "+" };
        format!(
            "T{}{:02}:{:02}:{:02}",
            sign,
            d.num_hours().abs(),
            (d.num_minutes() % 60).abs(),
            (d.num_seconds() % 60).abs()
        )
    }
}
