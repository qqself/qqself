echo "# qqself-client-cli" > README.md
echo "Command line client for qqself with common operations" >> README.md
echo "<pre>" >> README.md
cargo run -- help >> README.md
subcommands=("delete" "download" "init" "report" "upload")
for subcommand in "${subcommands[@]}"; do
    echo -e "\n# SUBCOMMAND: $subcommand" >> README.md
    cargo run -- help "$subcommand" | tail -n +2 >> README.md
done
echo "</pre>" >> README.md
