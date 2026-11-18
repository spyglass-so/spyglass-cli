//! `x-fern-cli-table`: per-operation columns for human `table` output.
//!
//! An operation can declare which fields its table shows, with labels,
//! order, alignment, width, and light value formatting:
//!
//! ```yaml
//! x-fern-cli-table:
//!   columns:
//!     - { header: NAME, path: name }
//!     - { header: ROWS, path: stats.rows, align: right }
//!     - { header: CREATED, path: createdAt, format: date }
//!     - { header: DESCRIPTION, path: description, maxWidth: 40, default: false }
//! ```
//!
//! Only `table` output changes. `--query` and every other format see the
//! full response. Paths are relative to each row, after
//! `x-fern-sdk-return-value` extraction.

use serde::Deserialize;
use serde_json::Value;

/// Long name of the field picker on operations without `x-fern-cli-table`.
pub const PICKER_FLAG: &str = "properties";

/// The spec an operation's picker works against: its declared
/// `x-fern-cli-table` (the `--columns` picker), or, for any other operation
/// that returns JSON, a field spec of its `x-twilio.defaultOutputProperties`
/// (every field when it has none) for `--properties`.
pub(crate) fn effective_spec(
    method: &crate::openapi::discovery::RestMethod,
) -> Option<std::borrow::Cow<'_, TableSpec>> {
    if let Some(spec) = method.table.as_ref() {
        return Some(std::borrow::Cow::Borrowed(spec));
    }
    let returns_json =
        method.response.is_some() && !method.has_binary_response && method.streaming.is_none();
    returns_json.then(|| {
        let mut spec =
            TableSpec::from_fields(method.default_properties.as_deref().unwrap_or_default());
        spec.known_fields = method.response_fields.clone();
        std::borrow::Cow::Owned(spec)
    })
}
use std::fmt::Write;

use crate::table_layout::{one_line, truncate, ColumnFit, TableStyle};

/// Widest a column grows when the spec sets no `maxWidth`. Matches the
/// generic table formatter.
const DEFAULT_MAX_WIDTH: usize = 60;

/// Column widths of a streamed `--page-all` table, set by the first page
/// with rows and reused for later pages so their columns line up under the
/// one header. Longer values on later pages are cut off with `…`. `None`
/// marks a column hidden to fit the terminal.
#[derive(Debug, Clone, Default)]
pub struct PageWidths(std::sync::Arc<std::sync::Mutex<PageLayouts>>);

#[derive(Debug, Default)]
struct PageLayouts {
    declared: Option<Vec<Option<usize>>>,
    generic: Option<crate::formatter::GenericLayout>,
}

impl PageWidths {
    pub(crate) fn with<R>(&self, f: impl FnOnce(&mut Option<Vec<Option<usize>>>) -> R) -> R {
        let mut slot = self.0.lock().unwrap_or_else(|e| e.into_inner());
        f(&mut slot.declared)
    }

    /// The same, for a streamed generic table (no `x-fern-cli-table`).
    pub(crate) fn with_generic<R>(
        &self,
        f: impl FnOnce(&mut Option<crate::formatter::GenericLayout>) -> R,
    ) -> R {
        let mut slot = self.0.lock().unwrap_or_else(|e| e.into_inner());
        f(&mut slot.generic)
    }
}

/// Parsed `x-fern-cli-table` extension.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableSpec {
    pub columns: Vec<TableColumn>,
    /// Built for `--properties` from `x-twilio.defaultOutputProperties` (or
    /// nothing) rather than declared with `x-fern-cli-table`.
    #[serde(skip)]
    pub fields_mode: bool,
    /// Field names the response can have (`--properties` only). A name
    /// outside this list is an error rather than an empty column.
    #[serde(skip)]
    pub known_fields: Option<KnownFields>,
}

/// Field names a response's records can have, from its schema.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KnownFields {
    /// The record fields, listed in the unknown-field error.
    pub fields: Vec<String>,
    /// Also accepted: a list envelope's own keys (`meta`), which a single
    /// record of the same shape would have.
    pub envelope: Vec<String>,
}

/// One column of a [`TableSpec`].
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TableColumn {
    /// Column label printed in the header row.
    pub header: String,
    /// Dot-separated field path inside each row.
    pub path: String,
    #[serde(default)]
    pub align: Align,
    /// Longer values are cut to this many characters, ending in `…`.
    #[serde(default)]
    pub max_width: Option<usize>,
    #[serde(default)]
    pub format: Option<CellFormat>,
    /// `false` hides the column unless the user adds it with `--columns`.
    #[serde(default = "default_true")]
    pub default: bool,
    /// Match field names across case and `_`/`-` (`--properties` only).
    #[serde(skip)]
    pub loose: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Align {
    #[default]
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CellFormat {
    /// Shorten an ISO-8601 timestamp to its `YYYY-MM-DD` date.
    Date,
}

impl TableSpec {
    /// Parse the raw extension value. Returns a description of the problem
    /// when the value is malformed so the caller can warn and ignore it.
    pub fn from_yaml(raw: &serde_yaml::Value) -> Result<Self, String> {
        let spec: TableSpec = serde_yaml::from_value(raw.clone()).map_err(|e| e.to_string())?;
        if spec.columns.is_empty() {
            return Err("`columns` must list at least one column".to_string());
        }
        for column in &spec.columns {
            if column.header.trim().is_empty() {
                return Err("every column needs a non-empty `header`".to_string());
            }
            if column.path.trim().is_empty() {
                return Err(format!(
                    "column `{}` needs a non-empty `path`",
                    column.header
                ));
            }
            if column.max_width == Some(0) {
                return Err(format!(
                    "column `{}`: `maxWidth` must be at least 1",
                    column.header
                ));
            }
        }
        if !spec.columns.iter().any(|c| c.default) {
            return Err("at least one column must be shown by default".to_string());
        }
        Ok(spec)
    }

    /// A field-picker spec whose default columns are `fields`.
    pub fn from_fields(fields: &[String]) -> Self {
        let columns = fields
            .iter()
            .map(|f| TableColumn {
                header: f.clone(),
                path: f.clone(),
                align: Align::Left,
                max_width: None,
                format: None,
                default: true,
                loose: true,
            })
            .collect();
        Self {
            columns,
            fields_mode: true,
            known_fields: None,
        }
    }

    /// Columns shown when `--columns` is not passed.
    pub fn default_columns(&self) -> Vec<TableColumn> {
        self.columns.iter().filter(|c| c.default).cloned().collect()
    }

    /// Resolve the columns to render for a `--columns` value.
    ///
    /// - `None`: the default columns.
    /// - `all`: `Ok(None)`, meaning the generic table with every field.
    /// - `a,b,c`: exactly those columns, in that order.
    /// - `+a,-b`: the default columns with `a` added and `b` removed.
    ///
    /// Names match a column's `header` or `path` (case-insensitive). An
    /// unknown name in a list or after `+` is treated as a field path, so
    /// users can show fields the spec does not declare.
    pub fn select(&self, selection: Option<&str>) -> Result<Option<Vec<TableColumn>>, String> {
        let flag = if self.fields_mode {
            "--properties"
        } else {
            "--columns"
        };
        let Some(selection) = selection else {
            // No declared columns: the generic table with every field.
            if self.columns.is_empty() {
                return Ok(None);
            }
            return Ok(Some(self.default_columns()));
        };
        let tokens: Vec<&str> = selection
            .split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .collect();
        if tokens.is_empty() {
            return Err(format!("{flag} needs at least one column name"));
        }
        if tokens.iter().any(|t| t.eq_ignore_ascii_case("all")) {
            if tokens.len() > 1 {
                return Err(format!("{flag} all can't be combined with other columns"));
            }
            return Ok(None);
        }
        let modifiers = tokens
            .iter()
            .filter(|t| t.starts_with('+') || t.starts_with('-'))
            .count();
        if modifiers != 0 && modifiers != tokens.len() {
            return Err(format!(
                "{flag} takes either a list of columns (name,stage) or changes to the \
                 default columns (+description,-sync), not both"
            ));
        }
        if modifiers != 0 && self.columns.is_empty() {
            return Err(format!(
                "{flag} takes a comma-separated list of fields, e.g. {flag} id,name"
            ));
        }

        if modifiers == 0 {
            let mut columns: Vec<TableColumn> = Vec::new();
            for token in &tokens {
                self.check_known(token)?;
                let column = self.column_for(token);
                if !columns.contains(&column) {
                    columns.push(column);
                }
            }
            return Ok(Some(columns));
        }

        let mut columns = self.default_columns();
        for token in tokens {
            let name = token[1..].trim();
            if name.is_empty() {
                return Err(format!("{flag}: `{token}` is missing a column name"));
            }
            if token.starts_with('+') {
                self.check_known(name)?;
                let column = self.column_for(name);
                if !columns.contains(&column) {
                    columns.push(column);
                }
            } else {
                let before = columns.len();
                columns.retain(|c| !matches_name(c, name));
                if columns.len() == before {
                    return Err(format!(
                        "{flag}: `{name}` is not one of the shown columns ({})",
                        self.default_columns()
                            .iter()
                            .map(|c| c.header.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
        }
        // Keep declared order however the modifiers were written; ad hoc
        // field paths go last.
        columns.sort_by_key(|c| {
            self.columns
                .iter()
                .position(|d| d == c)
                .unwrap_or(usize::MAX)
        });
        if columns.is_empty() {
            return Err(format!("{flag} removed every column"));
        }
        Ok(Some(columns))
    }

    /// Reject a `--properties` name the response schema doesn't have. A dot
    /// path is checked by its first segment.
    fn check_known(&self, name: &str) -> Result<(), String> {
        let Some(known) = self
            .known_fields
            .as_ref()
            .filter(|k| self.fields_mode && !k.fields.is_empty())
        else {
            return Ok(());
        };
        let head = name.split('.').next().unwrap_or(name);
        let wanted = [normalize_key(name), normalize_key(head)];
        if known
            .fields
            .iter()
            .chain(&known.envelope)
            .any(|k| wanted.contains(&normalize_key(k)))
        {
            return Ok(());
        }
        Err(format!(
            "--{PICKER_FLAG}: unknown field `{name}`. Fields: {}",
            known.fields.join(", ")
        ))
    }

    fn column_for(&self, name: &str) -> TableColumn {
        self.columns
            .iter()
            .find(|c| matches_name(c, name))
            .cloned()
            .unwrap_or_else(|| TableColumn {
                header: name.to_string(),
                path: name.to_string(),
                align: Align::Left,
                max_width: None,
                format: None,
                default: true,
                loose: self.fields_mode,
            })
    }

    /// `--columns` help text listing the declared columns.
    pub fn columns_help(&self) -> String {
        let names = |shown: bool| {
            self.columns
                .iter()
                .filter(|c| c.default == shown)
                .map(|c| c.header.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        };
        if self.fields_mode {
            let defaults = if self.columns.is_empty() {
                "every field".to_string()
            } else {
                names(true)
            };
            return format!("Fields to show, comma-separated, or `all`. Default: {defaults}");
        }
        let mut help = format!(
            "Columns for table output, comma-separated. Use +COL / -COL to change the \
             default set, a field path for an undeclared field, or `all` for every field. \
             Default: {}",
            names(true)
        );
        let hidden = names(false);
        if !hidden.is_empty() {
            let _ = write!(help, ". Also available: {hidden}");
        }
        help
    }
}

fn matches_name(column: &TableColumn, name: &str) -> bool {
    column.header.eq_ignore_ascii_case(name) || column.path.eq_ignore_ascii_case(name)
}

/// Look up a dot-separated `path` in `row`. A key that itself contains dots
/// is matched before the path is split.
fn get<'a>(row: &'a Value, column: &TableColumn) -> Option<&'a Value> {
    lookup(row, &column.path, column.loose)
}

/// `loose` also matches keys that differ only in case or `_`/`-`.
fn lookup<'a>(row: &'a Value, path: &str, loose: bool) -> Option<&'a Value> {
    if let Some(v) = row.get(path) {
        return Some(v);
    }
    let mut current = row;
    for segment in path.split('.') {
        current = if loose {
            field(current, segment)?
        } else {
            current.get(segment)?
        };
    }
    Some(current)
}

/// `value[name]`, falling back to a key that differs only in case or
/// `_`/`-` separators so `date_sent` finds `dateSent`.
fn field<'a>(value: &'a Value, name: &str) -> Option<&'a Value> {
    if let Some(v) = value.get(name) {
        return Some(v);
    }
    let wanted = normalize_key(name);
    value
        .as_object()?
        .iter()
        .find(|(k, _)| normalize_key(k) == wanted)
        .map(|(_, v)| v)
}

/// Lowercased with `_`/`-` removed, so `date_sent` and `dateSent` compare equal.
fn normalize_key(s: &str) -> String {
    s.chars()
        .filter(|c| *c != '_' && *c != '-')
        .flat_map(char::to_lowercase)
        .collect()
}

fn cell(row: &Value, column: &TableColumn) -> String {
    let Some(value) = get(row, column) else {
        return String::new();
    };
    let text = one_line(&crate::formatter::value_to_cell(value));
    match column.format {
        Some(CellFormat::Date) => shorten_to_date(&text).unwrap_or(text),
        None => text,
    }
}

fn shorten_to_date(text: &str) -> Option<String> {
    let date = text.get(..10)?;
    let bytes = date.as_bytes();
    let is_date = bytes.iter().enumerate().all(|(i, b)| match i {
        4 | 7 => *b == b'-',
        _ => b.is_ascii_digit(),
    });
    is_date.then(|| date.to_string())
}

fn pad(text: &str, width: usize, align: Align) -> String {
    let fill = " ".repeat(width.saturating_sub(text.chars().count()));
    match align {
        Align::Left => format!("{text}{fill}"),
        Align::Right => format!("{fill}{text}"),
    }
}

/// Render one response (or one page of a paginated response) using
/// `columns`. Returns `None` when the value has no rows or record these
/// columns describe, so the caller falls back to the generic table.
///
/// `page_widths` is set for streamed pages: the first page with rows prints
/// the header and fixes the widths that later pages reuse.
pub(crate) fn render(
    value: &Value,
    columns: &[TableColumn],
    emit_header: bool,
    page_widths: Option<&mut Option<Vec<Option<usize>>>>,
) -> Option<String> {
    render_styled(
        value,
        columns,
        emit_header,
        page_widths,
        TableStyle::detect(),
    )
}

fn render_styled(
    value: &Value,
    columns: &[TableColumn],
    emit_header: bool,
    page_widths: Option<&mut Option<Vec<Option<usize>>>>,
    style: TableStyle,
) -> Option<String> {
    match value {
        Value::Array(items) => {
            let rows = page_rows(items, columns).unwrap_or_else(|| items.iter().collect());
            Some(render_rows(&rows, columns, emit_header, page_widths, style))
        }
        Value::Object(obj) => {
            // A record with its own array fields (e.g. `tags`) is still a
            // record, not a list envelope.
            if columns.iter().any(|c| get(value, c).is_some()) {
                return Some(render_record(value, columns));
            }
            if let Some((_, rows)) = crate::formatter::extract_items(value) {
                let rows: Vec<&Value> = rows.iter().collect();
                return Some(render_rows(&rows, columns, emit_header, page_widths, style));
            }
            // A list envelope whose page came back empty.
            if obj
                .values()
                .any(|v| matches!(v, Value::Array(a) if a.is_empty()))
            {
                return Some(render_rows(&[], columns, emit_header, page_widths, style));
            }
            None
        }
        _ => None,
    }
}

/// Rows of a captured `--page-all` result, which is one item per page: each
/// page is either a row array (after `x-fern-sdk-return-value`) or a list
/// envelope. `None` when `items` are already rows.
fn page_rows<'a>(items: &'a [Value], columns: &[TableColumn]) -> Option<Vec<&'a Value>> {
    if items.is_empty() {
        return None;
    }
    let mut rows = Vec::new();
    let mut parsed_pages = 0;
    for item in items {
        match item {
            Value::Array(page) => {
                parsed_pages += 1;
                rows.extend(page);
            }
            Value::Object(_) if !columns.iter().any(|c| get(item, c).is_some()) => {
                // A page without a row list (e.g. `{"next": null}`) adds no rows.
                if let Some((_, page)) = crate::formatter::extract_items(item) {
                    parsed_pages += 1;
                    rows.extend(page);
                }
            }
            _ => return None,
        }
    }
    (parsed_pages > 0).then_some(rows)
}

/// Keep only `columns` of each record in `value`, for non-table formats.
/// A list (an array, a list envelope, or `--page-all` pages) becomes an
/// array of trimmed records; a single record becomes one trimmed object.
/// Fields are keyed by the name the user asked for; missing ones are null.
pub(crate) fn project(value: &Value, columns: &[TableColumn]) -> Value {
    let rows: Option<Vec<&Value>> = match value {
        Value::Array(items) => {
            Some(page_rows(items, columns).unwrap_or_else(|| items.iter().collect()))
        }
        Value::Object(obj) if !columns.iter().any(|c| get(value, c).is_some()) => {
            match crate::formatter::extract_items(value) {
                Some((_, rows)) => Some(rows.iter().collect()),
                None if obj
                    .values()
                    .any(|v| matches!(v, Value::Array(a) if a.is_empty())) =>
                {
                    Some(Vec::new())
                }
                None => None,
            }
        }
        Value::Object(_) => None,
        _ => return value.clone(),
    };
    let record = |row: &Value| {
        Value::Object(
            columns
                .iter()
                .map(|c| (c.path.clone(), get(row, c).cloned().unwrap_or(Value::Null)))
                .collect(),
        )
    };
    match rows {
        Some(rows) => Value::Array(rows.into_iter().map(record).collect()),
        None => record(value),
    }
}

fn render_rows(
    rows: &[&Value],
    columns: &[TableColumn],
    emit_header: bool,
    page_widths: Option<&mut Option<Vec<Option<usize>>>>,
    style: TableStyle,
) -> String {
    if rows.is_empty() {
        // An empty page partway through a streamed table adds nothing.
        if matches!(page_widths, Some(Some(_))) {
            return String::new();
        }
        return "(empty)\n".to_string();
    }
    let cells: Vec<Vec<String>> = rows
        .iter()
        .map(|row| columns.iter().map(|c| cell(row, c)).collect())
        .collect();
    // Later streamed pages keep the first page's widths. A longer value
    // pushes the rest of its row right rather than being cut off.
    let (widths, emit_header, carried) = match page_widths {
        Some(Some(widths)) => (widths.clone(), false, true),
        Some(slot) => {
            let widths = fit_to_terminal(measure(&cells, columns), columns, style);
            *slot = Some(widths.clone());
            (widths, true, false)
        }
        None => (
            fit_to_terminal(measure(&cells, columns), columns, style),
            emit_header,
            false,
        ),
    };
    // On a terminal the carried widths are a fit to it: keep to them.
    let carried = carried && style.width.is_none();

    let shown: Vec<(usize, &TableColumn, usize)> = columns
        .iter()
        .zip(&widths)
        .enumerate()
        .filter_map(|(i, (c, w))| w.map(|w| (i, c, w)))
        .collect();
    let mut output = String::new();
    if emit_header {
        let header: Vec<String> = shown
            .iter()
            .map(|(_, c, w)| pad(&truncate(&c.header, *w), *w, c.align))
            .collect();
        let _ = writeln!(output, "{}", style.header(header.join("  ").trim_end()));
        let rule: Vec<String> = shown.iter().map(|(_, _, w)| "─".repeat(*w)).collect();
        let _ = writeln!(output, "{}", rule.join("  "));
        if shown.len() < columns.len() {
            crate::table_layout::print_note(&crate::table_layout::hidden_note(
                shown.len(),
                columns.len(),
                "Use --columns to choose them, or --format json to see every field.",
            ));
        }
    }
    for row in &cells {
        let parts: Vec<String> = shown
            .iter()
            .map(|(i, c, w)| {
                let limit = if carried {
                    c.max_width.unwrap_or(DEFAULT_MAX_WIDTH)
                } else {
                    *w
                };
                pad(&truncate(&row[*i], limit), *w, c.align)
            })
            .collect();
        let _ = writeln!(output, "{}", parts.join("  ").trim_end());
    }
    output
}

/// Fit measured widths to the terminal. Declared columns keep their order
/// as priority, so the last ones are hidden first.
fn fit_to_terminal(
    widths: Vec<usize>,
    columns: &[TableColumn],
    style: TableStyle,
) -> Vec<Option<usize>> {
    let Some(available) = style.width else {
        return widths.into_iter().map(Some).collect();
    };
    let fits: Vec<ColumnFit> = widths
        .iter()
        .zip(columns)
        .enumerate()
        .map(|(rank, (w, c))| ColumnFit::shrinkable(*w, c.header.chars().count(), rank))
        .collect();
    crate::table_layout::fit(&fits, available)
}

fn measure(cells: &[Vec<String>], columns: &[TableColumn]) -> Vec<usize> {
    columns
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let widest = cells
                .iter()
                .map(|r| r[i].chars().count())
                .max()
                .unwrap_or(0);
            let cap = c.max_width.unwrap_or(DEFAULT_MAX_WIDTH);
            widest.min(cap).max(c.header.chars().count())
        })
        .collect()
}

fn render_record(record: &Value, columns: &[TableColumn]) -> String {
    let label_width = columns
        .iter()
        .map(|c| c.header.chars().count())
        .max()
        .unwrap_or(0);
    let mut output = String::new();
    for column in columns {
        let text = cell(record, column);
        let text = truncate(&text, column.max_width.unwrap_or(DEFAULT_MAX_WIDTH));
        let line = format!("{}  {text}", pad(&column.header, label_width, Align::Left));
        let _ = writeln!(output, "{}", line.trim_end());
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn spec(yaml: &str) -> TableSpec {
        TableSpec::from_yaml(&serde_yaml::from_str(yaml).unwrap()).unwrap()
    }

    fn datasets_spec() -> TableSpec {
        spec(
            r#"
columns:
  - { header: NAME, path: name }
  - { header: V, path: version, align: right }
  - { header: STAGE, path: stage }
  - { header: ROWS, path: stats.rows, align: right }
  - { header: CREATED, path: createdAt, format: date }
  - { header: DESCRIPTION, path: description, maxWidth: 12, default: false }
"#,
        )
    }

    fn rows() -> Value {
        json!([
            {
                "name": "aa_lcr", "version": 10, "stage": "approved",
                "stats": { "rows": 3870 }, "createdAt": "2026-06-30T12:00:00Z",
                "description": "Long context retrieval set", "datasetId": "79d0"
            },
            {
                "name": "aa_lcr_128k", "version": 1, "stage": "draft",
                "stats": { "rows": 3872 }, "createdAt": "2026-06-17T08:30:00Z",
                "description": "Short", "datasetId": "80e1"
            }
        ])
    }

    fn select(spec: &TableSpec, s: Option<&str>) -> Vec<String> {
        spec.select(s)
            .unwrap()
            .unwrap()
            .into_iter()
            .map(|c| c.header)
            .collect()
    }

    #[test]
    fn renders_declared_columns_in_order_with_alignment_and_dates() {
        let spec = datasets_spec();
        let out = render(&rows(), &spec.default_columns(), true, None).unwrap();
        assert_eq!(
            out,
            "NAME          V  STAGE     ROWS  CREATED\n\
             ───────────  ──  ────────  ────  ──────────\n\
             aa_lcr       10  approved  3870  2026-06-30\n\
             aa_lcr_128k   1  draft     3872  2026-06-17\n"
        );
    }

    #[test]
    fn continuation_pages_skip_the_header() {
        let spec = datasets_spec();
        let out = render(&rows(), &spec.default_columns(), false, None).unwrap();
        assert!(out.starts_with("aa_lcr "), "{out}");
        assert!(!out.contains("NAME"), "{out}");
    }

    #[test]
    fn streamed_pages_reuse_the_first_page_widths() {
        let spec = datasets_spec();
        let cols = spec.select(Some("name,v")).unwrap().unwrap();
        let mut widths = None;
        let first = render(&json!([]), &cols, true, Some(&mut widths)).unwrap();
        assert_eq!(first, "(empty)\n");
        let page1 = render(
            &json!([{ "name": "a", "version": 1 }]),
            &cols,
            false,
            Some(&mut widths),
        )
        .unwrap();
        assert_eq!(page1, "NAME  V\n────  ─\na     1\n");
        let page2 = render(
            &json!([{ "name": "very-long-name", "version": 10 }]),
            &cols,
            false,
            Some(&mut widths),
        )
        .unwrap();
        assert_eq!(page2, "very-long-name  10\n");
        let page3 = render(
            &json!([{ "name": "b", "version": 2 }]),
            &cols,
            false,
            Some(&mut widths),
        )
        .unwrap();
        assert_eq!(page3, "b     2\n");
        let empty = render(&json!([]), &cols, false, Some(&mut widths)).unwrap();
        assert_eq!(empty, "");
    }

    #[test]
    fn rows_are_found_inside_a_list_envelope() {
        let spec = datasets_spec();
        let envelope = json!({ "datasets": rows(), "nextCursor": "abc" });
        let out = render(&envelope, &spec.default_columns(), true, None).unwrap();
        assert!(out.contains("aa_lcr_128k"), "{out}");
    }

    #[test]
    fn captured_page_all_results_are_flattened() {
        let spec = datasets_spec();
        let cols = spec.default_columns();
        let pages = json!([rows(), [rows()[0].clone()]]);
        let out = render(&pages, &cols, true, None).unwrap();
        assert_eq!(
            out.lines().filter(|l| l.starts_with("aa_lcr ")).count(),
            2,
            "{out}"
        );
        let envelopes = json!([{ "datasets": rows(), "next": "p2" }, { "datasets": rows() }]);
        let out = render(&envelopes, &cols, true, None).unwrap();
        assert_eq!(
            out.lines().filter(|l| l.starts_with("aa_lcr_128k")).count(),
            2,
            "{out}"
        );
        let trailing_empty_page = json!([{ "datasets": rows(), "next": "p2" }, { "next": null }]);
        let out = render(&trailing_empty_page, &cols, true, None).unwrap();
        assert_eq!(out.lines().count(), 4, "{out}");
    }

    #[test]
    fn a_record_with_array_fields_renders_as_a_record() {
        let spec = datasets_spec();
        let cols = spec.default_columns();
        let record = json!({ "name": "aa_lcr", "tags": ["a", "b"] });
        let out = render(&record, &cols, true, None).unwrap();
        assert!(out.starts_with("NAME"), "{out}");
        assert!(out.lines().next().unwrap().ends_with("aa_lcr"), "{out}");
    }

    #[test]
    fn empty_pages_render_as_empty() {
        let spec = datasets_spec();
        let cols = spec.default_columns();
        assert_eq!(render(&json!([]), &cols, true, None).unwrap(), "(empty)\n");
        assert_eq!(
            render(&json!({ "datasets": [] }), &cols, true, None).unwrap(),
            "(empty)\n"
        );
    }

    #[test]
    fn single_records_render_as_label_value_pairs() {
        let spec = datasets_spec();
        let out = render(&rows()[0], &spec.default_columns(), true, None).unwrap();
        assert_eq!(
            out,
            "NAME     aa_lcr\nV        10\nSTAGE    approved\nROWS     3870\nCREATED  2026-06-30\n"
        );
    }

    #[test]
    fn unrelated_objects_fall_back_to_the_generic_table() {
        let spec = datasets_spec();
        assert!(render(
            &json!({ "saved_file": "x" }),
            &spec.default_columns(),
            true,
            None
        )
        .is_none());
    }

    #[test]
    fn max_width_truncates_long_values() {
        let spec = datasets_spec();
        let cols = spec.select(Some("name,description")).unwrap().unwrap();
        let out = render(&rows(), &cols, true, None).unwrap();
        assert!(out.contains("Long contex…"), "{out}");
        assert!(
            out.lines()
                .all(|l| l.chars().count() <= "aa_lcr_128k  DESCRIPTION".len() + 1),
            "{out}"
        );
    }

    #[test]
    fn non_date_values_are_left_alone_by_date_format() {
        assert_eq!(
            shorten_to_date("2026-09-08T10:00:00Z").as_deref(),
            Some("2026-09-08")
        );
        assert_eq!(shorten_to_date("yesterday"), None);
        assert_eq!(shorten_to_date("2026/09/08 10:00"), None);
    }

    #[test]
    fn select_defaults_lists_modifiers_and_all() {
        let spec = datasets_spec();
        assert_eq!(
            select(&spec, None),
            ["NAME", "V", "STAGE", "ROWS", "CREATED"]
        );
        assert_eq!(select(&spec, Some("stage, NAME")), ["STAGE", "NAME"]);
        assert_eq!(
            select(&spec, Some("+description,-v,-createdAt")),
            ["NAME", "STAGE", "ROWS", "DESCRIPTION"]
        );
        assert_eq!(select(&spec, Some("name,datasetId")), ["NAME", "datasetId"]);
        assert_eq!(select(&spec, Some("name,NAME,stage")), ["NAME", "STAGE"]);
        assert_eq!(
            select(&spec, Some("-v,+datasetId,+v")),
            ["NAME", "V", "STAGE", "ROWS", "CREATED", "datasetId"]
        );
        assert_eq!(spec.select(Some("all")).unwrap(), None);
    }

    #[test]
    fn empty_field_spec_defaults_to_every_field_and_picks_ad_hoc_fields() {
        let spec = TableSpec::from_fields(&[]);
        assert_eq!(spec.select(None).unwrap(), None);
        assert_eq!(spec.select(Some("all")).unwrap(), None);
        let cols = spec.select(Some("sid, date_sent")).unwrap().unwrap();
        let paths: Vec<&str> = cols.iter().map(|c| c.path.as_str()).collect();
        assert_eq!(paths, ["sid", "date_sent"]);
        assert!(spec.select(Some("+sid")).is_err());
        assert!(spec.select(Some("-sid")).is_err());
    }

    #[test]
    fn field_spec_defaults_to_declared_fields_and_lists_them_in_help() {
        let spec = TableSpec::from_fields(&["sid".to_string(), "date_sent".to_string()]);
        let defaults: Vec<String> = spec.select(None).unwrap().unwrap().into_iter().map(|c| c.path).collect();
        assert_eq!(defaults, ["sid", "date_sent"]);
        assert_eq!(spec.select(Some("all")).unwrap(), None);
        assert_eq!(
            spec.columns_help(),
            "Fields to show, comma-separated, or `all`. Default: sid, date_sent"
        );
        assert_eq!(
            TableSpec::from_fields(&[]).columns_help(),
            "Fields to show, comma-separated, or `all`. Default: every field"
        );
    }

    #[test]
    fn field_spec_rejects_fields_the_response_does_not_have() {
        let mut spec = TableSpec::from_fields(&[]);
        spec.known_fields = Some(KnownFields {
            fields: vec!["sid".into(), "to".into(), "tags".into()],
            envelope: vec!["meta".into()],
        });
        assert!(spec.select(Some("sid,TO,tags.campaign,meta")).is_ok());
        assert_eq!(
            spec.select(Some("dateSent,to")).unwrap_err(),
            "--properties: unknown field `dateSent`. Fields: sid, to, tags"
        );
        assert!(spec.select(Some("all")).is_ok());
        // Without a known field list every name is accepted.
        assert!(TableSpec::from_fields(&[]).select(Some("anything")).is_ok());
        // `x-fern-cli-table` columns still accept undeclared field paths.
        let mut declared = datasets_spec();
        declared.known_fields = Some(KnownFields {
            fields: vec!["name".into()],
            envelope: Vec::new(),
        });
        assert!(declared.select(Some("undeclared")).is_ok());
    }

    #[test]
    fn project_trims_records_lists_and_pages() {
        let cols = TableSpec::from_fields(&[]).select(Some("sid,date_sent")).unwrap().unwrap();
        let record = json!({"sid": "SM1", "dateSent": "d1", "body": "hi", "tags": ["a"]});
        assert_eq!(project(&record, &cols), json!({"sid": "SM1", "date_sent": "d1"}));
        let envelope = json!({"meta": {"page": 0}, "messages": [record.clone(), {"sid": "SM2"}]});
        assert_eq!(
            project(&envelope, &cols),
            json!([{"sid": "SM1", "date_sent": "d1"}, {"sid": "SM2", "date_sent": null}])
        );
        let empty = json!({"meta": {}, "messages": []});
        assert_eq!(project(&empty, &cols), json!([]));
        let pages = json!([[record.clone()], {"messages": [{"sid": "SM3"}]}]);
        assert_eq!(
            project(&pages, &cols),
            json!([{"sid": "SM1", "date_sent": "d1"}, {"sid": "SM3", "date_sent": null}])
        );
        assert_eq!(project(&json!("text"), &cols), json!("text"));
    }

    #[test]
    fn lookup_matches_across_case_and_separators_only_when_loose() {
        let row = json!({"dateSent": "2024-01-01", "error-code": 30003, "sub": {"ApiVersion": "v1"}});
        assert_eq!(lookup(&row, "date_sent", true), Some(&json!("2024-01-01")));
        assert_eq!(lookup(&row, "DATESENT", true), Some(&json!("2024-01-01")));
        assert_eq!(lookup(&row, "error_code", true), Some(&json!(30003)));
        assert_eq!(lookup(&row, "sub.api_version", true), Some(&json!("v1")));
        assert_eq!(lookup(&row, "missing", true), None);
        let exact = json!({"date_sent": 1, "dateSent": 2});
        assert_eq!(lookup(&exact, "dateSent", true), Some(&json!(2)));
        assert_eq!(lookup(&row, "date_sent", false), None);
        assert_eq!(lookup(&row, "dateSent", false), Some(&json!("2024-01-01")));
        // `x-fern-cli-table` columns keep exact matching.
        let declared = TableSpec::from_yaml(
            &serde_yaml::from_str("columns: [{ header: SENT, path: date_sent }]").unwrap(),
        )
        .unwrap();
        assert!(declared.select(None).unwrap().unwrap().iter().all(|c| !c.loose));
        assert!(declared.select(Some("dateSent")).unwrap().unwrap().iter().all(|c| !c.loose));
    }

    #[test]
    fn ad_hoc_fields_render_from_list_envelope() {
        let cols = TableSpec::from_fields(&[]).select(Some("sid,date_sent")).unwrap().unwrap();
        let page = json!({"messages": [{"sid": "SM1", "dateSent": "d1", "body": "hi"}], "meta": {"page": 0}});
        let out = render(&page, &cols, true, None).unwrap();
        assert!(out.contains("sid") && out.contains("date_sent"), "{out}");
        assert!(out.contains("SM1") && out.contains("d1"), "{out}");
        assert!(!out.contains("hi"), "{out}");
    }

    #[test]
    fn select_rejects_bad_input() {
        let spec = datasets_spec();
        for bad in [
            "",
            " , ",
            "all,name",
            "name,+description",
            "-nope",
            "+",
            "-name,-v,-stage,-rows,-created",
        ] {
            assert!(
                spec.select(Some(bad)).is_err(),
                "{bad:?} should be rejected"
            );
        }
    }

    #[test]
    fn from_yaml_rejects_malformed_extensions() {
        for bad in [
            "columns: []",
            "columns: [{ header: A }]",
            "columns: [{ header: '', path: a }]",
            "columns: [{ header: A, path: a, maxWidth: 0 }]",
            "columns: [{ header: A, path: a, align: center }]",
            "columns: [{ header: A, path: a, format: money }]",
            "columns: [{ header: A, path: a, colour: red }]",
            "columns: [{ header: A, path: a, default: false }]",
            "colums: [{ header: A, path: a }]",
        ] {
            let raw: serde_yaml::Value = serde_yaml::from_str(bad).unwrap();
            assert!(
                TableSpec::from_yaml(&raw).is_err(),
                "{bad} should be rejected"
            );
        }
    }

    #[test]
    fn help_lists_default_and_hidden_columns() {
        let help = datasets_spec().columns_help();
        assert!(
            help.contains("Default: NAME, V, STAGE, ROWS, CREATED"),
            "{help}"
        );
        assert!(help.contains("Also available: DESCRIPTION"), "{help}");
    }
}
