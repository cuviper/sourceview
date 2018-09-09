// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

extern crate gtk_source_sys;
extern crate shell_words;
extern crate tempdir;
use std::env;
use std::error::Error;
use std::path::Path;
use std::mem::{align_of, size_of};
use std::process::Command;
use std::str;
use gtk_source_sys::*;

static PACKAGES: &[&str] = &["gtksourceview-3.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) {
        let arg = match val.into() {
            None => format!("-D{}", var),
            Some(val) => format!("-D{}={}", var, val),
        };
        self.args.push(arg);
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}",
                               &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let mut cmd = Command::new("pkg-config");
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
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
    /// Number of tests that failed to compile.
    failed_to_compile: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn record_failed_to_compile(&mut self) {
        self.failed += 1;
        self.failed_to_compile += 1;
    }
    fn summary(&self) -> String {
        format!(
            "{} passed; {} failed (compilation errors: {})",
            self.passed,
            self.failed,
            self.failed_to_compile)
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
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!("1",
               get_c_value(tmpdir.path(), &cc, "1").expect("C constant"),
               "failed to obtain correct constant value for 1");

    let mut results : Results = Default::default();
    for (i, &(name, rust_value)) in RUST_CONSTANTS.iter().enumerate() {
        match get_c_value(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(ref c_value) => {
                if rust_value == c_value {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_value, c_value);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("constants ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let tmpdir = tempdir::TempDir::new("abi").expect("temporary directory");
    let cc = Compiler::new().expect("configured compiler");

    assert_eq!(Layout {size: 1, alignment: 1},
               get_c_layout(tmpdir.path(), &cc, "char").expect("C layout"),
               "failed to obtain correct layout for char type");

    let mut results : Results = Default::default();
    for (i, &(name, rust_layout)) in RUST_LAYOUTS.iter().enumerate() {
        match get_c_layout(tmpdir.path(), &cc, name) {
            Err(e) => {
                results.record_failed_to_compile();
                eprintln!("{}", e);
            },
            Ok(c_layout) => {
                if rust_layout == c_layout {
                    results.record_passed();
                } else {
                    results.record_failed();
                    eprintln!("Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                              name, rust_layout, &c_layout);
                }
            }
        };
        if (i + 1) % 25 == 0 {
            println!("layout    ... {}", results.summary());
        }
    }
    results.expect_total_success();
}

fn get_c_layout(dir: &Path, cc: &Compiler, name: &str) -> Result<Layout, Box<Error>> {
    let exe = dir.join("layout");
    let mut cc = cc.clone();
    cc.define("ABI_TYPE_NAME", name);
    cc.compile(Path::new("tests/layout.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    let stdout = str::from_utf8(&output.stdout)?;
    let mut words = stdout.trim().split_whitespace();
    let size = words.next().unwrap().parse().unwrap();
    let alignment = words.next().unwrap().parse().unwrap();
    Ok(Layout {size, alignment})
}

fn get_c_value(dir: &Path, cc: &Compiler, name: &str) -> Result<String, Box<Error>> {
    let exe = dir.join("constant");
    let mut cc = cc.clone();
    cc.define("ABI_CONSTANT_NAME", name);
    cc.compile(Path::new("tests/constant.c"), &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}",
                           &abi_cmd, &output).into());
    }

    Ok(str::from_utf8(&output.stdout)?.trim().to_owned())
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("GtkSourceBackgroundPatternType", Layout {size: size_of::<GtkSourceBackgroundPatternType>(), alignment: align_of::<GtkSourceBackgroundPatternType>()}),
    ("GtkSourceBracketMatchType", Layout {size: size_of::<GtkSourceBracketMatchType>(), alignment: align_of::<GtkSourceBracketMatchType>()}),
    ("GtkSourceBuffer", Layout {size: size_of::<GtkSourceBuffer>(), alignment: align_of::<GtkSourceBuffer>()}),
    ("GtkSourceBufferClass", Layout {size: size_of::<GtkSourceBufferClass>(), alignment: align_of::<GtkSourceBufferClass>()}),
    ("GtkSourceChangeCaseType", Layout {size: size_of::<GtkSourceChangeCaseType>(), alignment: align_of::<GtkSourceChangeCaseType>()}),
    ("GtkSourceCompletion", Layout {size: size_of::<GtkSourceCompletion>(), alignment: align_of::<GtkSourceCompletion>()}),
    ("GtkSourceCompletionActivation", Layout {size: size_of::<GtkSourceCompletionActivation>(), alignment: align_of::<GtkSourceCompletionActivation>()}),
    ("GtkSourceCompletionClass", Layout {size: size_of::<GtkSourceCompletionClass>(), alignment: align_of::<GtkSourceCompletionClass>()}),
    ("GtkSourceCompletionContext", Layout {size: size_of::<GtkSourceCompletionContext>(), alignment: align_of::<GtkSourceCompletionContext>()}),
    ("GtkSourceCompletionContextClass", Layout {size: size_of::<GtkSourceCompletionContextClass>(), alignment: align_of::<GtkSourceCompletionContextClass>()}),
    ("GtkSourceCompletionError", Layout {size: size_of::<GtkSourceCompletionError>(), alignment: align_of::<GtkSourceCompletionError>()}),
    ("GtkSourceCompletionInfo", Layout {size: size_of::<GtkSourceCompletionInfo>(), alignment: align_of::<GtkSourceCompletionInfo>()}),
    ("GtkSourceCompletionItem", Layout {size: size_of::<GtkSourceCompletionItem>(), alignment: align_of::<GtkSourceCompletionItem>()}),
    ("GtkSourceCompletionItemClass", Layout {size: size_of::<GtkSourceCompletionItemClass>(), alignment: align_of::<GtkSourceCompletionItemClass>()}),
    ("GtkSourceCompletionProposalIface", Layout {size: size_of::<GtkSourceCompletionProposalIface>(), alignment: align_of::<GtkSourceCompletionProposalIface>()}),
    ("GtkSourceCompletionProviderIface", Layout {size: size_of::<GtkSourceCompletionProviderIface>(), alignment: align_of::<GtkSourceCompletionProviderIface>()}),
    ("GtkSourceCompletionWords", Layout {size: size_of::<GtkSourceCompletionWords>(), alignment: align_of::<GtkSourceCompletionWords>()}),
    ("GtkSourceCompletionWordsClass", Layout {size: size_of::<GtkSourceCompletionWordsClass>(), alignment: align_of::<GtkSourceCompletionWordsClass>()}),
    ("GtkSourceCompressionType", Layout {size: size_of::<GtkSourceCompressionType>(), alignment: align_of::<GtkSourceCompressionType>()}),
    ("GtkSourceDrawSpacesFlags", Layout {size: size_of::<GtkSourceDrawSpacesFlags>(), alignment: align_of::<GtkSourceDrawSpacesFlags>()}),
    ("GtkSourceFile", Layout {size: size_of::<GtkSourceFile>(), alignment: align_of::<GtkSourceFile>()}),
    ("GtkSourceFileClass", Layout {size: size_of::<GtkSourceFileClass>(), alignment: align_of::<GtkSourceFileClass>()}),
    ("GtkSourceFileLoader", Layout {size: size_of::<GtkSourceFileLoader>(), alignment: align_of::<GtkSourceFileLoader>()}),
    ("GtkSourceFileLoaderClass", Layout {size: size_of::<GtkSourceFileLoaderClass>(), alignment: align_of::<GtkSourceFileLoaderClass>()}),
    ("GtkSourceFileLoaderError", Layout {size: size_of::<GtkSourceFileLoaderError>(), alignment: align_of::<GtkSourceFileLoaderError>()}),
    ("GtkSourceFileSaver", Layout {size: size_of::<GtkSourceFileSaver>(), alignment: align_of::<GtkSourceFileSaver>()}),
    ("GtkSourceFileSaverClass", Layout {size: size_of::<GtkSourceFileSaverClass>(), alignment: align_of::<GtkSourceFileSaverClass>()}),
    ("GtkSourceFileSaverError", Layout {size: size_of::<GtkSourceFileSaverError>(), alignment: align_of::<GtkSourceFileSaverError>()}),
    ("GtkSourceFileSaverFlags", Layout {size: size_of::<GtkSourceFileSaverFlags>(), alignment: align_of::<GtkSourceFileSaverFlags>()}),
    ("GtkSourceGutter", Layout {size: size_of::<GtkSourceGutter>(), alignment: align_of::<GtkSourceGutter>()}),
    ("GtkSourceGutterClass", Layout {size: size_of::<GtkSourceGutterClass>(), alignment: align_of::<GtkSourceGutterClass>()}),
    ("GtkSourceGutterRenderer", Layout {size: size_of::<GtkSourceGutterRenderer>(), alignment: align_of::<GtkSourceGutterRenderer>()}),
    ("GtkSourceGutterRendererAlignmentMode", Layout {size: size_of::<GtkSourceGutterRendererAlignmentMode>(), alignment: align_of::<GtkSourceGutterRendererAlignmentMode>()}),
    ("GtkSourceGutterRendererClass", Layout {size: size_of::<GtkSourceGutterRendererClass>(), alignment: align_of::<GtkSourceGutterRendererClass>()}),
    ("GtkSourceGutterRendererPixbuf", Layout {size: size_of::<GtkSourceGutterRendererPixbuf>(), alignment: align_of::<GtkSourceGutterRendererPixbuf>()}),
    ("GtkSourceGutterRendererPixbufClass", Layout {size: size_of::<GtkSourceGutterRendererPixbufClass>(), alignment: align_of::<GtkSourceGutterRendererPixbufClass>()}),
    ("GtkSourceGutterRendererState", Layout {size: size_of::<GtkSourceGutterRendererState>(), alignment: align_of::<GtkSourceGutterRendererState>()}),
    ("GtkSourceGutterRendererText", Layout {size: size_of::<GtkSourceGutterRendererText>(), alignment: align_of::<GtkSourceGutterRendererText>()}),
    ("GtkSourceGutterRendererTextClass", Layout {size: size_of::<GtkSourceGutterRendererTextClass>(), alignment: align_of::<GtkSourceGutterRendererTextClass>()}),
    ("GtkSourceLanguage", Layout {size: size_of::<GtkSourceLanguage>(), alignment: align_of::<GtkSourceLanguage>()}),
    ("GtkSourceLanguageClass", Layout {size: size_of::<GtkSourceLanguageClass>(), alignment: align_of::<GtkSourceLanguageClass>()}),
    ("GtkSourceLanguageManager", Layout {size: size_of::<GtkSourceLanguageManager>(), alignment: align_of::<GtkSourceLanguageManager>()}),
    ("GtkSourceLanguageManagerClass", Layout {size: size_of::<GtkSourceLanguageManagerClass>(), alignment: align_of::<GtkSourceLanguageManagerClass>()}),
    ("GtkSourceMap", Layout {size: size_of::<GtkSourceMap>(), alignment: align_of::<GtkSourceMap>()}),
    ("GtkSourceMark", Layout {size: size_of::<GtkSourceMark>(), alignment: align_of::<GtkSourceMark>()}),
    ("GtkSourceMarkAttributes", Layout {size: size_of::<GtkSourceMarkAttributes>(), alignment: align_of::<GtkSourceMarkAttributes>()}),
    ("GtkSourceMarkAttributesClass", Layout {size: size_of::<GtkSourceMarkAttributesClass>(), alignment: align_of::<GtkSourceMarkAttributesClass>()}),
    ("GtkSourceMarkClass", Layout {size: size_of::<GtkSourceMarkClass>(), alignment: align_of::<GtkSourceMarkClass>()}),
    ("GtkSourceNewlineType", Layout {size: size_of::<GtkSourceNewlineType>(), alignment: align_of::<GtkSourceNewlineType>()}),
    ("GtkSourcePrintCompositor", Layout {size: size_of::<GtkSourcePrintCompositor>(), alignment: align_of::<GtkSourcePrintCompositor>()}),
    ("GtkSourcePrintCompositorClass", Layout {size: size_of::<GtkSourcePrintCompositorClass>(), alignment: align_of::<GtkSourcePrintCompositorClass>()}),
    ("GtkSourceRegion", Layout {size: size_of::<GtkSourceRegion>(), alignment: align_of::<GtkSourceRegion>()}),
    ("GtkSourceRegionClass", Layout {size: size_of::<GtkSourceRegionClass>(), alignment: align_of::<GtkSourceRegionClass>()}),
    ("GtkSourceRegionIter", Layout {size: size_of::<GtkSourceRegionIter>(), alignment: align_of::<GtkSourceRegionIter>()}),
    ("GtkSourceSearchContext", Layout {size: size_of::<GtkSourceSearchContext>(), alignment: align_of::<GtkSourceSearchContext>()}),
    ("GtkSourceSearchContextClass", Layout {size: size_of::<GtkSourceSearchContextClass>(), alignment: align_of::<GtkSourceSearchContextClass>()}),
    ("GtkSourceSearchSettings", Layout {size: size_of::<GtkSourceSearchSettings>(), alignment: align_of::<GtkSourceSearchSettings>()}),
    ("GtkSourceSearchSettingsClass", Layout {size: size_of::<GtkSourceSearchSettingsClass>(), alignment: align_of::<GtkSourceSearchSettingsClass>()}),
    ("GtkSourceSmartHomeEndType", Layout {size: size_of::<GtkSourceSmartHomeEndType>(), alignment: align_of::<GtkSourceSmartHomeEndType>()}),
    ("GtkSourceSortFlags", Layout {size: size_of::<GtkSourceSortFlags>(), alignment: align_of::<GtkSourceSortFlags>()}),
    ("GtkSourceSpaceDrawer", Layout {size: size_of::<GtkSourceSpaceDrawer>(), alignment: align_of::<GtkSourceSpaceDrawer>()}),
    ("GtkSourceSpaceDrawerClass", Layout {size: size_of::<GtkSourceSpaceDrawerClass>(), alignment: align_of::<GtkSourceSpaceDrawerClass>()}),
    ("GtkSourceSpaceLocationFlags", Layout {size: size_of::<GtkSourceSpaceLocationFlags>(), alignment: align_of::<GtkSourceSpaceLocationFlags>()}),
    ("GtkSourceSpaceTypeFlags", Layout {size: size_of::<GtkSourceSpaceTypeFlags>(), alignment: align_of::<GtkSourceSpaceTypeFlags>()}),
    ("GtkSourceStyleScheme", Layout {size: size_of::<GtkSourceStyleScheme>(), alignment: align_of::<GtkSourceStyleScheme>()}),
    ("GtkSourceStyleSchemeChooserButton", Layout {size: size_of::<GtkSourceStyleSchemeChooserButton>(), alignment: align_of::<GtkSourceStyleSchemeChooserButton>()}),
    ("GtkSourceStyleSchemeChooserInterface", Layout {size: size_of::<GtkSourceStyleSchemeChooserInterface>(), alignment: align_of::<GtkSourceStyleSchemeChooserInterface>()}),
    ("GtkSourceStyleSchemeChooserWidget", Layout {size: size_of::<GtkSourceStyleSchemeChooserWidget>(), alignment: align_of::<GtkSourceStyleSchemeChooserWidget>()}),
    ("GtkSourceStyleSchemeClass", Layout {size: size_of::<GtkSourceStyleSchemeClass>(), alignment: align_of::<GtkSourceStyleSchemeClass>()}),
    ("GtkSourceStyleSchemeManager", Layout {size: size_of::<GtkSourceStyleSchemeManager>(), alignment: align_of::<GtkSourceStyleSchemeManager>()}),
    ("GtkSourceStyleSchemeManagerClass", Layout {size: size_of::<GtkSourceStyleSchemeManagerClass>(), alignment: align_of::<GtkSourceStyleSchemeManagerClass>()}),
    ("GtkSourceTag", Layout {size: size_of::<GtkSourceTag>(), alignment: align_of::<GtkSourceTag>()}),
    ("GtkSourceTagClass", Layout {size: size_of::<GtkSourceTagClass>(), alignment: align_of::<GtkSourceTagClass>()}),
    ("GtkSourceUndoManagerIface", Layout {size: size_of::<GtkSourceUndoManagerIface>(), alignment: align_of::<GtkSourceUndoManagerIface>()}),
    ("GtkSourceView", Layout {size: size_of::<GtkSourceView>(), alignment: align_of::<GtkSourceView>()}),
    ("GtkSourceViewGutterPosition", Layout {size: size_of::<GtkSourceViewGutterPosition>(), alignment: align_of::<GtkSourceViewGutterPosition>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("GTK_SOURCE_BACKGROUND_PATTERN_TYPE_GRID", "1"),
    ("GTK_SOURCE_BACKGROUND_PATTERN_TYPE_NONE", "0"),
    ("GTK_SOURCE_BRACKET_MATCH_FOUND", "3"),
    ("GTK_SOURCE_BRACKET_MATCH_NONE", "0"),
    ("GTK_SOURCE_BRACKET_MATCH_NOT_FOUND", "2"),
    ("GTK_SOURCE_BRACKET_MATCH_OUT_OF_RANGE", "1"),
    ("GTK_SOURCE_CHANGE_CASE_LOWER", "0"),
    ("GTK_SOURCE_CHANGE_CASE_TITLE", "3"),
    ("GTK_SOURCE_CHANGE_CASE_TOGGLE", "2"),
    ("GTK_SOURCE_CHANGE_CASE_UPPER", "1"),
    ("GTK_SOURCE_COMPLETION_ACTIVATION_INTERACTIVE", "1"),
    ("GTK_SOURCE_COMPLETION_ACTIVATION_NONE", "0"),
    ("GTK_SOURCE_COMPLETION_ACTIVATION_USER_REQUESTED", "2"),
    ("GTK_SOURCE_COMPLETION_ERROR_ALREADY_BOUND", "0"),
    ("GTK_SOURCE_COMPLETION_ERROR_NOT_BOUND", "1"),
    ("GTK_SOURCE_COMPRESSION_TYPE_GZIP", "1"),
    ("GTK_SOURCE_COMPRESSION_TYPE_NONE", "0"),
    ("GTK_SOURCE_DRAW_SPACES_ALL", "127"),
    ("GTK_SOURCE_DRAW_SPACES_LEADING", "16"),
    ("GTK_SOURCE_DRAW_SPACES_NBSP", "8"),
    ("GTK_SOURCE_DRAW_SPACES_NEWLINE", "4"),
    ("GTK_SOURCE_DRAW_SPACES_SPACE", "1"),
    ("GTK_SOURCE_DRAW_SPACES_TAB", "2"),
    ("GTK_SOURCE_DRAW_SPACES_TEXT", "32"),
    ("GTK_SOURCE_DRAW_SPACES_TRAILING", "64"),
    ("GTK_SOURCE_FILE_LOADER_ERROR_CONVERSION_FALLBACK", "2"),
    ("GTK_SOURCE_FILE_LOADER_ERROR_ENCODING_AUTO_DETECTION_FAILED", "1"),
    ("GTK_SOURCE_FILE_LOADER_ERROR_TOO_BIG", "0"),
    ("GTK_SOURCE_FILE_SAVER_ERROR_EXTERNALLY_MODIFIED", "1"),
    ("GTK_SOURCE_FILE_SAVER_ERROR_INVALID_CHARS", "0"),
    ("GTK_SOURCE_FILE_SAVER_FLAGS_CREATE_BACKUP", "4"),
    ("GTK_SOURCE_FILE_SAVER_FLAGS_IGNORE_INVALID_CHARS", "1"),
    ("GTK_SOURCE_FILE_SAVER_FLAGS_IGNORE_MODIFICATION_TIME", "2"),
    ("GTK_SOURCE_FILE_SAVER_FLAGS_NONE", "0"),
    ("GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_CELL", "0"),
    ("GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_FIRST", "1"),
    ("GTK_SOURCE_GUTTER_RENDERER_ALIGNMENT_MODE_LAST", "2"),
    ("GTK_SOURCE_GUTTER_RENDERER_STATE_CURSOR", "1"),
    ("GTK_SOURCE_GUTTER_RENDERER_STATE_NORMAL", "0"),
    ("GTK_SOURCE_GUTTER_RENDERER_STATE_PRELIT", "2"),
    ("GTK_SOURCE_GUTTER_RENDERER_STATE_SELECTED", "4"),
    ("GTK_SOURCE_NEWLINE_TYPE_CR", "1"),
    ("GTK_SOURCE_NEWLINE_TYPE_CR_LF", "2"),
    ("GTK_SOURCE_NEWLINE_TYPE_LF", "0"),
    ("GTK_SOURCE_SMART_HOME_END_AFTER", "2"),
    ("GTK_SOURCE_SMART_HOME_END_ALWAYS", "3"),
    ("GTK_SOURCE_SMART_HOME_END_BEFORE", "1"),
    ("GTK_SOURCE_SMART_HOME_END_DISABLED", "0"),
    ("GTK_SOURCE_SORT_FLAGS_CASE_SENSITIVE", "1"),
    ("GTK_SOURCE_SORT_FLAGS_NONE", "0"),
    ("GTK_SOURCE_SORT_FLAGS_REMOVE_DUPLICATES", "4"),
    ("GTK_SOURCE_SORT_FLAGS_REVERSE_ORDER", "2"),
    ("GTK_SOURCE_SPACE_LOCATION_ALL", "7"),
    ("GTK_SOURCE_SPACE_LOCATION_INSIDE_TEXT", "2"),
    ("GTK_SOURCE_SPACE_LOCATION_LEADING", "1"),
    ("GTK_SOURCE_SPACE_LOCATION_NONE", "0"),
    ("GTK_SOURCE_SPACE_LOCATION_TRAILING", "4"),
    ("GTK_SOURCE_SPACE_TYPE_ALL", "15"),
    ("GTK_SOURCE_SPACE_TYPE_NBSP", "8"),
    ("GTK_SOURCE_SPACE_TYPE_NEWLINE", "4"),
    ("GTK_SOURCE_SPACE_TYPE_NONE", "0"),
    ("GTK_SOURCE_SPACE_TYPE_SPACE", "1"),
    ("GTK_SOURCE_SPACE_TYPE_TAB", "2"),
    ("GTK_SOURCE_VIEW_GUTTER_POSITION_LINES", "-30"),
    ("GTK_SOURCE_VIEW_GUTTER_POSITION_MARKS", "-20"),
];

