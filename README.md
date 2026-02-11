# rcp

Copy files between local and remote systems. The rcp server must be running on the remote system.

## Usage

### Copy file/directory

```text
rcp <SRC> <DEST>
```

`SRC` and `DEST` can be either local or remote paths:

- local path can be either absolute or relative path, directory must end with `/`:
  - `/absolute/local/path`
  - `relative/local/path`
  - `/absolute/local/directory/`
  - `relative/local/directory/`
- remote path must be in the format of `HOST[:PORT]:PATH`, `PATH` has the same rules as local path:
  - `hostname:/absolute/remote/path`
  - `hostname:relative/remote/path`
  - `hostname:/absolute/remote/directory/`
  - `hostname:relative/remote/directory/`
  - `hostname:1234:/absolute/remote/path`

### Run rcp server

```text
rcp [OPTIONS]
```

Options:
- `-p, --port <PORT>`        Port to listen on (default: 14523)
