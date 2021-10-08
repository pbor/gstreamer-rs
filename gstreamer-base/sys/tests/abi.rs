// Generated by gir (https://github.com/gtk-rs/gir @ 05fe12c0b7e7)
// from gir-files (https://github.com/gtk-rs/gir-files @ b827978e7d18)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ eabb1f9cac5b)
// DO NOT EDIT

use gstreamer_base_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gstreamer-base-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "GstAggregator",
        Layout {
            size: size_of::<GstAggregator>(),
            alignment: align_of::<GstAggregator>(),
        },
    ),
    (
        "GstAggregatorClass",
        Layout {
            size: size_of::<GstAggregatorClass>(),
            alignment: align_of::<GstAggregatorClass>(),
        },
    ),
    (
        "GstAggregatorPad",
        Layout {
            size: size_of::<GstAggregatorPad>(),
            alignment: align_of::<GstAggregatorPad>(),
        },
    ),
    (
        "GstAggregatorPadClass",
        Layout {
            size: size_of::<GstAggregatorPadClass>(),
            alignment: align_of::<GstAggregatorPadClass>(),
        },
    ),
    (
        "GstAggregatorStartTimeSelection",
        Layout {
            size: size_of::<GstAggregatorStartTimeSelection>(),
            alignment: align_of::<GstAggregatorStartTimeSelection>(),
        },
    ),
    (
        "GstBaseParse",
        Layout {
            size: size_of::<GstBaseParse>(),
            alignment: align_of::<GstBaseParse>(),
        },
    ),
    (
        "GstBaseParseClass",
        Layout {
            size: size_of::<GstBaseParseClass>(),
            alignment: align_of::<GstBaseParseClass>(),
        },
    ),
    (
        "GstBaseParseFrame",
        Layout {
            size: size_of::<GstBaseParseFrame>(),
            alignment: align_of::<GstBaseParseFrame>(),
        },
    ),
    (
        "GstBaseParseFrameFlags",
        Layout {
            size: size_of::<GstBaseParseFrameFlags>(),
            alignment: align_of::<GstBaseParseFrameFlags>(),
        },
    ),
    (
        "GstBaseSink",
        Layout {
            size: size_of::<GstBaseSink>(),
            alignment: align_of::<GstBaseSink>(),
        },
    ),
    (
        "GstBaseSinkClass",
        Layout {
            size: size_of::<GstBaseSinkClass>(),
            alignment: align_of::<GstBaseSinkClass>(),
        },
    ),
    (
        "GstBaseSrc",
        Layout {
            size: size_of::<GstBaseSrc>(),
            alignment: align_of::<GstBaseSrc>(),
        },
    ),
    (
        "GstBaseSrcClass",
        Layout {
            size: size_of::<GstBaseSrcClass>(),
            alignment: align_of::<GstBaseSrcClass>(),
        },
    ),
    (
        "GstBaseSrcFlags",
        Layout {
            size: size_of::<GstBaseSrcFlags>(),
            alignment: align_of::<GstBaseSrcFlags>(),
        },
    ),
    (
        "GstBaseTransform",
        Layout {
            size: size_of::<GstBaseTransform>(),
            alignment: align_of::<GstBaseTransform>(),
        },
    ),
    (
        "GstBaseTransformClass",
        Layout {
            size: size_of::<GstBaseTransformClass>(),
            alignment: align_of::<GstBaseTransformClass>(),
        },
    ),
    (
        "GstBitReader",
        Layout {
            size: size_of::<GstBitReader>(),
            alignment: align_of::<GstBitReader>(),
        },
    ),
    (
        "GstBitWriter",
        Layout {
            size: size_of::<GstBitWriter>(),
            alignment: align_of::<GstBitWriter>(),
        },
    ),
    (
        "GstByteReader",
        Layout {
            size: size_of::<GstByteReader>(),
            alignment: align_of::<GstByteReader>(),
        },
    ),
    (
        "GstByteWriter",
        Layout {
            size: size_of::<GstByteWriter>(),
            alignment: align_of::<GstByteWriter>(),
        },
    ),
    (
        "GstCollectData",
        Layout {
            size: size_of::<GstCollectData>(),
            alignment: align_of::<GstCollectData>(),
        },
    ),
    (
        "GstCollectPads",
        Layout {
            size: size_of::<GstCollectPads>(),
            alignment: align_of::<GstCollectPads>(),
        },
    ),
    (
        "GstCollectPadsClass",
        Layout {
            size: size_of::<GstCollectPadsClass>(),
            alignment: align_of::<GstCollectPadsClass>(),
        },
    ),
    (
        "GstCollectPadsStateFlags",
        Layout {
            size: size_of::<GstCollectPadsStateFlags>(),
            alignment: align_of::<GstCollectPadsStateFlags>(),
        },
    ),
    (
        "GstDataQueue",
        Layout {
            size: size_of::<GstDataQueue>(),
            alignment: align_of::<GstDataQueue>(),
        },
    ),
    (
        "GstDataQueueClass",
        Layout {
            size: size_of::<GstDataQueueClass>(),
            alignment: align_of::<GstDataQueueClass>(),
        },
    ),
    (
        "GstDataQueueItem",
        Layout {
            size: size_of::<GstDataQueueItem>(),
            alignment: align_of::<GstDataQueueItem>(),
        },
    ),
    (
        "GstDataQueueSize",
        Layout {
            size: size_of::<GstDataQueueSize>(),
            alignment: align_of::<GstDataQueueSize>(),
        },
    ),
    (
        "GstPushSrc",
        Layout {
            size: size_of::<GstPushSrc>(),
            alignment: align_of::<GstPushSrc>(),
        },
    ),
    (
        "GstPushSrcClass",
        Layout {
            size: size_of::<GstPushSrcClass>(),
            alignment: align_of::<GstPushSrcClass>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) GST_AGGREGATOR_START_TIME_SELECTION_FIRST", "1"),
    ("(gint) GST_AGGREGATOR_START_TIME_SELECTION_SET", "2"),
    ("(gint) GST_AGGREGATOR_START_TIME_SELECTION_ZERO", "0"),
    ("GST_BASE_PARSE_FLAG_DRAINING", "2"),
    ("GST_BASE_PARSE_FLAG_LOST_SYNC", "1"),
    ("(guint) GST_BASE_PARSE_FRAME_FLAG_CLIP", "4"),
    ("(guint) GST_BASE_PARSE_FRAME_FLAG_DROP", "8"),
    ("(guint) GST_BASE_PARSE_FRAME_FLAG_NEW_FRAME", "1"),
    ("(guint) GST_BASE_PARSE_FRAME_FLAG_NONE", "0"),
    ("(guint) GST_BASE_PARSE_FRAME_FLAG_NO_FRAME", "2"),
    ("(guint) GST_BASE_PARSE_FRAME_FLAG_QUEUE", "16"),
    ("(guint) GST_BASE_SRC_FLAG_LAST", "1048576"),
    ("(guint) GST_BASE_SRC_FLAG_STARTED", "32768"),
    ("(guint) GST_BASE_SRC_FLAG_STARTING", "16384"),
    ("GST_BASE_TRANSFORM_SINK_NAME", "sink"),
    ("GST_BASE_TRANSFORM_SRC_NAME", "src"),
    ("(guint) GST_COLLECT_PADS_STATE_EOS", "1"),
    ("(guint) GST_COLLECT_PADS_STATE_FLUSHING", "2"),
    ("(guint) GST_COLLECT_PADS_STATE_LOCKED", "16"),
    ("(guint) GST_COLLECT_PADS_STATE_NEW_SEGMENT", "4"),
    ("(guint) GST_COLLECT_PADS_STATE_WAITING", "8"),
];
