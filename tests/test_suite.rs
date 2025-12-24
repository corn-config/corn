use corn::Value as CornValue;
use std::{fs, path::Path};

fn positive(path: &Path, content: String) -> datatest_stable::Result<()> {
    let json: CornValue = {
        let mut path = path.to_path_buf();
        path.set_extension("");
        path.set_extension("");
        path.set_extension("json");

        let path = Path::new("test-suite/json").join(path.strip_prefix("test-suite/corn/")?);

        serde_json::from_str(&fs::read_to_string(path)?)?
    };

    let corn: CornValue = corn::from_str(&content)?;

    assert_eq!(json, corn);

    Ok(())
}

fn negative(_path: &Path, content: String) -> datatest_stable::Result<()> {
    assert!(corn::from_str::<CornValue>(&content).is_err());

    Ok(())
}

datatest_stable::harness! {
    { test = positive, root = "test-suite", pattern = r"^.*\.pos\.corn$" },
    { test = negative, root = "test-suite", pattern = r"^.*\.neg\.corn$" },
}
