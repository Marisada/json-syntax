use super::{Context, Error, Parse, Parser};
use crate::object::Key;
use decoded_char::DecodedChar;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum StartFragment {
	Empty,
	NonEmpty((Key, usize)),
}

impl Parse for StartFragment {
	fn parse_in<C, E>(
		parser: &mut Parser<C, E>,
		_context: Context,
	) -> Result<(Self, usize), Error<E>>
	where
		C: Iterator<Item = Result<DecodedChar, E>>,
	{
		let i = parser.begin_fragment();
		match parser.next_char()? {
			(_, Some('{')) => {
				parser.skip_whitespaces()?;

				match parser.peek_char()? {
					Some('}') => {
						parser.next_char()?;
						Ok((StartFragment::Empty, i))
					}
					_ => {
						let e = parser.begin_fragment();
						let key = Key::parse_in(parser, Context::ObjectKey)?;
						parser.skip_whitespaces()?;
						match parser.next_char()? {
							(_, Some(':')) => Ok((Self::NonEmpty((key.0, e)), i)),
							(p, unexpected) => Err(Error::unexpected(p, unexpected)),
						}
					}
				}
			}
			(p, unexpected) => Err(Error::unexpected(p, unexpected)),
		}
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ContinueFragment {
	End,
	Entry((Key, usize)),
}

impl ContinueFragment {
	pub fn parse_in<C, E>(parser: &mut Parser<C, E>, object: usize) -> Result<Self, Error<E>>
	where
		C: Iterator<Item = Result<DecodedChar, E>>,
	{
		parser.skip_whitespaces()?;
		match parser.next_char()? {
			(_, Some(',')) => {
				parser.skip_whitespaces()?;
				let e = parser.begin_fragment();
				let key = Key::parse_in(parser, Context::ObjectKey)?;
				parser.skip_whitespaces()?;
				match parser.next_char()? {
					(_, Some(':')) => Ok(Self::Entry((key.0, e))),
					(p, unexpected) => Err(Error::unexpected(p, unexpected)),
				}
			}
			(_, Some('}')) => {
				parser.end_fragment(object);
				Ok(Self::End)
			}
			(p, unexpected) => Err(Error::unexpected(p, unexpected)),
		}
	}
}
