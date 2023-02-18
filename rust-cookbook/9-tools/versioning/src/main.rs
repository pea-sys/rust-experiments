
use semver::{BuildMetadata, Prerelease, Version,VersionReq};

fn main() {
    let req = VersionReq::parse(">=1.2.3, <1.8.0").unwrap();

    // Check whether this requirement matches version 1.2.3-alpha.1 (no)
    let version = Version {
        major: 1,
        minor: 2,
        patch: 3,
        pre: Prerelease::new("alpha.1").unwrap(),
        build: BuildMetadata::EMPTY,
    };
    assert!(!req.matches(&version));

    // Check whether it matches 1.3.0 (yes it does)
    let version = Version::parse("1.3.0").unwrap();
    assert!(req.matches(&version));


    let version_str = "1.0.49-125+g72ee7853";
    let parsed_version = Version::parse(version_str).unwrap();

    assert_eq!(
        parsed_version,
        Version {
            major: 1,
            minor: 0,
            patch: 49,
            pre: Prerelease::new("125").unwrap(),
        build: BuildMetadata::EMPTY,
        }
    );


    let serialized_version = parsed_version.to_string();
    assert_eq!(&serialized_version, version_str);


    let version_1 = Version::parse("1.0.0-alpha").unwrap();
    let version_2 = Version::parse("1.0.0").unwrap();

    assert!(version_1.pre==Prerelease::EMPTY);
    assert!(version_2.pre!=Prerelease::EMPTY);

   
}