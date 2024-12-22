use voladdress::{Safe, VolAddress};

pub trait SaneApplyBehaviour<T: Copy> {
    fn mutate<F: FnOnce(T) -> T>(&self, block: F);
}

impl<T: Copy> SaneApplyBehaviour<T> for VolAddress<T, Safe, Safe> {
    fn mutate<F: FnOnce(T) -> T>(&self, block: F) {
        let mut value = self.read();
        value = block(value);
        self.write(value);
    }
}
