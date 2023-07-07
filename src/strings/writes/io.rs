pub trait Write16 {
  fn write16(&mut self, buf: &[u16]) -> std::io::Result<usize>;
  fn flush16(&mut self) -> std::io::Result<()>;
  fn write16_all(&mut self, mut buf: &[u16]) -> std::io::Result<()> {
    while !buf.is_empty() {
      match self.write16(buf) {
        Ok(0) => {
          return Err(std::io::Error::new(
            std::io::ErrorKind::WriteZero,
            "failed to write whole buffer",
          ));
        }
        Ok(n) => buf = &buf[n..],
        Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
        Err(e) => return Err(e),
      }
    }
    Ok(())
  }
}

pub trait Write32 {
  fn write32(&mut self, buf: &[u32]) -> std::io::Result<usize>;
  fn flush32(&mut self) -> std::io::Result<()>;
  fn write32_all(&mut self, mut buf: &[u32]) -> std::io::Result<()> {
    while !buf.is_empty() {
      match self.write32(buf) {
        Ok(0) => {
          return Err(std::io::Error::new(
            std::io::ErrorKind::WriteZero,
            "failed to write whole buffer",
          ));
        }
        Ok(n) => buf = &buf[n..],
        Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
        Err(e) => return Err(e),
      }
    }
    Ok(())
  }
}

macro_rules! impl_for_slices {
  ($trait:ident, $type:ty, $write:ident, $flush:ident) => {
    impl $trait for &mut [$type] {
      fn $write(&mut self, buf: &[$type]) -> std::io::Result<usize> {
        let written = std::cmp::min(buf.len(), self.len());
        self.copy_from_slice(&buf[0..written]);
        *self = unsafe {
          std::slice::from_raw_parts_mut(self.as_mut_ptr().add(written), self.len() - written)
        };
        // *self = &mut self[written..];
        Ok(written)
      }

      fn $flush(&mut self) -> std::io::Result<()> {
        Ok(())
      }
    }
  };
}

impl_for_slices!(Write16, u16, write16, flush16);
impl_for_slices!(Write32, u32, write32, flush32);

macro_rules! impl_for_vecs {
  ($trait:ident, $type:ty, $write:ident, $flush:ident) => {
    impl $trait for Vec<$type> {
      fn $write(&mut self, buf: &[$type]) -> std::io::Result<usize> {
        let written = buf.len();
        let old_len = self.len();
        let written = if self.try_reserve_exact(written).is_err() {
          std::cmp::min(self.capacity() - old_len, written)
        } else {
          written
        };
        self.resize(old_len + written, 0);
        self[old_len..old_len + written].copy_from_slice(buf);
        Ok(written)
      }

      fn $flush(&mut self) -> std::io::Result<()> {
        Ok(())
      }
    }
  };
}

impl_for_vecs!(Write16, u16, write16, flush16);
impl_for_vecs!(Write32, u32, write32, flush32);
