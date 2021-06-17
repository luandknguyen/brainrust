# brainrust

**brainrust** is a Brainfuck interpreter written in Rust. It focuses on features over speed.

## ✨ Features

- Dynamic or static array size.
- Set input from and output to a file instead of command-line.
- Cell pointer wrapping (with `--wrapping` flag).
- Specify behavior when read EOF:
    + Leave the current cell as is.
    + Set the current cell to 0.
    + Set the current cell to -1 (underflow to 255).
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

## 📝 Notes

- Newline character defaults to MS-DOS convension (i.e. CRLF). Use `--newline_mode=LF` to change to Linux convension.
- When cell pointer is at the first cell, '<' will wrap around if `--wrapping` flag is set.
- When cell pointer is at the final cell, '>' will wrap around if `--wrapping` flag is set, unless `--dynamic_size` flag is also set.

## 🔖 Examples

*Notes: the examples in `examples` folder use CRLF.*

### 1. Hello World

This program prints `Hello World!` and the final array state.

**run**: ```brainfuck_rs "examples/hello_world.txt" --array_size=10 --final_array```

![hello_world.txt](examples/hello_world.png)

### 2. Hello World (shortest)

This program also prints `Hello World!` but is shorter than the previous program and requires wrapping.

**run**: ```brainfuck_rs "examples/hello_world_shortest.txt" --array_size=10 --final_array --wrapping```

![hello_world.txt](examples/hello_world_shortest.png)

### 3. Cat 

This program print out whatever the user put in.

**run**: ```brainfuck_rs "examples/cat.txt"```

*file: examples/cat.txt*
```
,[.,]
```

### 4. Multiply

This program read 2 number then multiply them and store the result in the third cell.

**run**: ```brainfuck_rs "examples/multiply.txt" --input_mode=digit --array_size=10 --final_array```

![multiply](examples/multiply.png)

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
