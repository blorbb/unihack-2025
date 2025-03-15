use std::{collections::HashMap, path::Path};

use anyhow::{Result, anyhow};
use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use tokio_stream::{StreamExt, wrappers::ReadDirStream};

pub type UnitCode = String;
pub type Activity = String;
pub type Username = String;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, FromPrimitive)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Class {
    pub day: WeekDay,
    pub code: String,

    /// Minutes from midnight
    pub start: u16,
    /// Minutes from midnight
    pub end: u16,
}

pub type Classes = HashMap<Activity, Vec<Class>>;

#[derive(Clone, Debug)]
pub struct UnitInfo {
    pub name: String,
}

pub enum Preference {
    ShareClass(UnitCode, Activity, Username),
}

pub struct UserInfo {
    pub units: Vec<UnitCode>,
    pub preferences: Vec<Preference>,
}

fn parse_class(data: &serde_json::Value) -> Option<Option<(Activity, Class)>> {
    let part = data.get("part")?.as_str()?;
    if !(part.is_empty() || part == "P1") {
        return None;
    }

    let activity = data.get("type")?.as_str()?.to_owned();
    let day = FromPrimitive::from_u64(data.get("day")?.as_u64()?)?;
    let code = data.get("series")?.as_str()?.to_owned();
    let start = {
        let (hr, min) = data.get("startTime")?.as_str()?.split_once(':')?;
        60 * hr.parse::<u16>().ok()? + min.parse::<u16>().ok()?
    };
    let end = start + 60 * u16::try_from(data.get("duration")?.as_u64()?).ok()?;
    Some(Some((
        activity,
        Class {
            day,
            code,
            start,
            end,
        },
    )))
}

async fn load_unit_classes(file: &Path) -> Result<(UnitCode, UnitInfo, Classes)> {
    let data = serde_json::from_slice::<serde_json::Value>(&tokio::fs::read(file).await?)?;
    let result: Option<(UnitCode, UnitInfo, HashMap<Activity, Vec<Class>>)> = try {
        let code = data.get("code")?.as_str()?[..6].to_owned();
        let name = data.get("title")?.as_str()?.to_owned();
        let classes = data
            .get("activity_data")?
            .as_array()?
            .iter()
            .filter_map(parse_class)
            .collect::<Option<Vec<_>>>()?
            .into_iter()
            .into_group_map();
        (code, UnitInfo { name }, classes)
    };
    result.ok_or(anyhow!("invalid data: {data}"))
}

pub async fn load_classes(dir: &Path) -> Result<HashMap<UnitCode, (UnitInfo, Classes)>> {
    Ok(ReadDirStream::new(tokio::fs::read_dir(dir).await?)
        .then(async |x| Ok(load_unit_classes(&x?.path()).await))
        .collect::<Result<Result<Vec<_>>>>()
        .await??
        .into_iter()
        .map(|(code, info, classes)| (code, (info, classes)))
        .collect::<HashMap<_, _>>())
}
