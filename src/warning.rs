use annotate_snippets::display_list::{DisplayList, FormatOptions};
use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

/// An warning formatter.
pub struct Warning<'a> {
    snippet: Snippet<'a>,
}

impl<'a> Warning<'a> {
    /// Create a new `Warning` formatter.
    pub fn new(label: &'a str) -> Self {
        Self {
            snippet: Snippet {
                title: Some(Annotation {
                    label: Some(label),
                    id: None,
                    annotation_type: AnnotationType::Warning,
                }),
                slices: vec![],
                footer: vec![],
                opt: FormatOptions {
                    color: true,
                    ..Default::default()
                },
            },
        }
    }

    /// Pass a new warning to the formatter.
    pub fn warning(
        mut self,
        line_start: usize,
        start: usize,
        end: usize,
        source: &'a str,
        label: &'a str,
    ) -> Self {
        self.snippet.slices.push(Slice {
            source: source,
            line_start,
            origin: None,
            fold: false,
            annotations: vec![SourceAnnotation {
                label: label,
                annotation_type: AnnotationType::Warning,
                range: (start, end),
            }],
        });
        self
    }

    /// Create a new footer.
    pub fn help(mut self, label: &'a str) -> Self {
        self.snippet.footer.push(Annotation {
            label: Some(label),
            id: None,
            annotation_type: AnnotationType::Help,
        });
        self
    }

    pub fn to_string(self) -> String {
        DisplayList::from(self.snippet).to_string()
    }
}
