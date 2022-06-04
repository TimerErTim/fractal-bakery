use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::mem;
use std::rc::Rc;

use crate::interpolation::interpolatable::{Interpolatable, InterpolatableLocation, Interpolation};
use crate::interpolation::interpolation_list::simple::SimpleHasher;

pub struct InterpolationList<T: Interpolatable<T, Output=T>> {
    buffer: HashMap<u64, T, BuildHasherDefault<SimpleHasher>>,
    first_entry: Option<Rc<InterpolationListValue<T>>>,
    last_entry: InterpolationListValue<T>,
}

impl<T: Interpolatable<T, Output=T>> InterpolationList<T> {
    pub fn new(distance: u64, value: T) -> InterpolationList<T> {
        let initial_entry = InterpolationListValue::new(distance, value);

        InterpolationList {
            buffer: HashMap::<u64, T, BuildHasherDefault<SimpleHasher>>::with_hasher(
                BuildHasherDefault::<SimpleHasher>::default()
            ),
            first_entry: None,
            last_entry: initial_entry,
        }
    }

    pub fn insert(&mut self, interpolation: Interpolation, length: u64, value: T) {
        let new_value = InterpolationListValue::new(
            self.last_entry.sum_distance + length,
            value,
        );

        let old_last_entry = mem::replace(&mut self.last_entry, new_value);

        let link = {
            InterpolationListLink::<T> {
                interpolation,
                //length,
                left_value: Rc::new(old_last_entry),
            }
        };

        self.last_entry.left_link = Some(link);

        if self.first_entry.is_none() {
            self.first_entry = Some(Rc::clone(&self.last_entry.left_link.as_ref().unwrap().left_value))
        }
    }

    pub fn get_buffered(&mut self, position: u64) -> &T {
        if let Some(first_entry) = &self.first_entry {
            if position <= first_entry.sum_distance {
                &first_entry.value
            } else if position >= self.last_entry.sum_distance {
                &self.last_entry.value
            } else {
                self.buffer.entry(position).or_insert(Self::interpolate_at(&self.last_entry, position))
            }
        } else {
            &self.last_entry.value
        }
    }

    fn interpolate_at(entry: &InterpolationListValue<T>, position: u64) -> T {
        let mut current_entry = entry;
        let mut prev_entry;
        let mut current_link;
        loop {
            current_link = current_entry.left_link.as_ref().unwrap();
            prev_entry = current_entry;
            current_entry = &current_link.left_value;
            if current_entry.sum_distance < position { break };
        }
        let left_entry = &*current_link.left_value;
        let right_entry = prev_entry;

        let left_location = InterpolatableLocation::new(
            &left_entry.value,
            left_entry.sum_distance as f64,
        );
        let right_location = InterpolatableLocation::new(
            &right_entry.value,
            right_entry.sum_distance as f64,
        );

        current_link.interpolation.interpolate(&left_location, position as f64, &right_location)
    }

    #[inline]
    pub fn get_max_position(&self) -> u64 {
        self.last_entry.sum_distance
    }

    #[inline]
    pub fn get_min_position(&self) -> u64 {
        if let Some(first_entry) = &self.first_entry {
            first_entry.sum_distance
        } else {
            self.get_max_position()
        }
    }
}

struct InterpolationListLink<T: Interpolatable<T, Output=T>> {
    interpolation: Interpolation,
    //length: u64,
    left_value: Rc<InterpolationListValue<T>>,
}

struct InterpolationListValue<T: Interpolatable<T, Output=T>> {
    value: T,
    sum_distance: u64,
    left_link: Option<InterpolationListLink<T>>,
}

impl<T: Interpolatable<T, Output=T>> InterpolationListValue<T> {
    fn new(distance: u64, value: T) -> InterpolationListValue<T> {
        InterpolationListValue {
            value,
            sum_distance: distance,
            left_link: None,
        }
    }
}

mod simple {
    use std::hash::Hasher;

    #[derive(Default)]
    pub struct SimpleHasher(u64);

    #[inline]
    fn load_u64_le(buf: &[u8], len: usize) -> u64 {
        use std::ptr;
        debug_assert!(len <= buf.len());
        let mut data = 0u64;
        unsafe {
            ptr::copy_nonoverlapping(buf.as_ptr(), &mut data as *mut _ as *mut u8, len);
        }
        data.to_le()
    }

    impl Hasher for SimpleHasher {
        #[inline]
        fn finish(&self) -> u64 {
            self.0
        }

        #[inline]
        fn write(&mut self, bytes: &[u8]) {
            *self = SimpleHasher(load_u64_le(bytes, bytes.len()));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interpolatable::Interpolation;
    use crate::Interpolation;

    use super::InterpolationList;

    #[test]
    fn basics() {
        let mut list = InterpolationList::new(10, 0);

        // check initial list
        assert_eq!(*list.get_buffered(0), 0);
        assert_eq!(*list.get_buffered(5), 0);
        assert_eq!(*list.get_buffered(10), 0);
        assert_eq!(*list.get_buffered(30), 0);

        list.insert(Interpolation::LINEAR, 10, 100);

        // check list with first and last reference
        assert_eq!(*list.get_buffered(0), 0);
        assert_eq!(*list.get_buffered(5), 0);
        assert_eq!(*list.get_buffered(10), 0);
        assert_eq!(*list.get_buffered(15), 50);
        assert_eq!(*list.get_buffered(20), 100);
        assert_eq!(*list.get_buffered(30), 100);

        // check list has buffer
        assert_eq!(*list.buffer.get(&15u64).unwrap(), 50);

        list.insert(Interpolation::CUBIC, 20, 50);

        // check list resets necessary buffer
        assert_eq!(*list.get_buffered(0), 0);
        assert_eq!(*list.get_buffered(5), 0);
        assert_eq!(*list.get_buffered(10), 0);
        assert_eq!(*list.get_buffered(15), 50);
        assert_eq!(*list.get_buffered(20), 100);
        assert_eq!(*list.get_buffered(23), 97);
        assert_eq!(*list.get_buffered(30), 75);
        assert_eq!(*list.get_buffered(40), 50);

        // check list buffer still valid
        assert_eq!(*list.buffer.get(&15u64).unwrap(), 50);
        assert_eq!(*list.buffer.get(&23u64).unwrap(), 97);
    }

    #[test]
    fn bounds() {
        let mut list = InterpolationList::new(10, 0);

        // check lowest and highest bound
        assert_eq!(list.get_min_position(), 10);
        assert_eq!(list.get_max_position(), 10);

        list.insert(Interpolation::LINEAR, 10, 100);

        // check new lowest and highest bound
        assert_eq!(list.get_min_position(), 10);
        assert_eq!(list.get_max_position(), 20);

        list.insert(Interpolation::CUBIC, 20, 50);

        // check even newer lowest and highest bound
        assert_eq!(list.get_min_position(), 10);
        assert_eq!(list.get_max_position(), 40);
    }
}
