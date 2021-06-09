# brainfuck_rs

**brainfuck_rs** a Brainfuck interpreter written in Rust. It focuses on features over speed.

## Features

- **Dynamic** or **Static** array size.
- Cell pointer wrapping (with `--wrapping` flag).
- Specify behavior when read EOF:
    + Leave the current cell as is.
    + Set the current cell to 0.
    + Set the current cell to -1.
- Select between ASCII or digit input mode.
- Specify newline character (CRLF or LF).

```
    -i <INPUT>                          Specify which file to read input from. Default: stdin.
    -o <OUTPUT>                         Specify which file to write output to. Default: stdout.
    
    -s, --array_size <SIZE>             Specify the size of array. Default: 30000.
    -d, --dynamic_size                  Use dynamic size instead of fixed size array. If this flag is set, `--array_size`
                                            will specify the initial size.
    -w, --wrapping                      Wrapping '>' and '<'. "--dynamic_size" will override this flag.

        --final_array                   Display final array after program finished.
        --ignore_newline                Ignore newline input character. Flag is set by default if input is stdin.
        
        --eof_behavior <EOF_BEHAVIOR>   Behavior when received EOF as input:
                                            as_is [Default] -- leave the current cell untouched.
                                            zero -- set the current cell value to 0.
                                            negative_one -- set the current cell value to -1.

        --input_mode <INPUT_MODE>       Select input mode:
                                            ascii [Default] -- no input conversion.
                                            digit -- convert input into digit (between 0 ... 255)

        --newline_mode <NEWLINE_MODE>   Select newline mode: CRLF or LF. Default: CRLF
```

## Examples

*Notes: the examples in `examples` folder use CRLF.*

### Hello World

This program prints `Hello World!` and the final array state.

examples/hello_world.txt
```
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

run: ```brainfuck_rs "examples/hello_world.txt" --array_size=10 --final_array```

output:
```
Hello World!

Final array: [0, 0, 72, 100, 87, 33, 10, 0, 0, 0]
```

### Hello World (shortest)

This program also prints `Hello World!` but is shorter than the previous program and requires wrapping.

examples/hello_world_shortest.txt
```
+[-->-[>>+>-----<<]<--<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.
```

run: ```brainfuck_rs "examples/hello_world.txt" --array_size=10 --final_array --wrapping```

output:
```
Hello, World!
Final array: [172, 108, 44, 33, 87, 0, 72, 0, 153, 100]
```

## Notes

- When cell pointer is at the first cell, '<' will wrap around if `--wrapping` flag is set.
- When cell pointer is at the final cell, '>' will wrap around if `--wrapping` flag is set, unless `--dynamic_size` flag is also set.

## 🔰 Commit Emoji Guide

| Emoji          | Meaning        |
| -------------- | -------------- |
| :bug:          | Bugfix         |
| :package:      | Dependency     |
| :no_entry:     | Deprecation    |
| :book:         | Documentation  |
| :sparkles:     | Features       |
| :construction: | In-Progress    |
| :zap:          | Performance    |
| :recycle:      | Refactoring    |
| :lock:         | Security       |
| :test_tube:    | Tests          |
| :pencil:       | Typos          |
| :lipstick:     | UI / Cosmetic  |
| :bookmark:     | Version        |
|                |                |
| :tada:         | Initial Commit |
| :rocket:       | Release        |
| :rewind:       | Revert Changes |
