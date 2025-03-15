use std::{collections::HashMap, path::Path};

use anyhow::{Result, anyhow};
use itertools::Itertools;
use num_traits::FromPrimitive;
use tokio_stream::{StreamExt, wrappers::ReadDirStream};

use crate::shared::activity::{Activity, Class, Classes, UnitCode, UnitInfo};

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

fn load_unit_classes(file: &Path) -> Result<(UnitCode, UnitInfo, Classes)> {
    let data = serde_json::from_slice::<serde_json::Value>(&std::fs::read(file)?)?;
    let result: Option<(UnitCode, UnitInfo, HashMap<Activity, Vec<Class>>)> = try {
        let code = data.get("code")?.as_str()?[..7].to_owned();
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

pub fn load_classes(dir: &Path) -> Result<HashMap<UnitCode, (UnitInfo, Classes)>> {
    Ok(std::fs::read_dir(dir)?
        .map(|x| Ok(load_unit_classes(&x?.path())))
        .collect::<Result<Result<Vec<_>>>>()??
        .into_iter()
        .map(|(code, info, classes)| (code, (info, classes)))
        .collect::<HashMap<_, _>>())
}
