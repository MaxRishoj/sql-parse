use alloc::{boxed::Box, vec::Vec};

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
use crate::{
    data_type::parse_data_type,
    keywords::Keyword,
    lexer::Token,
    parser::{ParseError, Parser},
    select::{parse_select, Select},
    statement::parse_statement,
    DataType, Identifier, SString, Span, Spanned, Statement,
};

#[derive(Clone, Debug)]
pub enum TableOption<'a> {
    AutoExtendSize {
        identifier: Span,
        value: Identifier<'a>,
    },
    AutoIncrement {
        identifier: Span,
        value: Identifier<'a>,
    },
    AvgRowLength {
        identifier: Span,
        value: Identifier<'a>,
    },
    CharSet {
        identifier: Span,
        value: Identifier<'a>,
    },
    DefaultCharSet {
        identifier: Span,
        value: Identifier<'a>,
    },
    Checksum {
        identifier: Span,
        value: (bool, Span),
    },
    Collate {
        identifier: Span,
        value: Identifier<'a>,
    },
    DefaultCollate {
        identifier: Span,
        value: Identifier<'a>,
    },
    Comment {
        identifier: Span,
        value: SString<'a>,
    },
    Compression {
        identifier: Span,
        value: SString<'a>,
    },
    Connection {
        identifier: Span,
        value: SString<'a>,
    },
    DataDirectory {
        identifier: Span,
        value: SString<'a>,
    },
    IndexDirectory {
        identifier: Span,
        value: SString<'a>,
    },
    DelayKeyWrite {
        identifier: Span,
        value: (bool, Span),
    },
    Encryption {
        identifier: Span,
        value: (bool, Span),
    },
    Engine {
        identifier: Span,
        value: Identifier<'a>,
    },
    EngineAttribute {
        identifier: Span,
        value: SString<'a>,
    },
    InsertMethod {
        identifier: Span,
        value: Identifier<'a>,
    },
    KeyBlockSize {
        identifier: Span,
        value: (usize, Span),
    },
    MaxRows {
        identifier: Span,
        value: (usize, Span),
    },
    MinRows {
        identifier: Span,
        value: (usize, Span),
    },
    // PACK_KEYS
    Password {
        identifier: Span,
        value: SString<'a>,
    },
    RowFormat {
        identifier: Span,
        value: Identifier<'a>,
    },
    SecondaryEngineAttribute {
        identifier: Span,
        value: SString<'a>,
    },
    //StatsAutoRecalc
    //StatsPersistance
    //StatsSamplePages
    //TABLESPACE
    //UNION
}

impl<'a> Spanned for TableOption<'a> {
    fn span(&self) -> Span {
        match &self {
            TableOption::AutoExtendSize { identifier, value } => identifier.span().join_span(value),
            TableOption::AutoIncrement { identifier, value } => identifier.span().join_span(value),
            TableOption::AvgRowLength { identifier, value } => identifier.span().join_span(value),
            TableOption::CharSet { identifier, value } => identifier.span().join_span(value),
            TableOption::DefaultCharSet { identifier, value } => identifier.span().join_span(value),
            TableOption::Checksum { identifier, value } => identifier.span().join_span(value),
            TableOption::Collate { identifier, value } => identifier.span().join_span(value),
            TableOption::DefaultCollate { identifier, value } => identifier.span().join_span(value),
            TableOption::Comment { identifier, value } => identifier.span().join_span(value),
            TableOption::Compression { identifier, value } => identifier.span().join_span(value),
            TableOption::Connection { identifier, value } => identifier.span().join_span(value),
            TableOption::DataDirectory { identifier, value } => identifier.span().join_span(value),
            TableOption::IndexDirectory { identifier, value } => identifier.span().join_span(value),
            TableOption::DelayKeyWrite { identifier, value } => identifier.span().join_span(value),
            TableOption::Encryption { identifier, value } => identifier.span().join_span(value),
            TableOption::Engine { identifier, value } => identifier.span().join_span(value),
            TableOption::EngineAttribute { identifier, value } => {
                identifier.span().join_span(value)
            }
            TableOption::InsertMethod { identifier, value } => identifier.span().join_span(value),
            TableOption::KeyBlockSize { identifier, value } => identifier.span().join_span(value),
            TableOption::MaxRows { identifier, value } => identifier.span().join_span(value),
            TableOption::MinRows { identifier, value } => identifier.span().join_span(value),
            TableOption::Password { identifier, value } => identifier.span().join_span(value),
            TableOption::RowFormat { identifier, value } => identifier.span().join_span(value),
            TableOption::SecondaryEngineAttribute { identifier, value } => {
                identifier.span().join_span(value)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum CreateDefinition<'a> {
    ColumnDefinition {
        identifier: Identifier<'a>,
        data_type: DataType<'a>,
    },
}

impl<'a> Spanned for CreateDefinition<'a> {
    fn span(&self) -> Span {
        match &self {
            CreateDefinition::ColumnDefinition {
                identifier,
                data_type,
            } => identifier.span().join_span(data_type),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CreateAlgorithm {
    Undefined(Span),
    Merge(Span),
    TempTable(Span),
}
impl<'a> Spanned for CreateAlgorithm {
    fn span(&self) -> Span {
        match &self {
            CreateAlgorithm::Undefined(s) => s.span(),
            CreateAlgorithm::Merge(s) => s.span(),
            CreateAlgorithm::TempTable(s) => s.span(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CreateOption<'a> {
    OrReplace(Span),
    Temporary(Span),
    Algorithm(Span, CreateAlgorithm),
    Definer {
        definer_span: Span,
        user: Identifier<'a>,
        host: Identifier<'a>,
    },
    SqlSecurityDefiner(Span, Span),
    SqlSecurityUser(Span, Span),
}
impl<'a> Spanned for CreateOption<'a> {
    fn span(&self) -> Span {
        match &self {
            CreateOption::OrReplace(v) => v.span(),
            CreateOption::Temporary(v) => v.span(),
            CreateOption::Algorithm(s, a) => s.join_span(a),
            CreateOption::Definer {
                definer_span,
                user,
                host,
            } => definer_span.join_span(user).join_span(host),
            CreateOption::SqlSecurityDefiner(a, b) => a.join_span(b),
            CreateOption::SqlSecurityUser(a, b) => a.join_span(b),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateTable<'a> {
    pub create_span: Span,
    pub create_options: Vec<CreateOption<'a>>,
    pub table_span: Span,
    pub identifier: Identifier<'a>,
    pub if_not_exists: Option<Span>,
    pub create_definitions: Vec<CreateDefinition<'a>>,
    pub options: Vec<TableOption<'a>>,
}

impl<'a> Spanned for CreateTable<'a> {
    fn span(&self) -> Span {
        self.create_span
            .join_span(&self.create_options)
            .join_span(&self.table_span)
            .join_span(&self.identifier)
            .join_span(&self.if_not_exists)
            .join_span(&self.create_definitions)
            .join_span(&self.options)
    }
}

#[derive(Clone, Debug)]
pub struct CreateView<'a> {
    pub create_span: Span,
    pub create_options: Vec<CreateOption<'a>>,
    pub view_span: Span,
    pub if_not_exists: Option<Span>,
    pub name: Identifier<'a>,
    pub as_span: Span,
    pub select: Select<'a>,
}

impl<'a> Spanned for CreateView<'a> {
    fn span(&self) -> Span {
        self.create_span
            .join_span(&self.create_options)
            .join_span(&self.view_span)
            .join_span(&self.if_not_exists)
            .join_span(&self.name)
            .join_span(&self.as_span)
            .join_span(&self.select)
    }
}

pub(crate) fn parse_create_definition<'a, 'b>(
    parser: &mut Parser<'a, 'b>,
) -> Result<CreateDefinition<'a>, ParseError> {
    match &parser.token {
        Token::Ident(_, _) => Ok(CreateDefinition::ColumnDefinition {
            identifier: parser.consume_plain_identifier()?,
            data_type: parse_data_type(parser)?,
        }),
        _ => parser.expected_failure("identifier"),
    }
}

fn parse_create_view<'a, 'b>(
    parser: &mut Parser<'a, 'b>,
    create_span: Span,
    create_options: Vec<CreateOption<'a>>,
) -> Result<Statement<'a>, ParseError> {
    let view_span = parser.consume_keyword(Keyword::VIEW)?;

    let if_not_exists = if let Some(if_) = parser.skip_keyword(Keyword::IF) {
        Some(
            parser
                .consume_keywords(&[Keyword::NOT, Keyword::EXISTS])?
                .join_span(&if_),
        )
    } else {
        None
    };

    let name = parser.consume_plain_identifier()?;
    // TODO (column_list)

    let as_span = parser.consume_keyword(Keyword::AS)?;

    let select = parse_select(parser)?;

    // TODO [WITH [CASCADED | LOCAL] CHECK OPTION]

    Ok(Statement::CreateView(CreateView {
        create_span,
        create_options,
        view_span,
        if_not_exists,
        name,
        as_span,
        select,
    }))
}

#[derive(Clone, Debug)]
pub enum FunctionCharacteristic<'a> {
    LanguageSql(Span),
    NotDeterministic(Span),
    Deterministic(Span),
    ContainsSql(Span),
    NoSql(Span),
    ReadsSqlData(Span),
    ModifiesSqlData(Span),
    SqlSecurityDefiner(Span),
    SqlSecurityUser(Span),
    Comment(SString<'a>),
}

impl<'a> Spanned for FunctionCharacteristic<'a> {
    fn span(&self) -> Span {
        match &self {
            FunctionCharacteristic::LanguageSql(v) => v.span(),
            FunctionCharacteristic::NotDeterministic(v) => v.span(),
            FunctionCharacteristic::Deterministic(v) => v.span(),
            FunctionCharacteristic::ContainsSql(v) => v.span(),
            FunctionCharacteristic::NoSql(v) => v.span(),
            FunctionCharacteristic::ReadsSqlData(v) => v.span(),
            FunctionCharacteristic::ModifiesSqlData(v) => v.span(),
            FunctionCharacteristic::SqlSecurityDefiner(v) => v.span(),
            FunctionCharacteristic::SqlSecurityUser(v) => v.span(),
            FunctionCharacteristic::Comment(v) => v.span(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateFunction<'a> {
    pub create_span: Span,
    pub create_options: Vec<CreateOption<'a>>,
    pub function_span: Span,
    pub if_not_exists: Option<Span>,
    pub name: Identifier<'a>,
    pub params: Vec<(Identifier<'a>, DataType<'a>)>,
    pub returns_span: Span,
    pub return_type: DataType<'a>,
    pub characteristics: Vec<FunctionCharacteristic<'a>>,
    pub return_span: Span,
    pub return_: Box<Statement<'a>>,
}

impl<'a> Spanned for CreateFunction<'a> {
    fn span(&self) -> Span {
        self.create_span
            .join_span(&self.create_options)
            .join_span(&self.function_span)
            .join_span(&self.if_not_exists)
            .join_span(&self.name)
            .join_span(&self.return_type)
            .join_span(&self.characteristics)
            .join_span(&self.return_span)
            .join_span(&self.return_)
    }
}

fn parse_create_function<'a, 'b>(
    parser: &mut Parser<'a, 'b>,
    create_span: Span,
    create_options: Vec<CreateOption<'a>>,
) -> Result<Statement<'a>, ParseError> {
    let function_span = parser.consume_keyword(Keyword::FUNCTION)?;

    let if_not_exists = if let Some(if_) = parser.skip_keyword(Keyword::IF) {
        Some(
            parser
                .consume_keywords(&[Keyword::NOT, Keyword::EXISTS])?
                .join_span(&if_),
        )
    } else {
        None
    };

    let name = parser.consume_plain_identifier()?;
    let mut params = Vec::new();
    parser.consume_token(Token::LParen)?;
    parser.recovered("')'", &|t| t == &Token::RParen, |parser| {
        loop {
            let name = parser.consume_plain_identifier()?;
            let type_ = parse_data_type(parser)?;
            params.push((name, type_));
            if parser.skip_token(Token::Comma).is_none() {
                break;
            }
        }
        Ok(())
    })?;
    parser.consume_token(Token::RParen)?;
    let returns_span = parser.consume_keyword(Keyword::RETURNS)?;
    let return_type = parse_data_type(parser)?;
    let mut characteristics = Vec::new();
    loop {
        let f = match &parser.token {
            Token::Ident(_, Keyword::LANGUAGE) => FunctionCharacteristic::LanguageSql(
                parser.consume_keywords(&[Keyword::LANGUAGE, Keyword::SQL])?,
            ),
            Token::Ident(_, Keyword::NOT) => FunctionCharacteristic::NotDeterministic(
                parser.consume_keywords(&[Keyword::NOT, Keyword::DETERMINISTIC])?,
            ),
            Token::Ident(_, Keyword::DETERMINISTIC) => FunctionCharacteristic::Deterministic(
                parser.consume_keyword(Keyword::DETERMINISTIC)?,
            ),
            Token::Ident(_, Keyword::CONTAINS) => FunctionCharacteristic::ContainsSql(
                parser.consume_keywords(&[Keyword::CONTAINS, Keyword::SQL])?,
            ),
            Token::Ident(_, Keyword::NO) => FunctionCharacteristic::NoSql(
                parser.consume_keywords(&[Keyword::NO, Keyword::SQL])?,
            ),
            Token::Ident(_, Keyword::READS) => {
                FunctionCharacteristic::ReadsSqlData(parser.consume_keywords(&[
                    Keyword::READS,
                    Keyword::SQL,
                    Keyword::DATA,
                ])?)
            }
            Token::Ident(_, Keyword::MODIFIES) => {
                FunctionCharacteristic::ModifiesSqlData(parser.consume_keywords(&[
                    Keyword::MODIFIES,
                    Keyword::SQL,
                    Keyword::DATA,
                ])?)
            }
            Token::Ident(_, Keyword::COMMENT) => {
                parser.consume_keyword(Keyword::COMMENT)?;
                FunctionCharacteristic::Comment(parser.consume_string()?)
            }
            Token::Ident(_, Keyword::SQL) => {
                let span = parser.consume_keywords(&[Keyword::SQL, Keyword::SECURITY])?;
                match &parser.token {
                    Token::Ident(_, Keyword::DEFINER) => {
                        FunctionCharacteristic::SqlSecurityDefiner(
                            parser.consume_keyword(Keyword::DEFINER)?.join_span(&span),
                        )
                    }
                    Token::Ident(_, Keyword::USER) => FunctionCharacteristic::SqlSecurityUser(
                        parser.consume_keyword(Keyword::USER)?.join_span(&span),
                    ),
                    _ => parser.expected_failure("'DEFINER' or 'USER'")?,
                }
            }
            _ => break,
        };
        characteristics.push(f);
    }

    let return_span = parser.consume_keyword(Keyword::RETURN)?;
    let return_ = match parse_statement(parser)? {
        Some(v) => Box::new(v),
        None => parser.expected_failure("statement")?,
    };

    Ok(Statement::CreateFunction(CreateFunction {
        create_span,
        create_options,
        function_span,
        if_not_exists,
        name,
        params,
        returns_span,
        return_type,
        characteristics,
        return_span,
        return_,
    }))
}

#[derive(Clone, Debug)]

pub enum TriggerTime {
    Before(Span),
    After(Span),
}

impl Spanned for TriggerTime {
    fn span(&self) -> Span {
        match &self {
            TriggerTime::Before(v) => v.span(),
            TriggerTime::After(v) => v.span(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TriggerEvent {
    Update(Span),
    Insert(Span),
    Delete(Span),
}

impl Spanned for TriggerEvent {
    fn span(&self) -> Span {
        match &self {
            TriggerEvent::Update(v) => v.span(),
            TriggerEvent::Insert(v) => v.span(),
            TriggerEvent::Delete(v) => v.span(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateTrigger<'a> {
    pub create_span: Span,
    pub create_options: Vec<CreateOption<'a>>,
    pub trigger_span: Span,
    pub if_not_exists: Option<Span>,
    pub name: Identifier<'a>,
    pub trigger_time: TriggerTime,
    pub trigger_event: TriggerEvent,
    pub on_span: Span,
    pub table: Identifier<'a>,
    pub for_each_row_span: Span,
    pub statement: Box<Statement<'a>>,
}

impl<'a> Spanned for CreateTrigger<'a> {
    fn span(&self) -> Span {
        self.create_span
            .join_span(&self.create_options)
            .join_span(&self.trigger_span)
            .join_span(&self.if_not_exists)
            .join_span(&self.name)
            .join_span(&self.trigger_time)
            .join_span(&self.trigger_event)
            .join_span(&self.on_span)
            .join_span(&self.table)
            .join_span(&self.for_each_row_span)
            .join_span(&self.statement)
    }
}

fn parse_create_trigger<'a, 'b>(
    parser: &mut Parser<'a, 'b>,
    create_span: Span,
    create_options: Vec<CreateOption<'a>>,
) -> Result<Statement<'a>, ParseError> {
    let trigger_span = parser.consume_keyword(Keyword::TRIGGER)?;

    let if_not_exists = if let Some(if_) = parser.skip_keyword(Keyword::IF) {
        Some(
            parser
                .consume_keywords(&[Keyword::NOT, Keyword::EXISTS])?
                .join_span(&if_),
        )
    } else {
        None
    };

    let name = parser.consume_plain_identifier()?;

    let trigger_time = match &parser.token {
        Token::Ident(_, Keyword::AFTER) => {
            TriggerTime::After(parser.consume_keyword(Keyword::AFTER)?)
        }
        Token::Ident(_, Keyword::BEFORE) => {
            TriggerTime::Before(parser.consume_keyword(Keyword::BEFORE)?)
        }
        _ => parser.expected_failure("'BEFORE' or 'AFTER'")?,
    };

    let trigger_event = match &parser.token {
        Token::Ident(_, Keyword::UPDATE) => {
            TriggerEvent::Update(parser.consume_keyword(Keyword::UPDATE)?)
        }
        Token::Ident(_, Keyword::INSERT) => {
            TriggerEvent::Insert(parser.consume_keyword(Keyword::INSERT)?)
        }
        Token::Ident(_, Keyword::DELETE) => {
            TriggerEvent::Delete(parser.consume_keyword(Keyword::DELETE)?)
        }
        _ => parser.expected_failure("'UPDATE' or 'INSERT' or 'DELETE'")?,
    };

    let on_span = parser.consume_keyword(Keyword::ON)?;

    let table = parser.consume_plain_identifier()?;

    let for_each_row_span =
        parser.consume_keywords(&[Keyword::FOR, Keyword::EACH, Keyword::ROW])?;

    // TODO [{ FOLLOWS | PRECEDES } other_trigger_name ]

    let statement = match parse_statement(parser)? {
        Some(v) => v,
        None => parser.expected_failure("statement")?,
    };

    Ok(Statement::CreateTrigger(CreateTrigger {
        create_span,
        create_options,
        trigger_span,
        if_not_exists,
        name,
        trigger_time,
        trigger_event,
        on_span,
        table,
        for_each_row_span,
        statement: Box::new(statement),
    }))
}

fn parse_create_table<'a, 'b>(
    parser: &mut Parser<'a, 'b>,
    create_span: Span,
    create_options: Vec<CreateOption<'a>>,
) -> Result<Statement<'a>, ParseError> {
    let table_span = parser.consume_keyword(Keyword::TABLE)?;

    let mut identifier = Identifier::new("", 0..0);
    let mut if_not_exists = None;

    parser.recovered("'('", &|t| t == &Token::LParen, |parser| {
        if let Some(if_) = parser.skip_keyword(Keyword::IF) {
            if_not_exists = Some(
                if_.start
                    ..parser
                        .consume_keywords(&[Keyword::NOT, Keyword::EXISTS])?
                        .end,
            );
        }
        identifier = parser.consume_plain_identifier()?;
        Ok(())
    })?;

    parser.consume_token(Token::LParen)?;

    let mut create_definitions = Vec::new();
    loop {
        parser.recovered(
            "')' or ','",
            &|t| matches!(t, Token::RParen | Token::Comma),
            |parser| {
                create_definitions.push(parse_create_definition(parser)?);
                Ok(())
            },
        )?;
        if matches!(parser.token, Token::RParen) {
            break;
        }
        parser.consume_token(Token::Comma)?;
    }
    parser.consume_token(Token::RParen)?;

    let mut options = Vec::new();
    let delimiter = parser.delimiter.clone();
    parser.recovered(
        delimiter.name(),
        &|t| t == &Token::Eof || t == &delimiter,
        |parser| {
            loop {
                let identifier = parser.span.clone();
                match &parser.token {
                    Token::Ident(_, Keyword::ENGINE) => {
                        parser.consume_keyword(Keyword::ENGINE)?;
                        parser.skip_token(Token::Eq);
                        options.push(TableOption::Engine {
                            identifier,
                            value: parser.consume_plain_identifier()?,
                        });
                    }
                    Token::Ident(_, Keyword::DEFAULT) => {
                        parser.consume_keyword(Keyword::DEFAULT)?;
                        match &parser.token {
                            Token::Ident(_, Keyword::CHARSET) => {
                                parser.consume_keyword(Keyword::CHARSET)?;
                                parser.skip_token(Token::Eq);
                                options.push(TableOption::DefaultCharSet {
                                    identifier,
                                    value: parser.consume_plain_identifier()?,
                                });
                            }
                            Token::Ident(_, Keyword::COLLATE) => {
                                parser.consume_keyword(Keyword::COLLATE)?;
                                parser.skip_token(Token::Eq);
                                options.push(TableOption::DefaultCollate {
                                    identifier,
                                    value: parser.consume_plain_identifier()?,
                                });
                            }
                            _ => parser.expected_failure("'CHARSET' or 'COLLATE'")?,
                        }
                    }
                    Token::Ident(_, Keyword::CHARSET) => {
                        parser.consume_keyword(Keyword::CHARSET)?;
                        parser.skip_token(Token::Eq);
                        options.push(TableOption::CharSet {
                            identifier,
                            value: parser.consume_plain_identifier()?,
                        });
                    }
                    Token::Ident(_, Keyword::COLLATE) => {
                        parser.consume_keyword(Keyword::COLLATE)?;
                        parser.skip_token(Token::Eq);
                        options.push(TableOption::Collate {
                            identifier,
                            value: parser.consume_plain_identifier()?,
                        });
                    }
                    Token::Ident(_, Keyword::ROW_FORMAT) => {
                        parser.consume_keyword(Keyword::ROW_FORMAT)?;
                        parser.skip_token(Token::Eq);
                        options.push(TableOption::RowFormat {
                            identifier,
                            value: parser.consume_plain_identifier()?,
                        });
                        //TODO validate raw format is in the keyword set
                    }
                    Token::Ident(_, Keyword::COMMENT) => {
                        parser.consume_keyword(Keyword::COMMENT)?;
                        parser.skip_token(Token::Eq);
                        options.push(TableOption::Comment {
                            identifier,
                            value: parser.consume_string()?,
                        });
                    }
                    t if t == &parser.delimiter => break,
                    Token::Eof => break,
                    _ => {
                        parser.expected_failure("table option or delimiter")?;
                    }
                }
            }
            Ok(())
        },
    )?;

    Ok(Statement::CreateTable(CreateTable {
        create_span,
        create_options,
        table_span,
        identifier,
        if_not_exists,
        options,
        create_definitions,
    }))
}

pub(crate) fn parse_create<'a, 'b>(
    parser: &mut Parser<'a, 'b>,
) -> Result<Statement<'a>, ParseError> {
    let create_span = parser.span.clone();
    parser.consume_keyword(Keyword::CREATE)?;

    let mut create_options = Vec::new();
    const CREATABLE: &str = "'TABLE' | 'VIEW' | 'TRIGGER' | 'FUNCTION'";

    parser.recovered(
        CREATABLE,
        &|t| {
            matches!(
                t,
                Token::Ident(
                    _,
                    Keyword::TABLE | Keyword::VIEW | Keyword::TRIGGER | Keyword::FUNCTION
                )
            )
        },
        |parser| {
            loop {
                let v = match &parser.token {
                    Token::Ident(_, Keyword::OR) => CreateOption::OrReplace(
                        parser.consume_keywords(&[Keyword::OR, Keyword::REPLACE])?,
                    ),
                    Token::Ident(_, Keyword::TEMPORARY) => {
                        CreateOption::Temporary(parser.consume_keyword(Keyword::TEMPORARY)?)
                    }
                    Token::Ident(_, Keyword::ALGORITHM) => {
                        let algorithm_span = parser.consume_keyword(Keyword::ALGORITHM)?;
                        parser.consume_token(Token::Eq)?;
                        let algorithm = match &parser.token {
                            Token::Ident(_, Keyword::UNDEFINED) => CreateAlgorithm::Undefined(
                                parser.consume_keyword(Keyword::UNDEFINED)?,
                            ),
                            Token::Ident(_, Keyword::MERGE) => {
                                CreateAlgorithm::Merge(parser.consume_keyword(Keyword::MERGE)?)
                            }
                            Token::Ident(_, Keyword::TEMPTABLE) => CreateAlgorithm::TempTable(
                                parser.consume_keyword(Keyword::TEMPTABLE)?,
                            ),
                            _ => parser.expected_failure("'UNDEFINED', 'MERGE' or 'TEMPTABLE'")?,
                        };
                        CreateOption::Algorithm(algorithm_span, algorithm)
                    }
                    Token::Ident(_, Keyword::DEFINER) => {
                        let definer_span = parser.consume_keyword(Keyword::DEFINER)?;
                        parser.consume_token(Token::Eq)?;
                        // TODO user | CURRENT_USER | role | CURRENT_ROLE
                        let user = parser.consume_plain_identifier()?;
                        parser.consume_token(Token::At)?;
                        let host = parser.consume_plain_identifier()?;
                        CreateOption::Definer {
                            definer_span,
                            user,
                            host,
                        }
                    }
                    Token::Ident(_, Keyword::SQL) => {
                        let sql_security =
                            parser.consume_keywords(&[Keyword::SQL, Keyword::SECURITY])?;
                        match &parser.token {
                            Token::Ident(_, Keyword::DEFINER) => CreateOption::SqlSecurityDefiner(
                                sql_security,
                                parser.consume_keyword(Keyword::DEFINER)?,
                            ),
                            Token::Ident(_, Keyword::USER) => CreateOption::SqlSecurityUser(
                                sql_security,
                                parser.consume_keyword(Keyword::USER)?,
                            ),
                            _ => parser.expected_failure("'DEFINER', 'USER'")?,
                        }
                    }
                    _ => break,
                };
                create_options.push(v);
            }
            Ok(())
        },
    )?;

    match &parser.token {
        Token::Ident(_, Keyword::TABLE) => parse_create_table(parser, create_span, create_options),
        Token::Ident(_, Keyword::VIEW) => parse_create_view(parser, create_span, create_options),
        Token::Ident(_, Keyword::FUNCTION) => {
            parse_create_function(parser, create_span, create_options)
        }
        Token::Ident(_, Keyword::TRIGGER) => {
            parse_create_trigger(parser, create_span, create_options)
        }
        _ => parser.expected_failure(CREATABLE),
    }
}
