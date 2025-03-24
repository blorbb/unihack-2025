use std::{collections::HashMap, path::Path};

use anyhow::Result;
use itertools::Itertools;
use num_traits::FromPrimitive;

use crate::shared::activity::{Activity, Class, Classes, UnitCode, UnitInfo};

fn parse_class(data: &serde_json::Value) -> Option<(Activity, Class)> {
    let part = data.get("part")?.as_str()?;
    if !(part.is_empty() || part == "P1") {
        return None;
    }

    let activity = data.get("type")?.as_str()?.to_owned();
    if activity.starts_with("PASS-Optional") || activity == "Assessment" {
        return None;
    }
    let day = FromPrimitive::from_u64(data.get("day")?.as_u64()? - 1)?;
    let code = data.get("series")?.as_str()?.to_owned();
    let start = {
        let (hr, min) = data.get("startTime")?.as_str()?.split_once(':')?;
        60 * hr.parse::<u16>().ok()? + min.parse::<u16>().ok()?
    };
    let end = start + 60 * u16::try_from(data.get("duration")?.as_u64()?).ok()?;
    Some((
        activity,
        Class {
            day,
            code,
            start,
            end,
        },
    ))
}

fn load_unit_classes(file: &Path) -> Option<(UnitCode, UnitInfo, Classes)> {
    let data = serde_json::from_slice::<serde_json::Value>(&std::fs::read(file).ok()?).ok()?;
    let code = data.get("code")?.as_str()?[..7].to_owned();
    let name = data.get("title")?.as_str()?.to_owned();
    let activities = data
        .get("activity_data")?
        .as_array()?
        .iter()
        .filter_map(parse_class)
        .collect::<Vec<_>>()
        .into_iter()
        .into_group_map();
    if activities.is_empty() {
        return None;
    }
    Some((code, UnitInfo { name }, activities))
}

pub fn load_classes(dir: &Path) -> Result<HashMap<UnitCode, (UnitInfo, Classes)>> {
    Ok(std::fs::read_dir(dir)?
        .map(|x| Ok(load_unit_classes(&x?.path())))
        .collect::<Result<Vec<Option<_>>>>()?
        .into_iter()
        .flatten()
        .map(|(code, info, classes)| (code, (info, classes)))
        .collect::<HashMap<_, _>>())
}
