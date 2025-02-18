use std::fmt::Debug;
// use super::lodging::{Accommodation, Description};
use crate::lodging::{Accommodation, Description, Hotel};

pub fn book_for_one_night(entity: &mut impl Accommodation, guest: &str) {
    entity.book(guest, 1);
}

pub fn book_for_one_night_gen<T: Accommodation>(entity: &mut T, guest: &str) {
    entity.book(guest, 1);
}

pub fn mix_and_match(
    first: &mut (impl Accommodation + Description),
    second: &mut impl Accommodation,
    guest: &str,
) {
    first.book(guest, 1);
    first.get_description();
    second.book(guest, 1);
}

pub fn mix_and_match_gen<T: Accommodation + Description, U: Accommodation>(
    first: &mut T,
    second: &mut U,
    guest: &str,
) {
    first.book(guest, 1);
    first.get_description();

    second.book(guest, 1);
}

pub fn mix_and_match_gen_where<T, U>(first: &mut T, second: &mut U, guest: &str)
where
    T: Accommodation + Description,
    U: Accommodation,
{
    first.book(guest, 1);
    first.get_description();

    second.book(guest, 1);
}

pub fn choose_best_place_to_stay() -> impl Accommodation + Description + Debug {
    let likes_luxury = true;
    if likes_luxury {
        Hotel::new("Lux Continental")
    } else {
        Hotel::new("Continental")
    }
}
