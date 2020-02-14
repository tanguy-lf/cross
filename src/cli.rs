use std::{env, path::PathBuf};

use rustc_version::Channel;

use crate::Target;
use crate::cargo::Subcommand;
use crate::errors::Result;
use crate::rustc::{ChannelExt, TargetList};

#[derive(Debug)]
pub struct Args {
    pub all: Vec<String>,
    pub subcommand: Option<Subcommand>,
    pub channel: Option<Channel>,
    pub target: Option<Target>,
    pub target_dir: Option<PathBuf>,
}

pub fn parse(target_list: &TargetList) -> Result<Args> {
    let mut channel = None;
    let mut target = None;
    let mut target_dir = None;
    let mut sc = None;
    let mut all: Vec<String> = Vec::new();

    {
        let mut args = env::args().skip(1);
        while let Some(arg) = args.next() {
            if arg.starts_with("+") {
                channel = Some(Channel::from_str(&arg[1..])?);
            } else if arg == "--target" {
                all.push(arg);
                if let Some(t) = args.next() {
                    target = Some(Target::from(&t, target_list));
                    all.push(t);
                }
            } else if arg.starts_with("--target=") {
                target = arg.splitn(2, '=').nth(1).map(|s| Target::from(&*s, target_list));
                all.push(arg);
            } else if arg == "--target-dir" {
                all.push(arg);
                if let Some(td) = args.next() {
                    target_dir = Some(PathBuf::from(&td));
                    all.push("/target".to_string());
                }
            } else if arg.starts_with("--target-dir=") {
                if let Some(td) = arg.splitn(2, '=').nth(1) {
                    target_dir = Some(PathBuf::from(&td));
                    all.push(format!("--target-dir=/target"));
                }
            } else {
              if !arg.starts_with('-') && sc.is_none() {
                  sc = Some(Subcommand::from(arg.as_ref()));
              }

              all.push(arg.to_string());
            }
        }
    }

    Ok(Args {
        all,
        subcommand: sc,
        channel,
        target,
        target_dir,
    })
}
