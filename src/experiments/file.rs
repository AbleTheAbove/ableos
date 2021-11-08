struct Permissions {
    write_files: bool,
    read_files: bool,
    execute_files: bool,
    global_write_files: bool,
    global_read_files: bool,
    global_execute_files: bool,
}

pub struct File {
    owner: u8,
    permissions: Permissions,
    data: Vec<u8>,
}

pub struct Folder {
    owner: u8,
    permissions: Permissions,
    folders: Vec<Folder>,
    files: Vec<File>,
}
