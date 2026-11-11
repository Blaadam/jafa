use serde_yaml_ng::{Mapping, Value};

pub struct Settings<'a> {
    pub environment: &'a str,
    pub group_id: Option<u64>,
    pub payments: Option<&'a str>,
    pub place_name: Option<&'a str>,
}

const LABEL_PREFIXED_ENVIRONMENTS: [&str; 2] = ["dev", "stg"];

/// Sets only the fields jafa owns and leaves the rest of a hand-edited mantle.yml alone
pub fn apply(config: &mut Value, place_file: &str, settings: &Settings) {
    let experience = entry(entry(config, "target"), "experience");
    let start = entry(entry(entry(experience, "places"), "start"), "file");
    *start = place_file.into();

    if let Some(group) = settings.group_id {
        let owner = entry(config, "owner");
        *owner = Value::Mapping(Mapping::new());
        *entry(owner, "group") = group.into();
    }

    if let Some(payments) = settings.payments {
        *entry(config, "payments") = payments.into();
    }

    if let Some(name) = settings.place_name {
        let start = start_place(config);
        *entry(entry(start, "configuration"), "name") = name.into();
    }

    set_environment(config, settings.environment);
}

pub fn uses_label_prefix(environment: &str) -> bool {
    LABEL_PREFIXED_ENVIRONMENTS.contains(&environment)
}

pub fn has_owner(config: &Value) -> bool {
    config.get("owner").is_some_and(|owner| !owner.is_null())
}

fn start_place(config: &mut Value) -> &mut Value {
    entry(entry(entry(entry(config, "target"), "experience"), "places"), "start")
}

fn set_environment(config: &mut Value, label: &str) {
    let environments = entry(config, "environments");
    if !environments.is_sequence() {
        *environments = Value::Sequence(Vec::new());
    }
    let Value::Sequence(list) = environments else { return };

    let position = list.iter().position(|env| env.get("label").and_then(Value::as_str) == Some(label));
    let prefixed = uses_label_prefix(label);

    let env = match position {
        Some(index) => &mut list[index],
        None => {
            let mut new = Mapping::new();
            new.insert("label".into(), label.into());
            new.insert("targetAccess".into(), "private".into());
            list.push(Value::Mapping(new));
            list.last_mut().expect("just pushed")
        }
    };

    if prefixed {
        *entry(env, "targetNamePrefix") = "environmentLabel".into();
    }
}

/// `parent[key]`, turning `parent` into a mapping and adding the key first if needed
fn entry<'a>(parent: &'a mut Value, key: &str) -> &'a mut Value {
    if !parent.is_mapping() {
        *parent = Value::Mapping(Mapping::new());
    }
    let Value::Mapping(map) = parent else { unreachable!() };
    map.entry(key.into()).or_insert(Value::Null)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(environment: &str) -> Settings<'_> {
        Settings { environment, group_id: None, payments: None, place_name: None }
    }

    fn parse(text: &str) -> Value {
        serde_yaml_ng::from_str(text).unwrap()
    }

    #[test]
    fn builds_a_config_from_nothing() {
        let mut config = Value::Null;
        apply(&mut config, "../build.rbxl", &settings("dev"));

        assert_eq!(
            config,
            parse(
                "target:\n  experience:\n    places:\n      start:\n        file: ../build.rbxl\n\
                 environments:\n  - label: dev\n    targetAccess: private\n    targetNamePrefix: environmentLabel\n"
            )
        );
    }

    #[test]
    fn only_dev_and_stg_get_the_label_prefix() {
        let mut config = Value::Null;
        apply(&mut config, "../build.rbxl", &settings("prod"));
        assert!(config["environments"][0].get("targetNamePrefix").is_none());
    }

    #[test]
    fn keeps_hand_edited_settings() {
        let mut config = parse(
            "target:\n  experience:\n    configuration:\n      genre: building\n    places:\n      start:\n        file: old.rbxl\n        configuration:\n          description: hi\n\
             environments:\n  - label: dev\n    targetAccess: public\n",
        );
        let with = Settings { group_id: Some(42), payments: Some("owner"), place_name: Some("Arena"), ..settings("dev") };
        apply(&mut config, "../build.rbxl", &with);

        let start = &config["target"]["experience"]["places"]["start"];
        assert_eq!(start["file"], "../build.rbxl");
        assert_eq!(start["configuration"]["description"], "hi");
        assert_eq!(start["configuration"]["name"], "Arena");
        assert_eq!(config["target"]["experience"]["configuration"]["genre"], "building");
        assert_eq!(config["owner"]["group"], 42);
        assert_eq!(config["payments"], "owner");
        assert_eq!(config["environments"][0]["targetAccess"], "public");
        assert_eq!(config["environments"].as_sequence().unwrap().len(), 1);
    }

    #[test]
    fn adds_a_missing_environment_beside_existing_ones() {
        let mut config = parse("environments:\n  - label: prod\n");
        apply(&mut config, "../build.rbxl", &settings("stg"));
        assert_eq!(config["environments"].as_sequence().unwrap().len(), 2);
        assert_eq!(config["environments"][1]["label"], "stg");
    }

    #[test]
    fn knows_when_there_is_no_owner() {
        let mut config = Value::Null;
        apply(&mut config, "../build.rbxl", &settings("dev"));
        assert!(!has_owner(&config));
        assert!(has_owner(&parse("owner: personal")));
    }
}
