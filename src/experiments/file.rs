pub struct Permissions {
    owner: u8,
    read: bool,
    write: bool,
}

pub struct File {
    permissions: Permissions,
    data: Vec<u8>,
}

pub struct Folder {
    permissions: Permissions,
    folders: Vec<Folder>,
    files: Vec<File>,
}
