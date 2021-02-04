// bits lifted from the skim project
use crate::database::LinkedItem;
use std::borrow::Cow;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

fn upgrade_check_time_path() -> PathBuf {
    let mut path = projectpadsql::config_path();
    path.push("upgrade-check-date");
    path
}

pub fn upgrade_check_mark_done() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = BufWriter::new(File::create(upgrade_check_time_path())?);
    file.write_all(
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
            .to_string()
            .as_bytes(),
    )?;
    Ok(())
}

pub fn upgrade_days_since_last_check() -> Result<u64, Box<dyn std::error::Error>> {
    let file_path = upgrade_check_time_path();
    if file_path.exists() {
        let file = File::open(file_path)?;
        let mut contents_str = String::new();
        BufReader::new(file).read_to_string(&mut contents_str)?;
        let trimmed = contents_str.trim();
        let previous_seconds = Duration::from_secs(trimmed.parse::<u64>()?);
        let previous_systime = SystemTime::UNIX_EPOCH + previous_seconds;
        Ok(SystemTime::now()
            .duration_since(previous_systime)?
            .as_secs()
            / 3600
            / 24)
    } else {
        Ok(365)
    }
}

fn history_file_path() -> PathBuf {
    let mut path = projectpadsql::config_path();
    path.push("cli-history");
    path
}

pub fn read_history() -> Result<(Vec<String>, Vec<LinkedItem>), std::io::Error> {
    let file = File::open(history_file_path())?;
    let mut history_strs = vec![];
    let mut history_linked_items = vec![];
    for line in BufReader::new(file).lines() {
        let (history_str, linked_item) = parse_history_line(&line?);
        history_strs.push(history_str);
        history_linked_items.push(linked_item);
    }
    Ok((history_strs, history_linked_items))
}

fn parse_history_line(line: &str) -> (String, LinkedItem) {
    let elts: Vec<_> = line.splitn(3, ';').collect();
    match (
        elts.len(),
        elts.get(0),
        elts.get(1).and_then(|i| i.parse::<i32>().ok()),
    ) {
        (3, Some(&"S"), Some(id)) => (elts[2].to_owned(), LinkedItem::ServerId(id)),
        (3, Some(&"P"), Some(id)) => (elts[2].to_owned(), LinkedItem::ProjectPoiId(id)),
        (3, Some(&"SP"), Some(id)) => (elts[2].to_owned(), LinkedItem::ServerPoiId(id)),
        _ => (line.to_owned(), LinkedItem::None),
    }
}

// TODO maybe i should also write which action was used in the history, for ranking, maybe. let's not
// prop up downloading the config file, if i always just want to edit it?
fn serialize_history_line<'a>(line: (&'a String, &LinkedItem)) -> Cow<'a, str> {
    match line.1 {
        LinkedItem::None => Cow::Borrowed(line.0),
        LinkedItem::ServerId(id) => Cow::Owned(format!("S;{};{}", id, line.0)),
        LinkedItem::ProjectPoiId(id) => Cow::Owned(format!("P;{};{}", id, line.0)),
        LinkedItem::ServerPoiId(id) => Cow::Owned(format!("SP;{};{}", id, line.0)),
    }
}

pub fn write_history(
    orig_history_strs: &[String],
    orig_linked_items: &[LinkedItem],
    latest: (&str, LinkedItem),
    limit: usize,
) -> Result<(), std::io::Error> {
    let orig_history: Vec<_> = orig_history_strs
        .iter()
        .zip(orig_linked_items.iter())
        .collect();
    if orig_history.last().map(|(l, i)| (l.as_str(), **i)) == Some(latest) {
        // no point of having at the end of the history 5x the same command...
        return Ok(());
    }
    let additional_lines = if latest.0.trim().is_empty() { 0 } else { 1 };
    let start_index = if orig_history.len() + additional_lines > limit {
        orig_history.len() + additional_lines - limit
    } else {
        0
    };

    let mut history = orig_history[start_index..].to_vec();
    let latest_str = latest.0.to_string();
    history.push((&latest_str, &latest.1));

    let file = File::create(history_file_path())?;
    let mut file = BufWriter::new(file);
    file.write_all(
        history
            .into_iter()
            .map(serialize_history_line)
            .collect::<Vec<_>>()
            .join("\n")
            .as_bytes(),
    )?;
    Ok(())
}
