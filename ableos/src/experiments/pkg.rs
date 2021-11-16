pub type PackageName = String;
use crate::experiments::kinfo::SemanticVersion;

// Scuffed
pub type Hash = u8;
pub struct MetaPackage {
    pub name: u8,
    pub version: SemanticVersion,
    pub authors: [u8; 8],
    pub support_email: u8,
    pub hash: Hash,
}

impl MetaPackage {
    pub fn new() -> Self {
        Self {
            name: 0,
            version: SemanticVersion {
                major: 0,
                minor: 0,
                patch: 0,
            },
            authors: [0; 8],
            support_email: 8,
            hash: 0,
        }
    }

    fn validate_hash(&self) {}
}

impl core::fmt::Display {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Packname: {}
Version: {}
Authors: {:?}
Support Email: {}
Hash: {}",
            self.name, self.version, self.authors, self.support_email, self.hash
        )
    }
}
