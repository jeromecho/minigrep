# minigrep

CLI tool that mimics grep (*g*lobally search for a *r*egular *e*xpression and *p*rint) behaviour. Modularized and thoroughly tested. Made with Rust. 

* Check **production** branch for polished and uncommented code


## Using the CLI Tool 

1. Clone app 
2. Type `cargo run <string_to_search> <file_name> [<case_insenitive?>]`

Optional: 
* Specify whether to do a case-insenitive grep by setting environment variable 
or using the third optional command line argument. Command line arguments overrule environment variables. 

```
IGNORE_CASE=1 cargo run <string_to_search> <file_name> [<case_insenitive?>]
```

