use std::{
    fs,
    io,
    path,
};
use std::io::BufRead;
use nom::{
    IResult,
};

pub trait ParseFrom<T> {
    fn parse_from(input: T) -> IResult<T, Self> where Self: std::marker::Sized;
}

pub struct Esp {
    //pub addresses: RegAddr,
}

pub fn parse_idf(path: &path::Path) -> Result<Esp, io::Error> {
    let esp = Esp{
        //addresses: parse_text_file(&path.join("soc.h"))?,
    };

    Ok(esp)
}

// TODO: Fix error types
// fn parse_text_file<'a, T>(path: &path::Path) -> Result<T, io::Error>
//     where T: ParseFrom<&'a str>
// {
//     let file = fs::File::open(path)?;
//     let reader = io::BufReader::new(file);

//     for line in reader.lines() {
//         println!("{}", line?);
//     }

//     T::parse_from("")
// }
