# minicat: A self-made CLI tool written with Rust-lang, simple implement of cat.

## USAGE
	minicat [OPTIONS] [FILE]
## OPTIONS

+ -A, --show-all           
equivalent to -vET
+ -b, --number-nonblank    
number nonempty output lines, overrides -n
+ -e               
equivalent to -vE
+ -E, --show-ends          
display $ at end of each line
+ -n, --number             
number all output lines
+ ~~-s, --squeeze-blank~~                
~~suppress repeated empty output lines~~
+ -t                                 
equivalent to -vT
+ -T, --show-tabs          
display TAB characters as ^I
+ -u                       
(ignore)
+ -v                                       
display non-ASCII characters as @
+ --help               
display help and exit
+ --version            
output version information and exit
