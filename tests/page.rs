//! The project page, `index.html`, says only what is true (docs/engineering.md, "Rules and
//! their guards"):
//! - its hero terminal shows a real run: each `$ coldframe ...` line, run now, prints the
//!   lines that follow it. While coldframe is under construction, the frame may instead preview
//!   a command not built yet, titled "preview", beside the under-construction ribbon;
//! - the ribbon stays exactly until the first real command ships;
//! - its proof band's four numbers are what the repository held at the commit the band names;
//! - it loads the brand's three fonts and nothing else;
//! - every outside address it links is on the list of public ones.

use std::fs;
use std::process::Command;

fn page() -> String {
    fs::read_to_string("index.html").expect("index.html is missing")
}

/// The text between the first `open` and the next `close` after it.
fn between<'a>(text: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = text.find(open)? + open.len();
    let end = text[start..].find(close)? + start;
    Some(&text[start..end])
}

/// HTML with its tags removed and the five entities the page uses read back.
fn plain(html: &str) -> String {
    let mut out = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

/// Each command the terminal shows, and the output it shows under it.
fn runs(terminal: &str) -> Vec<(Vec<String>, String)> {
    let mut found: Vec<(Vec<String>, String)> = Vec::new();
    for line in terminal.lines() {
        if let Some(rest) = line.strip_prefix("$ coldframe") {
            found.push((
                rest.split_whitespace().map(str::to_string).collect(),
                String::new(),
            ));
        } else if let Some((_, out)) = found.last_mut() {
            out.push_str(line);
            out.push('\n');
        }
    }
    found
}

/// The four numbers, counted from the repository as it was at `commit`.
fn counted_at(commit: &str) -> Vec<(String, u64)> {
    let git = |args: &[&str]| {
        let out = Command::new("git").args(args).output().expect("git runs");
        assert!(
            out.status.success(),
            "git {args:?} failed; CI must fetch the whole history"
        );
        String::from_utf8(out.stdout).expect("git prints UTF-8")
    };
    let show = |path: &str| git(&["show", &format!("{commit}:{path}")]);
    let tests: u64 = git(&["ls-tree", "--name-only", commit, "tests/"])
        .lines()
        .filter(|path| path.ends_with(".rs"))
        .map(|path| show(path).lines().filter(|l| l.trim() == "#[test]").count() as u64)
        .sum();
    let states: Vec<String> = show("docs/engineering.md")
        .lines()
        .filter(|line| line.starts_with("| "))
        .filter_map(|line| line.trim_end_matches('|').rsplit('|').next())
        .map(|cell| cell.trim().to_string())
        .collect();
    let held = states
        .iter()
        .filter(|s| s.starts_with("held") && !s.starts_with("held outside"))
        .count() as u64;
    let waiting = states
        .iter()
        .filter(|s| s.starts_with("nothing yet"))
        .count() as u64;
    let cargo = show("Cargo.toml");
    let dependencies = cargo
        .split("\n[")
        .filter(|section| section.starts_with("dependencies]"))
        .flat_map(|section| section.lines().skip(1))
        .filter(|line| line.contains('='))
        .count() as u64;
    vec![
        ("tests".to_string(), tests),
        ("dependencies".to_string(), dependencies),
        ("rules CI checks".to_string(), held),
        ("rules written, not yet checked".to_string(), waiting),
    ]
}

/// The proof band's commit and its numbers, as the page shows them.
fn band(page: &str) -> (String, Vec<(String, u64)>) {
    let section =
        between(page, "<section class=\"proofband\"", "</section>").expect("no proof band");
    let commit = between(section, "commit ", " ")
        .expect("the band names no commit")
        .to_string();
    let figures = between(section, "<dl class=\"figures\">", "</dl>").expect("no figures");
    let numbers = figures
        .split("<div>")
        .filter_map(|pair| {
            let name = between(pair, "<dt>", "</dt>")?.to_string();
            let value = between(pair, "<dd>", "</dd>")?.parse().ok()?;
            Some((name, value))
        })
        .collect();
    (commit, numbers)
}

const FONTS: &str = "https://fonts.googleapis.com/css2?family=Protest+Riot&family=Hanken+Grotesk:wght@400..800&family=JetBrains+Mono:wght@400;500;700&display=swap";

/// Outside addresses the page may link: coldframe's own, the garden's public pages and
/// repositories, the brand's fonts, and the support link.
const PUBLIC: [&str; 16] = [
    "https://github.com/jahala/coldframe",
    "https://jahala.github.io/coldframe",
    "https://github.com/jahala/plotplot",
    "https://github.com/jahala/mrkd",
    "https://plotplot.ai",
    "https://jahala.github.io/weeder/",
    "https://jahala.github.io/pleach/",
    "https://jahala.github.io/umbel/",
    "https://jahala.github.io/tilth/",
    "https://jahala.github.io/mycelium/",
    "https://jahala.github.io/petals/",
    "https://jahala.github.io/well/",
    "https://buymeacoffee.com/jahala",
    "https://fonts.googleapis.com",
    "https://fonts.gstatic.com",
    FONTS,
];

/// Every absolute address in an `href` or `src`.
fn addresses(page: &str) -> Vec<String> {
    let mut found = Vec::new();
    for attr in ["href=\"", "src=\""] {
        for piece in page.split(attr).skip(1) {
            let value = piece.split('"').next().unwrap_or("");
            if value.starts_with("http") {
                found.push(value.to_string());
            }
        }
    }
    found
}

fn is_public(address: &str) -> bool {
    PUBLIC.iter().any(|allowed| {
        address == *allowed
            || address
                .strip_prefix(allowed)
                .is_some_and(|rest| rest.starts_with('/') || rest.starts_with('#'))
    })
}

#[test]
fn the_readers_find_what_they_look_for() {
    let html = "<pre class=\"term-body\"><span>$</span> coldframe --version\ncoldframe 9.9.9\n<span>$</span> coldframe nope\ncoldframe: not a command: nope\n</pre>";
    let terminal = plain(between(html, "<pre class=\"term-body\">", "</pre>").unwrap());
    assert_eq!(
        runs(&terminal),
        [
            (
                vec!["--version".to_string()],
                "coldframe 9.9.9\n".to_string()
            ),
            (
                vec!["nope".to_string()],
                "coldframe: not a command: nope\n".to_string()
            ),
        ]
    );
    assert_eq!(plain("a &lt;b&gt; &amp; c"), "a <b> & c");
    let band_html = "<section class=\"proofband\"><h2>x <span>· commit abc1234 · 5 nov</span></h2><dl class=\"figures\"><div><dt>tests</dt><dd>21</dd></div></dl></section>";
    assert_eq!(
        band(band_html),
        ("abc1234".to_string(), vec![("tests".to_string(), 21)])
    );
    assert!(is_public(
        "https://jahala.github.io/coldframe/docs/tend2/coldframe.tend2.html"
    ));
    assert!(!is_public("https://github.com/jahala/coldframe-notes"));
    assert!(!is_public("https://example.com/"));
}

fn coldframe(args: &[String]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_coldframe"))
        .args(args)
        .output()
        .expect("coldframe runs")
}

/// The commands the binary has shipped: the words of its usage line that are neither a flag,
/// the program's name, nor punctuation.
fn shipped_commands() -> Vec<String> {
    let usage = String::from_utf8_lossy(&coldframe(&["--help".to_string()]).stdout).to_string();
    usage
        .split_whitespace()
        .filter(|word| !["usage:", "coldframe", "|"].contains(word) && !word.starts_with("--"))
        .map(str::to_string)
        .collect()
}

fn has_ribbon(page: &str) -> bool {
    page.contains("<div class=\"wip\"")
}

#[test]
fn the_hero_terminal_shows_a_real_run_or_previews_one_not_built_yet() {
    let page = page();
    let title =
        plain(between(&page, "<span class=\"term-title\">", "</span>").expect("no frame title"));
    let terminal =
        plain(between(&page, "<pre class=\"term-body\">", "</pre>").expect("no hero terminal"));
    let shown = runs(&terminal);
    assert!(!shown.is_empty(), "the hero terminal shows no run");
    let preview = title.starts_with("preview");
    if preview {
        assert!(
            has_ribbon(&page),
            "a preview stands only beside the under-construction ribbon"
        );
    }
    for (args, expected) in shown {
        let out = coldframe(&args);
        if preview {
            assert!(
                out.status.code() == Some(2)
                    && String::from_utf8_lossy(&out.stderr).contains("not a command"),
                "coldframe {args:?} exists now, so the frame shows its real run, not a preview"
            );
        } else {
            let printed = format!(
                "{}{}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            );
            assert_eq!(printed, expected, "coldframe {args:?}");
        }
    }
}

#[test]
fn the_ribbon_stays_until_the_first_real_command_ships() {
    let shipped = shipped_commands();
    assert_eq!(
        has_ribbon(&page()),
        shipped.is_empty(),
        "the under-construction ribbon goes the day a real command ships; shipped: {shipped:?}"
    );
}

#[test]
fn the_proof_band_holds_at_the_commit_it_names() {
    let (commit, shown) = band(&page());
    assert_eq!(
        shown,
        counted_at(&commit),
        "the proof band at commit {commit}"
    );
}

#[test]
fn the_page_loads_the_three_brand_fonts_and_nothing_else() {
    let page = page();
    let stylesheets: Vec<&str> = page
        .split("<link ")
        .skip(1)
        .filter(|tag| tag.contains("rel=\"stylesheet\""))
        .collect();
    assert_eq!(stylesheets.len(), 1, "one stylesheet link, the fonts");
    assert!(stylesheets[0].contains(&format!("href=\"{FONTS}\"")));
    assert!(!page.contains("@import") && !page.contains("@font-face"));
}

#[test]
fn the_favicon_is_a_file_the_site_serves() {
    let page = page();
    let icon = between(&page, "<link rel=\"icon\" href=\"", "\"").expect("no favicon");
    assert!(
        std::path::Path::new(icon).is_file(),
        "the favicon {icon} is not in the repository"
    );
    // GitHub Pages runs Jekyll unless .nojekyll exists, and Jekyll skips every path with a
    // part that starts with a dot, such as .brand/.
    if icon.split('/').any(|part| part.starts_with('.')) {
        assert!(
            std::path::Path::new(".nojekyll").is_file(),
            "{icon} sits in a dot folder, which Pages skips without .nojekyll"
        );
    }
}

#[test]
fn every_outside_link_is_a_public_address() {
    let private: Vec<String> = addresses(&page())
        .into_iter()
        .filter(|a| !is_public(a))
        .collect();
    assert_eq!(private, Vec::<String>::new());
}
