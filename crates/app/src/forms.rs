//! Generic forms: a page describes its fields once ([`FieldSpec`]); the
//! values come from a draft and the errors from validation. The result is the
//! `[FormSection]` model rendered by `FormView` in `widgets.slint`.

use crate::ui::{FieldKind, FormField, FormSection};
use cashflow_core::ValidationErrors;
use slint::{ModelRc, SharedString, VecModel};

#[derive(Clone)]
pub struct FieldSpec {
    pub key: &'static str,
    pub label: &'static str,
    pub kind: FieldKind,
    pub required: bool,
    pub wide: bool,
    pub placeholder: &'static str,
    pub suffix: &'static str,
    /// For choice fields: (stored value, visible label).
    pub choices: Vec<(String, String)>,
}

impl FieldSpec {
    fn new(key: &'static str, label: &'static str, kind: FieldKind) -> Self {
        Self { key, label, kind, required: false, wide: false, placeholder: "", suffix: "", choices: Vec::new() }
    }

    pub fn text(key: &'static str, label: &'static str) -> Self {
        Self::new(key, label, FieldKind::Text)
    }

    pub fn date(key: &'static str, label: &'static str) -> Self {
        Self { placeholder: "TT.MM.JJJJ", ..Self::new(key, label, FieldKind::Date) }
    }

    pub fn amount(key: &'static str, label: &'static str) -> Self {
        Self { placeholder: "0,00", suffix: "€", ..Self::new(key, label, FieldKind::Amount) }
    }

    pub fn percent(key: &'static str, label: &'static str) -> Self {
        Self { placeholder: "0", suffix: "%", ..Self::new(key, label, FieldKind::Percent) }
    }

    pub fn number(key: &'static str, label: &'static str, suffix: &'static str) -> Self {
        Self { placeholder: "0", suffix, ..Self::new(key, label, FieldKind::Number) }
    }

    /// Choice field; the first entry should usually be ("", "– keine Angabe –").
    pub fn choice(key: &'static str, label: &'static str, choices: Vec<(String, String)>) -> Self {
        Self { choices, ..Self::new(key, label, FieldKind::Choice) }
    }

    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn wide(mut self) -> Self {
        self.wide = true;
        self
    }

    pub fn placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn suffix(mut self, suffix: &'static str) -> Self {
        self.suffix = suffix;
        self
    }

    fn to_field(&self, value: &str, error: Option<&str>) -> FormField {
        let (choice_values, choice_labels): (Vec<SharedString>, Vec<SharedString>) =
            self.choices.iter().map(|(v, l)| (SharedString::from(v.as_str()), SharedString::from(l.as_str()))).unzip();
        let choice_index = self.choices.iter().position(|(v, _)| v == value).map(|i| i as i32).unwrap_or(-1);
        FormField {
            key: self.key.into(),
            label: self.label.into(),
            value: value.into(),
            error: error.unwrap_or_default().into(),
            kind: self.kind,
            choices: ModelRc::new(VecModel::from(choice_labels)),
            choice_values: ModelRc::new(VecModel::from(choice_values)),
            choice_index,
            placeholder: self.placeholder.into(),
            suffix: self.suffix.into(),
            required: self.required,
            wide: self.wide,
            read_only: false,
        }
    }
}

/// A titled group of fields.
pub struct SectionSpec {
    pub title: &'static str,
    pub description: &'static str,
    pub fields: Vec<FieldSpec>,
}

impl SectionSpec {
    pub fn new(title: &'static str, fields: Vec<FieldSpec>) -> Self {
        Self { title, description: "", fields }
    }

    pub fn described(title: &'static str, description: &'static str, fields: Vec<FieldSpec>) -> Self {
        Self { title, description, fields }
    }
}

/// Builds the form model from the field specs, current values and errors.
pub fn build<'a>(
    sections: &[SectionSpec],
    value: impl Fn(&str) -> &'a str,
    errors: Option<&ValidationErrors>,
) -> ModelRc<FormSection> {
    let sections: Vec<FormSection> = sections
        .iter()
        .map(|section| FormSection {
            title: section.title.into(),
            description: section.description.into(),
            fields: ModelRc::new(VecModel::from(
                section
                    .fields
                    .iter()
                    .map(|spec| spec.to_field(value(spec.key), errors.and_then(|e| e.get(spec.key))))
                    .collect::<Vec<_>>(),
            )),
        })
        .collect();
    ModelRc::new(VecModel::from(sections))
}

/// "– keine Angabe –" as first entry of optional choice fields.
pub fn none_choice() -> (String, String) {
    (String::new(), "– keine Angabe –".into())
}
