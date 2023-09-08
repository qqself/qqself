# qqself-client-cli
Command line client for qqself with common operations
<pre>
Usage: qqself-client-cli <COMMAND>

Commands:
  init      Creates new key file
  upload    Uploads all the records from journal file to the server
  download  Download all the entries from the server to the file
  report    Read the journal and report current state of things
  delete    Delete all the records from the server
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

# SUBCOMMAND: delete

Usage: qqself-client-cli delete [OPTIONS]

Options:
  -k, --keys-path <KEYS_PATH>  Path to key file [default: qqself_keys.txt]
  -h, --help                   Print help

# SUBCOMMAND: download

Usage: qqself-client-cli download [OPTIONS]

Options:
  -o, --output-folder <OUTPUT_FOLDER>  Path to folder where journal will be created with the name format of `qqself_journal_[TODAY].txt` [default: .]
  -k, --keys-path <KEYS_PATH>          Path to key file [default: qqself_keys.txt]
  -h, --help                           Print help

# SUBCOMMAND: init

Usage: qqself-client-cli init [OPTIONS]

Options:
  -k, --keys-path <KEYS_PATH>  Where new generated keys will be stored [default: qqself_keys.txt]
  -o, --overwrite              If existing config file should be ignored and overwritten
  -h, --help                   Print help

# SUBCOMMAND: report

Usage: qqself-client-cli report [OPTIONS]

Options:
  -j, --journal-path <JOURNAL_PATH>  Path to journal file with all the entries [default: journal.txt]
  -p, --period <PERIOD>              Period of time to make a report for [default: day] [possible values: day, week, month, year]
  -h, --help                         Print help

# SUBCOMMAND: upload

Usage: qqself-client-cli upload [OPTIONS]

Options:
  -j, --journal-path <JOURNAL_PATH>  Path to journal file with all the entries [default: journal.txt]
  -k, --keys-path <KEYS_PATH>        Path to key file [default: qqself_keys.txt]
  -h, --help                         Print help
</pre>
