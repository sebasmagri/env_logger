use std::io;

use crate::fmt::{WritableTarget, WriteStyle};

pub(in crate::fmt::writer) mod glob {}

pub(in crate::fmt::writer) struct BufferWriter {
    target: WritableTarget,
}

pub(in crate::fmt) struct Buffer(Vec<u8>);

impl BufferWriter {
    pub(in crate::fmt::writer) fn stderr(_is_test: bool, _write_style: WriteStyle) -> Self {
        BufferWriter {
            target: WritableTarget::Stderr,
        }
    }

    pub(in crate::fmt::writer) fn stdout(_is_test: bool, _write_style: WriteStyle) -> Self {
        BufferWriter {
            target: WritableTarget::Stdout,
        }
    }

    pub(in crate::fmt::writer) fn buffer(&self) -> Buffer {
        Buffer(Vec::new())
    }

    pub(in crate::fmt::writer) fn print(&self, buf: &Buffer) -> io::Result<()> {
        // This impl uses the `eprint` and `print` macros
        // instead of using the streams directly.
        // This is so their output can be captured by `cargo test`.
        match &self.target {
            WritableTarget::Stdout => print!("{}", String::from_utf8_lossy(&buf.0)),
            WritableTarget::Stderr => eprint!("{}", String::from_utf8_lossy(&buf.0)),
            WritableTarget::Pipe(_) => unreachable!(),
        }

        Ok(())
    }
}

impl Buffer {
    pub(in crate::fmt) fn clear(&mut self) {
        self.0.clear();
    }

    pub(in crate::fmt) fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.extend(buf);
        Ok(buf.len())
    }

    pub(in crate::fmt) fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    pub(in crate::fmt) fn bytes(&self) -> &[u8] {
        &self.0
    }
}
