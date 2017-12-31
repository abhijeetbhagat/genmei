use std::cmp::Ordering;
#[derive(Debug)]
pub struct Version {
    major: i32,
    minor: i32,
}

impl Version {
    pub fn new(major: i32, minor: i32) -> Self {
        Version {
            major,
            minor,
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.major == other.major && self.minor == other.minor
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self < other {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }

    fn lt(&self, other: &Version) -> bool {
        if self.major < other.major {
            return true;
        }
        if self.major == other.major && self.minor < other.minor {
            return true;
        }
        false
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}
