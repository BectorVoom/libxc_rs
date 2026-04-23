//! Phase 4 cross-family oracle verification sweep.
//!
//! Runs `cargo test` against each of the three oracle test binaries
//! (`lda_oracle`, `gga_oracle`, `mgga_oracle`), captures their stderr output,
//! parses the structured `FAMILY {unpol|pol} summary: tested=N
//! skipped_no_exc=N [skipped_deferred=N] [skipped_pending_params=N]
//! skipped_not_compiled=N failures=N` summary lines emitted by each test,
//! and returns a Phase4Report covering the full LDA+GGA+MGGA matrix.
//!
//! Invoked via `cargo xtask verify-phase-4`. No external dependencies beyond
//! anyhow (already in xtask/Cargo.toml). The parser is intentionally tolerant
//! of per-family key variation: LDA emits `skipped_deferred`, GGA emits
//! `skipped_pending_params`, and MGGA emits both. Missing keys parse to 0.

use anyhow::{Context, Result, anyhow, bail};
use std::process::Command;

#[derive(Debug, Default, Clone)]
pub struct FamilyReport {
    pub family: &'static str, // "LDA", "GGA", "MGGA"
    pub unpol_tested: u32,
    pub unpol_skipped_no_exc: u32,
    pub unpol_skipped_deferred: u32,
    pub unpol_skipped_pending_params: u32,
    pub unpol_skipped_not_compiled: u32,
    pub unpol_failures: u32,
    pub pol_tested: u32,
    pub pol_skipped_no_exc: u32,
    pub pol_skipped_deferred: u32,
    pub pol_skipped_pending_params: u32,
    pub pol_skipped_not_compiled: u32,
    pub pol_failures: u32,
    pub duration_s: u64,
}

#[derive(Debug, Default)]
pub struct Phase4Report {
    pub lda: FamilyReport,
    pub gga: FamilyReport,
    pub mgga: FamilyReport,
    pub exit_status: i32,
}

/// Run all three oracle test binaries and parse their structured summary lines.
pub fn run_phase_4_verification() -> Result<Phase4Report> {
    let mut report = Phase4Report::default();
    report.lda = run_family("lda_oracle")?;
    report.gga = run_family("gga_oracle")?;
    report.mgga = run_family("mgga_oracle")?;
    let total_failures = report.lda.unpol_failures
        + report.lda.pol_failures
        + report.gga.unpol_failures
        + report.gga.pol_failures
        + report.mgga.unpol_failures
        + report.mgga.pol_failures;
    report.exit_status = total_failures as i32;
    Ok(report)
}

fn run_family(test_binary: &str) -> Result<FamilyReport> {
    let family: &'static str = match test_binary {
        "lda_oracle" => "LDA",
        "gga_oracle" => "GGA",
        "mgga_oracle" => "MGGA",
        _ => bail!("unknown test binary: {test_binary}"),
    };

    let start = std::time::Instant::now();
    let out = Command::new("cargo")
        .args([
            "test",
            "-p",
            "libxc_rs-verify",
            "--test",
            test_binary,
            "--",
            "--nocapture",
            "--test-threads=1",
        ])
        .output()
        .with_context(|| format!("failed to run cargo test --test {test_binary}"))?;
    let duration_s = start.elapsed().as_secs();

    // Oracle tests print structured summary lines to stderr via eprintln.
    // cargo test may also mirror some output to stdout; concatenate both to
    // make parsing robust against either location.
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);

    let combined = format!("{stderr}\n{stdout}");
    let unpol = parse_summary_line(&combined, family, "unpol").ok_or_else(|| {
        anyhow!(
            "{test_binary}: missing '{family} unpol summary:' line in captured output\n\
             ===== stderr =====\n{stderr}\n===== stdout =====\n{stdout}"
        )
    })?;
    let pol = parse_summary_line(&combined, family, "pol").ok_or_else(|| {
        anyhow!(
            "{test_binary}: missing '{family} pol summary:' line in captured output\n\
             ===== stderr =====\n{stderr}\n===== stdout =====\n{stdout}"
        )
    })?;

    let report = FamilyReport {
        family,
        duration_s,
        unpol_tested: unpol.tested,
        unpol_skipped_no_exc: unpol.skipped_no_exc,
        unpol_skipped_deferred: unpol.skipped_deferred,
        unpol_skipped_pending_params: unpol.skipped_pending_params,
        unpol_skipped_not_compiled: unpol.skipped_not_compiled,
        unpol_failures: unpol.failures,
        pol_tested: pol.tested,
        pol_skipped_no_exc: pol.skipped_no_exc,
        pol_skipped_deferred: pol.skipped_deferred,
        pol_skipped_pending_params: pol.skipped_pending_params,
        pol_skipped_not_compiled: pol.skipped_not_compiled,
        pol_failures: pol.failures,
    };

    if !out.status.success() {
        bail!(
            "{test_binary} exited with status {:?}\n\
             ===== stderr =====\n{stderr}\n===== stdout =====\n{stdout}",
            out.status.code()
        );
    }
    Ok(report)
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Summary {
    tested: u32,
    skipped_no_exc: u32,
    skipped_deferred: u32,
    skipped_pending_params: u32,
    skipped_not_compiled: u32,
    failures: u32,
}

/// Parse a line of the form
/// `FAMILY {unpol|pol} summary: tested=N skipped_no_exc=N [skipped_deferred=N] [skipped_pending_params=N] skipped_not_compiled=N failures=N`.
///
/// Missing keys (for example `skipped_deferred` is absent from GGA output and
/// `skipped_pending_params` is absent from LDA output) parse to 0.
fn parse_summary_line(text: &str, family: &str, spin: &str) -> Option<Summary> {
    let prefix = format!("{family} {spin} summary:");
    let line = text.lines().find(|l| l.trim_start().starts_with(&prefix))?;
    let extract = |key: &str| -> u32 {
        let needle = format!("{key}=");
        line.split_whitespace()
            .find_map(|tok| tok.strip_prefix(&needle))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    };
    Some(Summary {
        tested: extract("tested"),
        skipped_no_exc: extract("skipped_no_exc"),
        skipped_deferred: extract("skipped_deferred"),
        skipped_pending_params: extract("skipped_pending_params"),
        skipped_not_compiled: extract("skipped_not_compiled"),
        failures: extract("failures"),
    })
}

pub fn print_phase_4_summary(report: &Phase4Report) {
    println!("Phase 4 cross-family verification summary");
    println!("=========================================");
    for fr in [&report.lda, &report.gga, &report.mgga] {
        println!();
        println!("{} ({} s):", fr.family, fr.duration_s);
        println!(
            "  unpol: tested={} skipped_no_exc={} skipped_deferred={} \
             skipped_pending_params={} skipped_not_compiled={} failures={}",
            fr.unpol_tested,
            fr.unpol_skipped_no_exc,
            fr.unpol_skipped_deferred,
            fr.unpol_skipped_pending_params,
            fr.unpol_skipped_not_compiled,
            fr.unpol_failures,
        );
        println!(
            "  pol:   tested={} skipped_no_exc={} skipped_deferred={} \
             skipped_pending_params={} skipped_not_compiled={} failures={}",
            fr.pol_tested,
            fr.pol_skipped_no_exc,
            fr.pol_skipped_deferred,
            fr.pol_skipped_pending_params,
            fr.pol_skipped_not_compiled,
            fr.pol_failures,
        );
    }
    println!();
    let total_tested = report.lda.unpol_tested
        + report.lda.pol_tested
        + report.gga.unpol_tested
        + report.gga.pol_tested
        + report.mgga.unpol_tested
        + report.mgga.pol_tested;
    let total_failures = report.lda.unpol_failures
        + report.lda.pol_failures
        + report.gga.unpol_failures
        + report.gga.pol_failures
        + report.mgga.unpol_failures
        + report.mgga.pol_failures;
    println!("TOTAL: tested={total_tested} failures={total_failures}");
    if total_failures == 0 {
        println!("STATUS: Phase 4 oracle matrix GREEN");
    } else {
        println!("STATUS: Phase 4 oracle matrix RED -- {total_failures} failures");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_lda_unpol_summary() {
        let text = "\
some preamble\n\
LDA unpol summary: tested=33 skipped_no_exc=0          skipped_deferred=4          skipped_not_compiled=30          failures=0\n\
other noise\n";
        let got = parse_summary_line(text, "LDA", "unpol").expect("summary parsed");
        assert_eq!(
            got,
            Summary {
                tested: 33,
                skipped_no_exc: 0,
                skipped_deferred: 4,
                skipped_pending_params: 0,
                skipped_not_compiled: 30,
                failures: 0,
            }
        );
    }

    #[test]
    fn parses_gga_pol_summary_without_deferred_key() {
        // GGA summaries omit `skipped_deferred` -- parser must default to 0.
        let text = "GGA pol summary: tested=91 skipped_no_exc=15 skipped_not_compiled=150 skipped_pending_params=0 failures=0";
        let got = parse_summary_line(text, "GGA", "pol").expect("summary parsed");
        assert_eq!(got.tested, 91);
        assert_eq!(got.skipped_no_exc, 15);
        assert_eq!(got.skipped_deferred, 0);
        assert_eq!(got.skipped_pending_params, 0);
        assert_eq!(got.skipped_not_compiled, 150);
        assert_eq!(got.failures, 0);
    }

    #[test]
    fn parses_mgga_unpol_summary_with_all_keys() {
        let text = "MGGA unpol summary: tested=72 skipped_no_exc=4 skipped_not_compiled=64 skipped_pending_params=12 skipped_deferred=6 failures=0";
        let got = parse_summary_line(text, "MGGA", "unpol").expect("summary parsed");
        assert_eq!(got.tested, 72);
        assert_eq!(got.skipped_no_exc, 4);
        assert_eq!(got.skipped_deferred, 6);
        assert_eq!(got.skipped_pending_params, 12);
        assert_eq!(got.skipped_not_compiled, 64);
        assert_eq!(got.failures, 0);
    }

    #[test]
    fn returns_none_when_summary_line_absent() {
        let text = "LDA pol summary: tested=33 failures=0";
        assert!(parse_summary_line(text, "LDA", "unpol").is_none());
        assert!(parse_summary_line(text, "GGA", "pol").is_none());
    }

    #[test]
    fn distinguishes_family_prefixes() {
        let text = "\
GGA unpol summary: tested=91 failures=0\n\
MGGA unpol summary: tested=72 failures=0\n";
        // GGA prefix is a substring of MGGA's "MGGA unpol summary:" (they share
        // "GA unpol summary:"), but starts_with on the full family prefix
        // must distinguish them.
        let gga = parse_summary_line(text, "GGA", "unpol").expect("gga parsed");
        let mgga = parse_summary_line(text, "MGGA", "unpol").expect("mgga parsed");
        assert_eq!(gga.tested, 91);
        assert_eq!(mgga.tested, 72);
    }

    #[test]
    fn parser_tolerates_leading_whitespace() {
        let text = "   LDA unpol summary: tested=10 skipped_deferred=4 failures=0";
        let got = parse_summary_line(text, "LDA", "unpol").expect("indented line parsed");
        assert_eq!(got.tested, 10);
        assert_eq!(got.skipped_deferred, 4);
    }
}
