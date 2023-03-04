use super::AsAny;
use crate::cid;

pub trait Component {
    fn id() -> usize {
        let val = cid!();

        println!("{val}");

        val
    }
}

impl<'a, C> AsAny<'a> for C where C: Component + 'a {}
