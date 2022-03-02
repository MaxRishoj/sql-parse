// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use alloc::vec;
use alloc::vec::Vec;

use crate::{
    expression::{parse_expression, Expression},
    keywords::Keyword,
    lexer::Token,
    parser::{ParseError, Parser},
    select::{parse_select, Select},
    Identifier, Issue, OptSpanned, Span, Spanned,
};

/// Flags for replace
#[derive(Clone, Debug)]
pub enum ReplaceFlag {
    LowPriority(Span),
    Delayed(Span),
}

impl<'a> Spanned for ReplaceFlag {
    fn span(&self) -> Span {
        match &self {
            ReplaceFlag::LowPriority(v) => v.span(),
            ReplaceFlag::Delayed(v) => v.span(),
        }
    }
}

/// Representation of replace Statement
///
/// ```
/// # use sql_parse::{SQLDialect, SQLArguments, ParseOptions, parse_statement, Replace, Statement};
/// # let options = ParseOptions::new().dialect(SQLDialect::MariaDB);
/// # let mut issues = Vec::new();
/// #
/// let sql = "REPLACE INTO t2 VALUES (1,'Leopard'),(2,'Dog')";
/// let stmt = parse_statement(sql, &mut issues, &options);
///
/// # assert!(issues.is_empty());
/// #
/// let r: Replace = match stmt {
///     Some(Statement::Replace(r)) => r,
///     _ => panic!("We should get an replace statement")
/// };
///
/// assert!(r.table[0].as_str() == "t2");
/// println!("{:#?}", r.values.unwrap())
/// ```
#[derive(Clone, Debug)]
pub struct Replace<'a> {
    /// Span of "REPLACE"
    pub replace_span: Span,
    /// Flags specified after "REPLACE"
    pub flags: Vec<ReplaceFlag>,
    /// Span of "INTO" if specified
    pub into_span: Option<Span>,
    /// Table to replace into
    pub table: Vec<Identifier<'a>>,
    /// List of columns to put values into
    pub columns: Vec<Identifier<'a>>,
    /// Span of "VALUES" and values to put into columns if specified
    pub values: Option<(Span, Vec<Vec<Expression<'a>>>)>,
    /// Select expression to put into columns if specified
    pub select: Option<Select<'a>>,
    /// Span of "SET" and list of key, value pairs to set if specified
    pub set: Option<(Span, Vec<(Identifier<'a>, Expression<'a>)>)>,
}

impl<'a> Spanned for Replace<'a> {
    fn span(&self) -> Span {
        self.replace_span
            .join_span(&self.flags)
            .join_span(&self.into_span)
            .join_span(&self.table)
            .join_span(&self.values)
            .join_span(&self.columns)
            .join_span(&self.select)
    }
}

pub(crate) fn parse_replace<'a, 'b>(
    parser: &mut Parser<'a, 'b>,
) -> Result<Replace<'a>, ParseError> {
    let replace_span = parser.consume_keyword(Keyword::REPLACE)?;
    let mut flags = Vec::new();

    loop {
        match &parser.token {
            Token::Ident(_, Keyword::LOW_PRIORITY) => flags.push(ReplaceFlag::LowPriority(
                parser.consume_keyword(Keyword::LOW_PRIORITY)?,
            )),
            Token::Ident(_, Keyword::DELAYED) => flags.push(ReplaceFlag::Delayed(
                parser.consume_keyword(Keyword::DELAYED)?,
            )),
            _ => break,
        }
    }
    let into_span = parser.skip_keyword(Keyword::INTO);

    let mut table = vec![parser.consume_plain_identifier()?];
    loop {
        if parser.skip_token(Token::Period).is_none() {
            break;
        }
        table.push(parser.consume_plain_identifier()?);
    }
    // [PARTITION (partition_list)]

    let mut columns = Vec::new();
    if parser.skip_token(Token::LParen).is_some() {
        parser.recovered(")", &|t| t == &Token::RParen, |parser| {
            loop {
                columns.push(parser.consume_plain_identifier()?);
                if parser.skip_token(Token::Comma).is_none() {
                    break;
                }
            }
            Ok(())
        })?;
        parser.consume_token(Token::RParen)?;
    }

    let mut select = None;
    let mut values = None;
    let mut set = None;
    match &parser.token {
        Token::Ident(_, Keyword::SELECT) => {
            select = Some(parse_select(parser)?);
        }
        Token::Ident(_, Keyword::VALUE | Keyword::VALUES) => {
            let values_span = parser.consume();
            let mut values_items = Vec::new();
            loop {
                let mut vals = Vec::new();
                parser.consume_token(Token::LParen)?;
                parser.recovered(")", &|t| t == &Token::RParen, |parser| {
                    loop {
                        vals.push(parse_expression(parser, false)?);
                        if parser.skip_token(Token::Comma).is_none() {
                            break;
                        }
                    }
                    Ok(())
                })?;
                parser.consume_token(Token::RParen)?;
                values_items.push(vals);
                if parser.skip_token(Token::Comma).is_none() {
                    break;
                }
            }
            values = Some((values_span, values_items));
        }
        Token::Ident(_, Keyword::SET) => {
            let set_span = parser.consume_keyword(Keyword::SET)?;
            let mut kvps = Vec::new();
            loop {
                let col = parser.consume_plain_identifier()?;
                parser.consume_token(Token::Eq)?;
                let val = parse_expression(parser, false)?;
                kvps.push((col, val));
                if parser.skip_token(Token::Comma).is_none() {
                    break;
                }
            }
            if let Some(cs) = columns.opt_span() {
                parser.issues.push(
                    Issue::err("Columns may not be used here", &cs)
                        .frag("Together with SET", &set_span),
                );
            }
            set = Some((set_span, kvps));
        }
        _ => {
            parser.expected_error("Expected VALUE, VALUES, SELECT or SET");
        }
    }

    //  [ ON DUPLICATE KEY UPDATE
    //    col=expr
    //      [, col=expr] ... ] [RETURNING select_expr
    //       [, select_expr ...]]
    Ok(Replace {
        flags,
        replace_span,
        table,
        into_span,
        values,
        select,
        columns,
        set,
    })
}
