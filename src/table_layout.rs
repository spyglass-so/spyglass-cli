//! Fitting human `table` output to the terminal.
//!
//! When stdout is a terminal, a table is laid out within its width: cells
//! keep to one line, the header is bold, and when the columns don't fit the
//! widest cells are shortened and the least useful columns are hidden, with
//! a note on stderr. Piped output keeps every column at full width.

use std::io::IsTerminal;

/// Spaces between columns.
pub(crate) const GAP: usize = 2;

/// Narrowest a column is shortened to before it is hidden instead.
const MIN_WIDTH: usize = 20;

/// How a table is laid out on the current stdout.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct TableStyle {
    /// Terminal width. `None` when stdout is not a terminal.
    pub width: Option<usize>,
    /// Bold the header row.
    pub bold: bool,
}

impl TableStyle {
    pub(crate) fn detect() -> Self {
        // Unit tests compare exact output, whatever terminal runs them.
        if cfg!(test) || !std::io::stdout().is_terminal() {
            return Self::default();
        }
        let no_color = std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty());
        let dumb = std::env::var_os("TERM").is_some_and(|t| t == "dumb");
        Self {
            width: crate::banner::stdout_terminal_size().map(|(cols, _)| cols),
            bold: !no_color && !dumb,
        }
    }

    pub(crate) fn header(&self, line: &str) -> String {
        if self.bold && !line.is_empty() {
            format!("\x1b[1m{line}\x1b[0m")
        } else {
            line.to_string()
        }
    }
}

/// One column as [`fit`] sees it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ColumnFit {
    /// Width that shows every value (already capped).
    pub natural: usize,
    /// Narrowest it may be shortened to.
    pub min: usize,
    /// Lower keeps the column and its width longer.
    pub rank: usize,
}

impl ColumnFit {
    /// A column whose values may be shortened down to [`MIN_WIDTH`], or to its
    /// header when that is wider.
    pub(crate) fn shrinkable(natural: usize, header: usize, rank: usize) -> Self {
        Self {
            natural,
            min: natural.min(header.max(MIN_WIDTH)),
            rank,
        }
    }
}

/// Widths for `columns` within `available` characters. `None` marks a hidden
/// column. When everything fits at its natural width, nothing changes.
///
/// Columns are kept in rank order while their narrowest width still fits;
/// the first one that doesn't, and every lower-ranked one, is hidden. Space
/// left over then widens the kept columns back toward their natural width,
/// best-ranked first.
pub(crate) fn fit(columns: &[ColumnFit], available: usize) -> Vec<Option<usize>> {
    let natural: usize = columns.iter().map(|c| c.natural).sum();
    if natural + GAP * columns.len().saturating_sub(1) <= available {
        return columns.iter().map(|c| Some(c.natural)).collect();
    }
    let mut order: Vec<usize> = (0..columns.len()).collect();
    order.sort_by_key(|&i| columns[i].rank);

    let mut widths: Vec<Option<usize>> = vec![None; columns.len()];
    let mut used = 0;
    for &i in &order {
        let gap = if used == 0 { 0 } else { GAP };
        if used + gap + columns[i].min > available {
            if used == 0 {
                // Not even one column fits: show the best one, cut to the width.
                widths[i] = Some(available.max(1));
                used = available;
            }
            break;
        }
        used += gap + columns[i].min;
        widths[i] = Some(columns[i].min);
    }
    let mut spare = available.saturating_sub(used);
    for &i in &order {
        let Some(width) = widths[i].as_mut() else {
            continue;
        };
        let grow = columns[i].natural.saturating_sub(*width).min(spare);
        *width += grow;
        spare -= grow;
    }
    widths
}

/// Rank of a generic (undeclared) column: identifiers first, then status,
/// type, and sender/recipient, then other fields; references to other resources (`accountId`), links,
/// and columns empty in every row go last.
pub(crate) fn rank_generic_column(key: &str, values: &[&str]) -> usize {
    let leaf = key.rsplit('.').next().unwrap_or(key).to_ascii_lowercase();
    let non_empty: Vec<&str> = values.iter().copied().filter(|v| !v.is_empty()).collect();
    if non_empty.is_empty() {
        return 5;
    }
    let is_link = key.starts_with("links.")
        || key.starts_with("_links.")
        || matches!(leaf.as_str(), "url" | "uri" | "href" | "self")
        || non_empty
            .iter()
            .all(|v| v.starts_with("https://") || v.starts_with("http://"));
    if is_link {
        return 4;
    }
    if is_reference(key.rsplit('.').next().unwrap_or(key)) {
        return 3;
    }
    match leaf.as_str() {
        "id" | "sid" | "uuid" | "name" | "title" | "slug" => 0,
        "status" | "state" | "type" | "from" | "to" => 1,
        _ => 2,
    }
}

/// `accountId`, `account_sid`, `parent-uuid`: a field naming another resource.
fn is_reference(leaf: &str) -> bool {
    let lower = leaf.to_ascii_lowercase();
    ["_id", "-id", "_sid", "-sid", "_uuid", "-uuid"]
        .iter()
        .any(|suffix| lower.ends_with(suffix))
        || ["Id", "ID", "Sid", "SID", "Uuid", "UUID"]
            .iter()
            .any(|suffix| {
                leaf.strip_suffix(suffix)
                    .and_then(|rest| rest.chars().last())
                    .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
            })
}

/// A cell's text on one line: line breaks become `↵` and tabs a space, so a
/// multi-line value doesn't split its row.
pub(crate) fn one_line(text: &str) -> String {
    if !text.contains(['\n', '\r', '\t']) {
        return text.to_string();
    }
    text.replace("\r\n", "↵")
        .replace(['\n', '\r'], "↵")
        .replace('\t', " ")
}

/// Shorten `text` to `width` characters, ending in `…`.
pub(crate) fn truncate(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_string();
    }
    let kept: String = text.chars().take(width.saturating_sub(1)).collect();
    format!("{kept}…")
}

/// The stderr note for hidden columns, e.g. "Showing 7 of 19 columns to fit
/// the terminal. Use --format json to see every field."
pub(crate) fn hidden_note(shown: usize, total: usize, hint: &str) -> String {
    format!("Showing {shown} of {total} columns. {hint}")
}

/// Print `note` on stderr, dimmed when stderr is a color terminal.
pub(crate) fn print_note(note: &str) {
    eprintln!("{}", crate::output::colorize(note, "2"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn col(natural: usize, min: usize, rank: usize) -> ColumnFit {
        ColumnFit { natural, min, rank }
    }

    #[test]
    fn a_table_that_fits_is_unchanged() {
        let cols = [col(10, 10, 2), col(30, 20, 0)];
        assert_eq!(fit(&cols, 42), vec![Some(10), Some(30)]);
    }

    #[test]
    fn low_ranked_columns_are_hidden_and_spare_space_goes_to_the_best() {
        // 34 + 2 + 20 + 2 + 20 = 78 at minimum; the rank-3 column can't fit.
        let cols = [
            col(34, 34, 0),
            col(60, 20, 2),
            col(30, 20, 2),
            col(34, 20, 3),
        ];
        assert_eq!(fit(&cols, 90), vec![Some(34), Some(32), Some(20), None]);
    }

    #[test]
    fn a_lone_column_wider_than_the_terminal_is_cut_to_it() {
        assert_eq!(
            fit(&[col(60, 60, 0), col(5, 5, 1)], 40),
            vec![Some(40), None]
        );
    }

    #[test]
    fn ranks_put_identifiers_first_and_links_and_constants_last() {
        assert_eq!(rank_generic_column("id", &["SM1", "SM2"]), 0);
        assert_eq!(rank_generic_column("status", &["sent", "failed"]), 1);
        assert_eq!(rank_generic_column("body", &["a", "b"]), 2);
        assert_eq!(rank_generic_column("accountId", &["AC1", "AC2"]), 3);
        assert_eq!(rank_generic_column("account_sid", &["AC1"]), 3);
        assert_eq!(rank_generic_column("from", &["+1500", "+1500"]), 1);
        assert_eq!(rank_generic_column("paid", &["true"]), 2);
        assert_eq!(rank_generic_column("parent_uuid", &["u1"]), 3);
        assert_eq!(rank_generic_column("links.media", &["/x", "/y"]), 4);
        assert_eq!(
            rank_generic_column("homepage", &["https://a", "https://b"]),
            4
        );
        assert_eq!(rank_generic_column("errorCode", &["", ""]), 5);
    }

    #[test]
    fn one_line_marks_line_breaks() {
        assert_eq!(one_line("hours:\nMon\r\nTue\tx"), "hours:↵Mon↵Tue x");
        assert_eq!(one_line("plain"), "plain");
    }

    #[test]
    fn header_is_bold_only_when_asked() {
        let plain = TableStyle::default();
        assert_eq!(plain.header("ID"), "ID");
        let bold = TableStyle {
            width: None,
            bold: true,
        };
        assert_eq!(bold.header("ID"), "\x1b[1mID\x1b[0m");
    }
}
