# Th(read)Get
Download content from 4chan threads using this CLI tool.

# Installing
1. Clone it.
2. Compile it with `cargo build --prod`
3. Move binary to wherever you like.
4. Add location where you moved thget binary to your PATH env variable.
5. Enjoy it.

# Usage
1. Look for a 4chan thread that you would like to download its content.
2. Run `thget https://boards.4chan.org/wg/thread/7712621#p7712621 [-o output]`.
3. For more help, use `thget --help`.

# Disclaimer
This is a program I made for practicing Rust programming (I'm not an expert as you can see).
Maybe in the future I will upgrade this program for supporting multi-threading downloading and
show a nice status bar. Also I wan't to add the possibility to exclude certain file types when
you are downloading a thread.

