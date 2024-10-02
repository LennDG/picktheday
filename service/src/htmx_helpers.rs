use derive_more::derive::Display;

// region:	  --- HtmxId

#[derive(Debug, Display, Clone)]
pub struct HtmxId(String);

impl HtmxId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl From<HtmxTarget> for HtmxId {
    fn from(value: HtmxTarget) -> Self {
        value.0
    }
}

// endregion: --- HtmxId

// region:	  --- HtmxTarget
#[derive(Debug, Clone)]
pub struct HtmxTarget(HtmxId);

impl HtmxTarget {
    pub fn new(htmx_id: HtmxId) -> Self {
        Self(htmx_id)
    }
}

impl std::fmt::Display for HtmxTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.0)
    }
}

impl From<HtmxId> for HtmxTarget {
    fn from(value: HtmxId) -> Self {
        Self::new(value)
    }
}

// endregion: --- HtmxTarget

// region:	  --- HtmxInclude
#[derive(Debug, Clone)]
pub struct HtmxInclude(Vec<HtmxTarget>);

impl HtmxInclude {
    pub fn new(targets: Vec<HtmxTarget>) -> Self {
        Self(targets)
    }
}
impl std::fmt::Display for HtmxInclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let targets = self
            .0
            .iter()
            .map(|target| target.to_string()) // Convert each HtmxTarget to String
            .collect::<Vec<_>>() // Collect into a Vec<String>
            .join(", "); // Join all strings with a space separator
        write!(f, "{}", targets) // Write the final string to the formatter
    }
}

impl From<Vec<HtmxTarget>> for HtmxInclude {
    fn from(value: Vec<HtmxTarget>) -> Self {
        Self::new(value)
    }
}

impl From<HtmxTarget> for HtmxInclude {
    fn from(value: HtmxTarget) -> Self {
        Self::new(vec![value])
    }
}

impl From<Vec<HtmxId>> for HtmxInclude {
    fn from(value: Vec<HtmxId>) -> Self {
        let targets = value
            .iter()
            .map(|id| HtmxTarget::new(id.to_owned()))
            .collect::<Vec<_>>();

        Self::new(targets)
    }
}

impl From<Vec<HtmxInput>> for HtmxInclude {
    fn from(value: Vec<HtmxInput>) -> Self {
        let targets = value
            .iter()
            .map(|id| HtmxTarget::new(id.id.to_owned()))
            .collect::<Vec<_>>();

        Self::new(targets)
    }
}

impl From<HtmxInput> for HtmxInclude {
    fn from(value: HtmxInput) -> Self {
        Self::from(vec![value])
    }
}
// endregion: --- HtmxInclude

// region:	  --- HtmxInput

#[derive(Debug, Clone)]
pub struct HtmxInput {
    pub id: HtmxId,
    pub name: String,
}

impl HtmxInput {
    pub fn new(id: HtmxId, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }
}

// endregion: --- HtmxInput
// region:    --- Tests
#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_htmx_target() -> Result<()> {
        let htmx_id = HtmxId::new("test");
        let htmx_target = HtmxTarget::new(htmx_id.clone());

        assert_eq!(format!("#{}", htmx_id), htmx_target.to_string());

        Ok(())
    }

    #[test]
    fn test_htmx_include() -> Result<()> {
        let htmx_id1 = HtmxId::new("test1");
        let htmx_id2 = HtmxId::new("test2");
        let htmx_id3 = HtmxId::new("test3");
        let htmx_include =
            HtmxInclude::from(vec![htmx_id1.clone(), htmx_id2.clone(), htmx_id3.clone()]);

        assert_eq!(
            format!("#{}, #{}, #{}", htmx_id1, htmx_id2, htmx_id3),
            htmx_include.to_string()
        );

        let htmx_input1 = HtmxInput::new(HtmxId::new("test1"), "test1");
        let htmx_input2 = HtmxInput::new(HtmxId::new("test2"), "test2");
        let htmx_input3 = HtmxInput::new(HtmxId::new("test3"), "test3");
        let htmx_include = HtmxInclude::from(vec![
            htmx_input1.clone(),
            htmx_input2.clone(),
            htmx_input3.clone(),
        ]);

        assert_eq!(
            format!("#{}, #{}, #{}", htmx_id1, htmx_id2, htmx_id3),
            htmx_include.to_string()
        );

        Ok(())
    }
}
// endregion: --- Tests
