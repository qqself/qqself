# qqself-client-cli
Command line client for qqself with common operations
<pre>
qqself-client-cli 0.0.0

USAGE:
    qqself-client-cli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    delete      Delete all the records from the server
    download    Download all the entries from the server to the file
    help        Prints this message or the help of the given subcommand(s)
    init        Creates new key file
    report      Read the journal and report current state of things
    upload      Uploads all the records from journal file to the server

# SUBCOMMAND: delete
Delete all the records from the server

USAGE:
    qqself-client-cli delete [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --keys-path <keys-path>    Path to key file [default: qqself_keys.txt]

# SUBCOMMAND: download
Download all the entries from the server to the file

USAGE:
    qqself-client-cli download [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --keys-path <keys-path>            Path to key file [default: qqself_keys.txt]
    -o, --output-folder <output-folder>    Path to folder where journal will be created with the name format of
                                           `qqself_journal_[TODAY].txt` [default: .]

# SUBCOMMAND: init
Creates new key file

USAGE:
    qqself-client-cli init [FLAGS] [OPTIONS]

FLAGS:
    -h, --help         Prints help information
    -o, --overwrite    If existing config file should be ignored and overwritten
    -V, --version      Prints version information

OPTIONS:
    -k, --keys-path <keys-path>    Where new generated keys will be stored [default: qqself_keys.txt]

# SUBCOMMAND: report
Read the journal and report current state of things

USAGE:
    qqself-client-cli report [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --journal-path <journal-path>    Path to journal file with all the entries [default: journal.txt]
    -p, --period <period>                Period of time to make a report for [default: day]  [possible values: Day,
                                         Week, Month, Year]

# SUBCOMMAND: upload
Uploads all the records from journal file to the server

USAGE:
    qqself-client-cli upload [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --journal-path <journal-path>    Path to journal file with all the entries [default: journal.txt]
    -k, --keys-path <keys-path>          Path to key file [default: qqself_keys.txt]
</pre>
