use crate::{
    diagnostic::{Diagnostic, Severity, Location},
    type_,
    type_::pretty::Printer,
};
use std::io::Write;
use std::path::PathBuf;
use termcolor::Buffer;

pub type Src = String;

#[derive(Debug, PartialEq)]
pub enum Warning {
    Type {
        path: PathBuf,
        src: Src,
        warning: crate::type_::Warning,
    },
}

impl Warning {
    pub fn to_diagnostic(&self) -> (Diagnostic, String) {
        #[allow(clippy::unwrap_used)]
        match self {
            Self::Type { path, warning, .. } => match warning {
                type_::Warning::Todo { location, typ } => {
                    let mut printer = Printer::new();
                    (
                        Diagnostic {
                            title: "Todo found".to_string(),
                            text: "Todo found".to_string(),
                            level: crate::diagnostic::Level::Warning,
                            location: Some(Location {
                                src: "todo".to_string(),
                                path: path.to_path_buf(),
                                label: crate::diagnostic::Label { text: None, span: *location},
                                extra_labels: Vec::new(),
                            }),
                        },
                        format!(
                            "Hint: I think its type is `{}`.

This code will crash if it is run. Be sure to remove this todo before running
your program.",
                            printer.pretty_print(typ, 0)
                        ),
                    )
                }

                _ => (
                    Diagnostic { 
                        title: "Other warning".to_string(),
                        text: "Other warning".to_string(),
                        level: crate::diagnostic::Level::Warning,
                        location: None,
                    },
                    "This is a hint".to_string()
                )

                // TODO: Check all this matches. OldDiagnostic does not exists

                // type_::Warning::ImplicitlyDiscardedResult { location } => (
                //     OldDiagnostic {
                //         title: "Unused result value".to_string(),
                //         label: "The Result value created here is unused".to_string(),
                //         file: path.to_str().unwrap().to_string(),
                //         src: src.to_string(),
                //         location: *location,
                //     },
                //     "Hint: If you are sure you don't need it you can assign it to `_`".to_string(),
                // ),

                // type_::Warning::UnusedLiteral { location } => (
                //     OldDiagnostic {
                //         title: "Unused literal".to_string(),
                //         label: "This value is never used".to_string(),
                //         file: path.to_str().unwrap().to_string(),
                //         src: src.to_string(),
                //         location: *location,
                //     },
                //     "Hint: You can safely remove it.".to_string(),
                // ),

                // type_::Warning::NoFieldsRecordUpdate { location } => (
                //     OldDiagnostic {
                //         title: "Fieldless record update".to_string(),
                //         label: "This record update doesn't change any fields.".to_string(),
                //         file: path.to_str().unwrap().to_string(),
                //         src: src.to_string(),
                //         location: *location,
                //     },
                //     "Hint: Add some fields to change or replace it with the record itself. "
                //         .to_string(),
                // ),

                // type_::Warning::AllFieldsRecordUpdate { location } => (
                //     OldDiagnostic {
                //         title: "Redundant record update".to_string(),
                //         label: "This record update specifies all fields".to_string(),
                //         file: path.to_str().unwrap().to_string(),
                //         src: src.to_string(),
                //         location: *location,
                //     },
                //     "Hint: It is better style to use the record creation syntax.".to_string(),
                // ),

                // type_::Warning::UnusedType {
                //     location, imported, ..
                // } => {
                //     let title = if *imported {
                //         "Unused imported type".to_string()
                //     } else {
                //         "Unused private type".to_string()
                //     };
                //     let label = if *imported {
                //         "This imported type is never used.".to_string()
                //     } else {
                //         "This private type is never used.".to_string()
                //     };

                //     (
                //         OldDiagnostic {
                //             title,
                //             label,
                //             file: path.to_str().unwrap().to_string(),
                //             src: src.to_string(),
                //             location: *location,
                //         },
                //         "Hint: You can safely remove it.".to_string(),
                //     )
                // }

                // type_::Warning::UnusedConstructor {
                //     location, imported, ..
                // } => {
                //     let title = if *imported {
                //         "Unused imported item".to_string()
                //     } else {
                //         "Unused private type constructor".to_string()
                //     };
                //     let label = if *imported {
                //         "This imported type constructor is never used.".to_string()
                //     } else {
                //         "This private type constructor is never used.".to_string()
                //     };

                //     (
                //         OldDiagnostic {
                //             title,
                //             label,
                //             file: path.to_str().unwrap().to_string(),
                //             src: src.to_string(),
                //             location: *location,
                //         },
                //         "Hint: You can safely remove it.".to_string(),
                //     )
                // }

                // type_::Warning::UnusedImportedValue { location, .. } => (
                //     OldDiagnostic {
                //         title: "Unused imported value".to_string(),
                //         label: "This imported value is never used.".to_string(),
                //         file: path.to_str().unwrap().to_string(),
                //         src: src.to_string(),
                //         location: *location,
                //     },
                //     "Hint: You can safely remove it.".to_string(),
                // ),

                // type_::Warning::UnusedPrivateModuleConstant { location, .. } => (
                //     OldDiagnostic {
                //         title: "Unused private constant".to_string(),
                //         label: "This private constant is never used.".to_string(),
                //         file: path.to_str().unwrap().to_string(),
                //         src: src.to_string(),
                //         location: *location,
                //     },
                //     "Hint: You can safely remove it.".to_string(),
                // ),

                // type_::Warning::UnusedPrivateFunction { location, .. } => (
                //     OldDiagnostic {
                //         title: "Unused private function".to_string(),
                //         label: "This private function is never used.".to_string(),
                //         file: path.to_str().unwrap().to_string(),
                //         src: src.to_string(),
                //         location: *location,
                //     },
                //     "Hint: You can safely remove it.".to_string(),
                // ),

                // type_::Warning::UnusedVariable { location, name, .. } => (
                //     OldDiagnostic {
                //         title: "Unused variable".to_string(),
                //         label: "This variable is never used.".to_string(),
                //         file: path.to_str().unwrap().to_string(),
                //         src: src.to_string(),
                //         location: *location,
                //     },
                //     format!("Hint: you can ignore it with an underscore: `_{}`.", name),
                // ),
            },
        }
    }

    pub fn pretty(&self, buffer: &mut Buffer) {
        #[allow(clippy::expect_used)]
        buffer
            .write_all(b"\n")
            .expect("error pretty buffer write space before");
        let (_diagnostic, extra) = self.to_diagnostic();
        // write_old(buffer, diagnostic, Severity::Warning);
        if !extra.is_empty() {
            writeln!(buffer, "{}", extra).unwrap();
        }
    }
}
