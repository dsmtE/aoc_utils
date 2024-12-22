use nom::{
    bytes::complete::tag, character::complete::{digit1, space1}, combinator::{map_res, opt, recognize}, multi::separated_list1, sequence::preceded, IResult
};

pub fn number<T: std::str::FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(preceded(opt(tag("-")), digit1)), str::parse::<T>)(input)
}

pub fn numbers_list<T: std::str::FromStr>(input: &str) -> IResult<&str, Vec<T>> {
    separated_list1(space1, number::<T>)(input)
}