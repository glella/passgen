# passclip
Simple password generator in Rust

Generates a random password and copies it to the clipboard

Clipboard access compatible with OSX, Windows and Linux


USAGE:

    passgen [OPTIONS]

FLAGS:

    -h, --help       Prints help information
    
    -V, --version    Prints version information

OPTIONS:

    -d <digits>         Number of digits, defaults to 2 when omitted
    
    -l <length>         Password length, defaults to 16 when omitted
    
    -s <special>        Special characters, defaults to 2 when omitted
    
    -u <upper>          Uppercase characters, defaults to 2 when omitted
