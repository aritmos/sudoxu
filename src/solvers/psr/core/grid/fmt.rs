use super::{Cell, Grid};

use std::fmt::Debug;

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = unsafe { std::mem::transmute::<_, [[Cell; 9]; 9]>(self.0) };
        for r in rows {
            writeln!(
                f,
                "{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8]
            )?;
        }
        Ok(())
    }
}
