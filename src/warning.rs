use annotate_snippets::display_list::DisplayList;
use annotate_snippets::formatter::DisplayListFormatter;
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

/// An warning formatter.
pub struct Warning {
    snippet: Snippet,
}

impl Warning {
    /// Create a new `Warning` formatter.
    pub fn new(label: impl ToString) -> Self {
        Self {
            snippet: Snippet {
                title: Some(Annotation {
                    label: Some(label.to_string()),
                    id: None,
                    annotation_type: AnnotationType::Warning,
                }),
                slices: vec![],
                footer: vec![],
            },
        }
    }

    /// Pass a new warning to the formatter.
    pub fn warning(
        mut self,
        line_start: usize,
        start: usize,
        end: usize,
        source: impl ToString,
        label: impl ToString,
    ) -> Self {
        self.snippet.slices.push(Slice {
            source: source.to_string(),
            line_start,
            origin: None,
            fold: false,
            annotations: vec![SourceAnnotation {
                label: label.to_string(),
                annotation_type: AnnotationType::Warning,
                range: (start, end),
            }],
        });
        self
    }

    /// Create a new footer.
    pub fn help(mut self, label: impl ToString) -> Self {
        self.snippet.footer.push(Annotation {
            label: Some(label.to_string()),
            id: None,
            annotation_type: AnnotationType::Help,
        });
        self
    }

    pub fn to_string(self) -> String {
        let dl = DisplayList::from(self.snippet);
        let dlf = DisplayListFormatter::new(true, false);
        format!("{}", dlf.format(&dl))
    }
}
