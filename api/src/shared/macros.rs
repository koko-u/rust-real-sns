/// Merges multiple garde::Result<T, garde::Reports> into one garde::Report.
macro_rules! merge {
    ($($arg:expr),* $(,)?) => {{
        let mut report = garde::Report::new();
        $(
            if let Err(r) = $arg {
                for (path, error) in r.into_inner() {
                    report.append(path, error);
                }
            }
        )*
        report
    }};
}

pub(crate) use merge;
