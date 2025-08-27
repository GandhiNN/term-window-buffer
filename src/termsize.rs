#![allow(unused)]
use libc::{STDOUT_FILENO, TIOCGWINSZ, c_ushort, ioctl};
use std::io::{self, IsTerminal};

/// Container for number of rows and columns
#[derive(Debug)]
pub struct Size {
    /// number of rows -> window height
    pub rows: u16,
    /// number of columns -> window width
    pub cols: u16,
}

/// A representation of the size of the
/// current terminal
#[repr(C)]
#[derive(Debug)]
pub struct UnixSize {
    /// number of rows
    pub rows: c_ushort,
    /// number of columns
    pub cols: c_ushort,
    /// x pixels
    x: c_ushort,
    /// y pixels
    y: c_ushort,
}

/// Gets the current terminal size
pub fn get_term_size() -> Option<Size> {
    // https://doc.rust-lang.org/beta/std/io/trait.IsTerminal.html
    // https://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let stdout = io::stdout();
    if !stdout.is_terminal() {
        return None;
    }
    let mut us = UnixSize {
        rows: 0,
        cols: 0,
        x: 0,
        y: 0,
    };
    // calling ioctl api from C -> https://man7.org/linux/man-pages/man2/ioctl.2.html
    // returns zero on success and -1 on error.
    // 1st arg: File descriptor is STDOUT_FILENO is an integer file descriptor
    // 2nd arg: TIOCGWINSZ as the second arg means we are getting the window size
    // 3rd arg: pointer to our UnixSize struct which will be populated by ioctl call
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut us) };
    if r == 0 {
        println!("{:#?}", us);
        Some(Size {
            rows: us.rows,
            cols: us.cols,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Size, get_term_size};
    use std::process::{Command, Output, Stdio};

    pub fn stty_size() -> Output {
        // execute the following command on shell:
        // stty -F /dev/stderr size
        // which basically print the number of rows and columns
        // according to the kernel
        Command::new("stty")
            .arg("-F")
            .arg("/dev/stderr")
            .arg("size")
            .stderr(Stdio::inherit())
            .output()
            .expect("expected stty output")
    }

    #[test]
    pub fn test_shell() {
        let output = stty_size();
        println!("{:#?}", output);
        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).expect("expected utf8");
        let mut data = stdout.split_whitespace();
        let rs = data
            .next()
            .expect("expected row")
            .parse::<u16>()
            .expect("expected u16 col");
        let cs = data
            .next()
            .expect("expected col")
            .parse::<u16>()
            .expect("expected u16 col");
        if let Some(Size { rows, cols }) = get_term_size() {
            assert_eq!(rows, rs);
            assert_eq!(cols, cs);
        }
    }
}
