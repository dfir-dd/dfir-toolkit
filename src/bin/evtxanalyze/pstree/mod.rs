pub mod process;
pub mod unique_pid;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
    io::stdout,
    ops::Deref,
    rc::{Rc, Weak},
};

use chrono::{DateTime, Utc};
use evtx::EvtxParser;
pub(crate) use process::*;
use regex::Regex;
use serde_json::{json, Value};

use crate::{
    cli::{Command, Format},
    pstree::unique_pid::UniquePid,
};

use super::Cli;

pub(crate) fn display_pstree(cli: &Cli) -> anyhow::Result<()> {
    match &cli.command {
        Command::PsTree {
            username,
            evtx_file,
            format,
        } => {
            let username_regex = username
                .as_ref()
                .map(|s| Regex::new(&format!("(?i){}", s)).expect("invalid username regex"));

            let has_username = |p: &Process| match username_regex.as_ref() {
                None => true,
                Some(username) => {
                    username.is_match(&p.subject_user_name)
                        || username.is_match(&p.target_user_name)
                }
            };

            let mut parser = EvtxParser::from_path(evtx_file)?;
            let mut unique_pids = HashMap::new();
            let mut events = HashMap::new();
            let mut handled_records = 0;
            let mut expected_records: usize = 0;
            for record in parser.records_json_value() {
                expected_records += 1;
                match record {
                    Err(why) => {
                        log::error!("error while parsing a record; read {handled_records} until now. I'll try to continue with the next record");
                        log::warn!("{why}")
                    }
                    Ok(record) => match Process::try_from(record) {
                        Err(why) => log::error!("{why}"),
                        Ok(Some(process)) => {
                            if has_username(&process) {
                                let pid = UniquePid::from(&process);
                                unique_pids
                                    .entry(process.new_process_id)
                                    .or_insert_with(HashSet::new)
                                    .insert(pid.clone());
                                events.insert(pid, Rc::new(RefCell::new(process)));
                            }
                            handled_records += 1;
                        }
                        Ok(None) => handled_records += 1,
                    },
                }
            }

            log::info!("finished reading all records");

            if handled_records < expected_records {
                log::warn!("I expected {expected_records}, but only {handled_records} could be handled.")
            }

            log::warn!("found {} process creations", events.len());

            for new_process in events.values() {
                let parent_pid = new_process.borrow().process_id;
                let timestamp = new_process.borrow().timestamp;

                /* find the unique parent pid. We assume that it is the pid with the
                 * largest timestamp which is less than the current timestamp */
                if let Some(parent_candidates) = unique_pids.get(&parent_pid) {
                    let mut sorted_candidates: Vec<&UniquePid> = parent_candidates
                        .iter()
                        .filter(|p| p.timestamp() <= &timestamp)
                        .collect();
                    sorted_candidates.sort();
                    if let Some(parent_pid) = sorted_candidates.last() {
                        if let Some(parent) = events.get(parent_pid) {
                            new_process.borrow_mut().is_root = false;
                            let child_ts = new_process.borrow().timestamp;
                            let child_process = Rc::downgrade(new_process);
                            parent.borrow_mut().children.insert(child_ts, child_process);
                        } else {
                            log::error!("parent process not found: {parent_pid}");
                        }
                    } else {
                        log::error!("found no parent for {}", new_process.borrow().command_line);
                    }
                }
            }

            let root_processes: BTreeMap<_, _> = events
                .values()
                .filter(|e| e.borrow().is_root)
                .map(|e| {
                    let timestamp = e.borrow().timestamp;
                    let value = Value::from(&*e.borrow());
                    (timestamp, value)
                })
                .collect();

            log::warn!("{} processes have no parent", root_processes.len());

            match format {
                Format::Json => {
                    let root_processes: BTreeMap<_, _> = events
                        .values()
                        .filter(|e| e.borrow().is_root)
                        .map(|e| {
                            let timestamp = e.borrow().timestamp;
                            let value = Value::from(&*e.borrow());
                            (timestamp, value)
                        })
                        .collect();

                    let procs_as_json = json!(root_processes);
                    println!("{}", serde_json::to_string_pretty(&procs_as_json)?);
                }

                Format::Csv => {
                    let mut wtr = csv::Writer::from_writer(stdout().lock());
                    let processes: HashSet<_> = events
                        .values()
                        .map(|p| ProcessTableEntry::from(p.as_ref().borrow().deref()))
                        .collect();
                    for process in processes {
                        wtr.serialize(process)?;
                    }
                }

                Format::Markdown => {
                    let root_processes: BTreeMap<_, _> = events
                        .values()
                        .filter(|e| e.borrow().is_root)
                        .map(|e| {
                            let timestamp = e.borrow().timestamp;
                            let proc = Rc::downgrade(e);
                            (timestamp, proc)
                        })
                        .collect();
                    display_markdown(&root_processes, 0);
                }

                Format::LaTeX => {
                    let root_processes: BTreeMap<_, _> = events
                        .values()
                        .filter(|e| e.borrow().is_root)
                        .map(|e| {
                            let timestamp = e.borrow().timestamp;
                            let proc = Rc::downgrade(e);
                            (timestamp, proc)
                        })
                        .collect();
                    display_latex(&root_processes);
                }

                Format::Dot => {
                    let root_processes: BTreeMap<_, _> = events
                        .values()
                        .filter(|e| e.borrow().is_root)
                        .map(|e| {
                            let timestamp = e.borrow().timestamp;
                            let proc = Rc::downgrade(e);
                            (timestamp, proc)
                        })
                        .collect();
                    println!("digraph {{");
                    println!("rankdir=\"LR\";");
                    display_dot(&root_processes);
                    println!("}}");
                }
            }

            Ok(())
        }
        _ => unreachable!(),
    }
}

fn display_markdown(procs: &BTreeMap<DateTime<Utc>, Weak<RefCell<Process>>>, indent: usize) {
    for proc in procs.values() {
        if let Some(proc) = proc.upgrade() {
            println!("{}- {}", " ".repeat(indent), proc.borrow());
            display_markdown(&proc.borrow().children, indent + 2);
        }
    }
}

fn display_latex(procs: &BTreeMap<DateTime<Utc>, Weak<RefCell<Process>>>) {
    if !procs.is_empty() {
        println!("\\begin{{enumerate}}");
        for proc in procs.values() {
            if let Some(proc) = proc.upgrade() {
                let p = proc.borrow();
                let pid = &p.new_process_id;
                let filename = &p.new_process_name;
                let timestamp = p.timestamp.format("%FT%T");
                let user = p.subject_user_name.replace('_', "\\_").replace('$', "\\$");
                println!("\\item[\\texttt{{{pid}}}] \\filename{{{filename}}}, gestartet: \\ts{{{timestamp}}}, Benutzer: \\username{{{user}}}",);
                display_latex(&proc.borrow().children);
            }
        }
        println!("\\end{{enumerate}}");
    }
}

fn display_dot(procs: &BTreeMap<DateTime<Utc>, Weak<RefCell<Process>>>) {
    for proc in procs.values() {
        if let Some(proc) = proc.upgrade() {
            let p = proc.borrow();
            dot_display_process(&p);
            println!(
                "p{} -> p{} [label=\"{}\"]",
                p.process_id,
                p.new_process_id,
                p.timestamp.format("%FT%T")
            );
            display_dot(&proc.borrow().children);
        }
    }
}

fn dot_display_process(process: &Process) {
    println!(
        "p{} [label=<<FONT FACE=\"Courier\">{}</FONT>>, shape=\"box\"];",
        process.new_process_id,
        process.new_process_name.replace('\\', "\\\\")
    );
}
