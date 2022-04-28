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

macro_rules! keywords {
    [$(
        $ident:ident
    )*] => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[allow(non_camel_case_types, clippy::upper_case_acronyms)]
        pub enum Keyword {
            NOT_A_KEYWORD,
            QUOTED_IDENTIFIER,
            $($ident),*
        }

        impl From<&str> for Keyword {
            fn from(v: &str) -> Self {
                match v {
                    $(stringify!($ident) => Keyword::$ident),*,
                    _ => Keyword::NOT_A_KEYWORD
                }
            }
        }

        impl Keyword {
            pub fn name(&self) -> &'static str {
                match self {
                    $(Keyword::$ident => stringify!($ident)),*,
                    Keyword::NOT_A_KEYWORD => "NOT_A_KEYWORD",
                    Keyword::QUOTED_IDENTIFIER => "QUOTED_IDENTIFIER",
                }
            }
        }
    };
}

macro_rules! reserved {
    [$(
        $ident:ident
    )*] => {
        impl Keyword {
            pub const fn reserved(&self) -> bool {
                match self {
                    $(Keyword::$ident => true),*,
                    _ => false
                }
            }
        }
    };
}

macro_rules! expr_ident {
    [$(
        $ident:ident
    )*] => {
        impl Keyword {
            pub const fn expr_ident(&self) -> bool {
                match self {
                    $(Keyword::$ident => true),*,
                    _ => !self.reserved()
                }
            }
        }
    };
}

impl Default for Keyword {
    fn default() -> Self {
        Keyword::NOT_A_KEYWORD
    }
}

keywords![
ABS
ACCESSIBLE
ACCOUNT
ACOS
ACTION
ADD
ADD_MONTHS
ADDDATE
ADDTIME
ADMIN
AFTER
AGAINST
AGGREGATE
ALGORITHM
ALL
ALTER
ALWAYS
ANALYZE
AND
ANY
AS
ASC
ASCII
ASENSITIVE
ASIN
AT
ATAN
ATAN2
ATOMIC
AUTHORS
AUTO
AUTO_INCREMENT
AUTOEXTEND_SIZE
AVG
AVG_ROW_LENGTH
BACKUP
BEFORE
BEGIN
BETWEEN
BIGINT
BIN
BINARY
BINLOG
BIT
BIT_LENGTH
BLOB
BLOCK
BODY
BOOL
BOOLEAN
BOTH
BTREE
BY
BYTE
CACHE
CALL
CASCADE
CASCADED
CASE
CAST
CATALOG_NAME
CEIL
CEILING
CHAIN
CHANGE
CHANGED
CHAR
CHAR_LENGTH
CHARACTER
CHARACTER_LENGTH
CHARSET
CHECK
CHECKPOINT
CHECKSUM
CHR
CIPHER
CLASS_ORIGIN
CLIENT
CLOB
CLOSE
COALESCE
CODE
COLLATE
COLLATION
COLUMN
COLUMN_ADD
COLUMN_CHECK
COLUMN_CREATE
COLUMN_DELETE
COLUMN_GET
COLUMN_NAME
COLUMNS
COMMENT
COMMIT
COMMITTED
COMPACT
COMPLETION
COMPRESSED
CONCAT
CONCAT_WS
CONCURRENT
CONDITION
CONNECTION
CONSISTENT
CONSTRAINT
CONSTRAINT_CATALOG
CONSTRAINT_NAME
CONSTRAINT_SCHEMA
CONTAINS
CONTEXT
CONTINUE
CONTRIBUTORS
CONV
CONVERT
CONVERT_TS
COPY
COS
COT
COUNT
CPU
CRC32
CRC32C
CREATE
CROSS
CUBE
CURDATE
CURRENT
CURRENT_DATE
CURRENT_POS
CURRENT_ROLE
CURRENT_TIME
CURRENT_TIMESTAMP
CURRENT_USER
CURSOR
CURSOR_NAME
CURTIME
CYCLE
DATA
DATABASE
DATABASES
DATAFILE
DATE
DATE_ADD
DATE_FORMAT
DATE_SUB
DATEDIFF
DATETIME
DAY
DAY_HOUR
DAY_MICROSECOND
DAY_MINUTE
DAY_SECOND
DAYNAME
DAYOFMONTH
DAYOFWEEK
DAYOFYEAR
DEALLOCATE
DEC
DECIMAL
DECLARE
DEFAULT
DEFINER
DEGREES
DELAY_KEY_WRITE
DELAYED
DELETE
DELETE_DOMAIN_ID
DELIMITER
DES_KEY_FILE
DESC
DESCRIBE
DETERMINISTIC
DIAGNOSTICS
DIRECTORY
DISABLE
DISCARD
DISK
DISTINCT
DISTINCTROW
DIV
DO
DO_DOMAIN_IDS
DOUBLE
DROP
DUAL
DUMPFILE
DUPLICATE
DYNAMIC
EACH
ELSE
ELSEIF
ELSIF
ELT
EMPTY
ENABLE
ENCLOSED
END
ENDS
ENGINE
ENGINES
ENUM
ERROR
ERRORS
ESCAPE
ESCAPED
EVENT
EVENTS
EVERY
EXAMINED
EXCEPT
EXCEPTION
EXCHANGE
EXCLUDE
EXECUTE
EXISTS
EXIT
EXP
EXPANSION
EXPIRE
EXPLAIN
EXPORT
EXPORT_SET
EXTENDED
EXTENT_SIZE
EXTRACTVALUE
FALSE
FAST
FAULTS
FEDERATED
FETCH
FIELD
FIELDS
FILE
FIND_IN_SET
FIRST
FIXED
FLOAT
FLOAT4
FLOAT8
FLOOR
FLUSH
FOLLOWING
FOLLOWS
FOR
FORCE
FOREIGN
FORMAT
FOUND
FROM
FROM_BASE64
FROM_DAYS
FROM_UNIXTIME
FULL
FULLTEXT
FUNCTION
GENERAL
GENERATED
GET
GET_FORMAT
GIST
GLOBAL
GOTO
GRANT
GRANTS
GREATEST
GROUP
HANDLER
HARD
HASH
HAVING
HELP
HEX
HIGH_PRIORITY
HISTORY
HOST
HOSTS
HOUR
HOUR_MICROSECOND
HOUR_MINUTE
HOUR_SECOND
ID
IDENTIFIED
IF
IFNULL
IGNORE
IGNORE_DOMAIN_IDS
IGNORE_SERVER_IDS
IGNORED
IMMEDIATE
IMPORT
IN
INCREMENT
INDEX
INDEXES
INFILE
INITIAL_SIZE
INNER
INOUT
INSENSITIVE
INSERT
INSERT_METHOD
INSTALL
INSTR
INT
INT1
INT2
INT3
INT4
INT8
INTEGER
INTERSECT
INTERSECTA
INTERVAL
INTO
INVISIBLE
INVOKER
IO
IO_THREAD
IPC
IS
ISOLATION
ISOPEN
ISSUER
ITERATE
JOIN
JSON
JSON_ARRAY
JSON_ARRAY_APPEND
JSON_ARRAY_INSERT
JSON_ARRAYAGG
JSON_COMPACT
JSON_CONTAINS
JSON_CONTAINS_PATH
JSON_DEPTH
JSON_DETAILED
JSON_EQUALS
JSON_EXISTS
JSON_EXTRACT
JSON_INSERT
JSON_KEYS
JSON_LENGTH
JSON_LOOSE
JSON_MERGE
JSON_MERGE_PATCH
JSON_MERGE_PRESERVE
JSON_NORMALIZE
JSON_OBJECT
JSON_OBJECTAGG
JSON_QUERY
JSON_QUOTE
JSON_REMOVE
JSON_REPLACE
JSON_SEARCH
JSON_SET
JSON_TABLE
JSON_TYPE
JSON_UNQUOTE
JSON_VALID
JSON_VALUE
KEY
KEY_BLOCK_SIZE
KEYS
KILL
LANGUAGE
LAST
LAST_VALUE
LASTVAL
LCASE
LEADING
LEAST
LEAVE
LEAVES
LEFT
LENGTH
LENGTHB
LESS
LEVEL
LIKE
LIMIT
LINEAR
LINES
LIST
LN
LOAD
LOAD_FILE
LOCAL
LOCALTIME
LOCALTIMESTAMP
LOCATE
LOCK
LOCKED
LOCKS
LOG
LOG10
LOG2
LOGFILE
LOGS
LONG
LONGBLOB
LONGTEXT
LOOP
LOW_PRIORITY
LOWER
LPAD
LTRIM
MAKE_SET
MAKEDATE
MAKETIME
MASTER
MASTER_CONNECT_RETRY
MASTER_DELAY
MASTER_GTID_POS
MASTER_HEARTBEAT_PERIOD
MASTER_HOST
MASTER_LOG_FILE
MASTER_LOG_POS
MASTER_PASSWORD
MASTER_PORT
MASTER_SERVER_ID
MASTER_SSL
MASTER_SSL_CA
MASTER_SSL_CAPATH
MASTER_SSL_CERT
MASTER_SSL_CIPHER
MASTER_SSL_CRL
MASTER_SSL_CRLPATH
MASTER_SSL_KEY
MASTER_SSL_VERIFY_SERVER_CERT
MASTER_USE_GTID
MASTER_USER
MATCH
MAX
MAX_CONNECTIONS_PER_HOUR
MAX_QUERIES_PER_HOUR
MAX_ROWS
MAX_SIZE
MAX_STATEMENT_TIME
MAX_UPDATES_PER_HOUR
MAX_USER_CONNECTIONS
MAXVALUE
MEDIUM
MEDIUMBLOB
MEDIUMINT
MEDIUMTEXT
MEMORY
MERGE
MESSAGE_TEXT
MICROSECOND
MID
MIDDLEINT
MIGRATE
MIN
MIN_ROWS
MINUS
MINUTE
MINUTE_MICROSECOND
MINUTE_SECOND
MINVALUE
MOD
MODE
MODIFIES
MODIFY
MONITOR
MONTH
MONTHNAME
MUTEX
MYSQL
MYSQL_ERRNO
NAME
NAMES
NATIONAL
NATURAL
NATURAL_SORT_KEY
NCHAR
NESTED
NEVER
NEW
NEXT
NEXTVAL
NO
NO_WAIT
NO_WRITE_TO_BINLOG
NOCACHE
NOCYCLE
NODEGROUP
NOMAXVALUE
NOMINVALUE
NONE
NOT
NOTFOUND
NOW
NOWAIT
NULL
NULLIF
NUMBER
NUMERIC
NVARCHAR
NVL
NVL2
OCT
OCTET_LENGTH
OF
OFFSET
OLD_PASSWORD
ON
ONE
ONLINE
ONLY
OPEN
OPTIMIZE
OPTION
OPTIONALLY
OPTIONS
OR
ORD
ORDER
ORDINALITY
OTHERS
OUT
OUTER
OUTFILE
OVER
OVERLAPS
OWNER
PACK_KEYS
PACKAGE
PAGE
PAGE_CHECKSUM
PARSE_VCOL_EXPR
PARSER
PARTIAL
PARTITION
PARTITIONING
PARTITIONS
PASSWORD
PATH
PERIOD
PERIOD_ADD
PERIOD_DIFF
PERSISTENT
PHASE
PI
PLUGIN
PLUGINS
PORT
PORTION
POSITION
POW
POWER
PRECEDES
PRECEDING
PRECISION
PREPARE
PRESERVE
PREV
PREVIOUS
PRIMARY
PRIVILEGES
PROCEDURE
PROCESS
PROCESSLIST
PROFILE
PROFILES
PROXY
PURGE
QUARTER
QUERY
QUICK
QUOTE
RADIANS
RAISE
RAND
RANGE
RAW
READ
READ_ONLY
READ_WRITE
READS
REAL
REBUILD
RECOVER
RECURSIVE
REDO_BUFFER_SIZE
REDOFILE
REDUNDANT
REF_SYSTEM_ID
REFERENCES
REGEXP
RELAY
RELAY_LOG_FILE
RELAY_LOG_POS
RELAY_THREAD
RELAYLOG
RELEASE
RELOAD
REMOVE
RENAME
REORGANIZE
REPAIR
REPEAT
REPEATABLE
REPLACE
REPLAY
REPLICA
REPLICA_POS
REPLICAS
REPLICATION
REQUIRE
RESET
RESIGNAL
RESTART
RESTORE
RESTRICT
RESUME
RETURN
RETURNED_SQLSTATE
RETURNING
RETURNS
REUSE
REVERSE
REVOKE
RIGHT
RLIKE
ROLE
ROLLBACK
ROLLUP
ROUND
ROUTINE
ROW
ROW_COUNT
ROW_FORMAT
ROWCOUNT
ROWNUM
ROWS
ROWTYPE
RPAD
RTREE
RTRIM
SAVEPOINT
SCHEDULE
SCHEMA
SCHEMA_NAME
SCHEMAS
SEC_TO_TIME
SECOND
SECOND_MICROSECOND
SECURITY
SELECT
SENSITIVE
SEPARATOR
SEQUENCE
SERIAL
SERIALIZABLE
SERVER
SESSION
SET
SETVAL
SFORMAT
SHARE
SHOW
SHUTDOWN
SIGN
SIGNAL
SIGNED
SIMPLE
SIN
SKIP
SLAVE
SLAVE_POS
SLAVES
SLOW
SMALLINT
SNAPSHOT
SOCKET
SOFT
SOME
SONAME
SOUNDEX
SOUNDS
SOURCE
SPACE
SPATIAL
SPECIFIC
SQL
SQL_BIG_RESULT
SQL_BUFFER_RESULT
SQL_CACHE
SQL_CALC_FOUND_ROWS
SQL_NO_CACHE
SQL_SMALL_RESULT
SQL_THREAD
SQL_TSI_DAY
SQL_TSI_HOUR
SQL_TSI_MINUTE
SQL_TSI_MONTH
SQL_TSI_QUARTER
SQL_TSI_SECOND
SQL_TSI_WEEK
SQL_TSI_YEAR
SQLEXCEPTION
SQLSTATE
SQLWARNING
SQRT
SSL
STAGE
START
STARTING
STARTS
STATEMENT
STATS_AUTO_RECALC
STATS_PERSISTENT
STATS_SAMPLE_PAGES
STATUS
STDIN
STOP
STORAGE
STORED
STR_TO_DATE
STRAIGHT_JOIN
STRCMP
STRING
SUBCLASS_ORIGIN
SUBDATE
SUBJECT
SUBPARTITION
SUBPARTITIONS
SUBSTR
SUBSTRING
SUBSTRING_INDEX
SUBTIME
SUM
SUPER
SUSPEND
SWAPS
SWITCHES
SYSDATE
SYSTEM
SYSTEM_TIME
TABLE
TABLE_CHECKSUM
TABLE_NAME
TABLES
TABLESPACE
TAN
TEMPORARY
TEMPTABLE
TERMINATED
TEXT
THAN
THEN
THREADS
TIES
TIME
TIME_FORMAT
TIME_TO_SEC
TIMEDIFF
TIMESTAMP
TIMESTAMPADD
TIMESTAMPDIFF
TINYBLOB
TINYINT
TINYTEXT
TO
TO_BASE64
TO_CHAR
TO_DAYS
TO_SECONDS
TRAILING
TRANSACTION
TRANSACTIONAL
TRIGGER
TRIGGERS
TRUE
TRUNCATE
TYPE
TYPES
UCASE
UNBOUNDED
UNCOMMITTED
UNCOMPRESSED_LENGTH
UNDEFINED
UNDO
UNDO_BUFFER_SIZE
UNDOFILE
UNHEX
UNICODE
UNINSTALL
UNION
UNIQUE
UNIX_TIMESTAMP
UNKNOWN
UNLOCK
UNSIGNED
UNTIL
UPDATE
UPDATEXML
UPGRADE
UPPER
USAGE
USE
USE_FRM
USER
USER_RESOURCES
USING
UTC_DATE
UTC_TIME
UTC_TIMESTAMP
VALUE
VALUES
VARBINARY
VARCHAR
VARCHAR2
VARCHARACTER
VARIABLES
VARYING
VERSIONING
VIA
VIEW
VIRTUAL
VISIBLE
WAIT
WARNINGS
WEEK
WEEKDAY
WEEKOFYEAR
WEIGHT_STRING
WHEN
WHERE
WHILE
WINDOW
WITH
WITHIN
WITHOUT
WORK
WRAPPER
WRITE
X509
XA
XML
XOR
YEAR
YEAR_MONTH
ZEROFILL
ZONE
];

reserved![ACCESSIBLE
ADD
ALL
ALTER
ANALYZE
AND
AS
ASC
ASENSITIVE
BEFORE
BETWEEN
BIGINT
BINARY
BLOB
BOTH
BY
CALL
CASCADE
CASE
CHANGE
CHAR
CHARACTER
CHECK
COLLATE
COLUMN
COMMENT
CONDITION
CONSTRAINT
CONTINUE
CONVERT
CREATE
CROSS
CURRENT_DATE
CURRENT_ROLE
CURRENT_TIME
CURRENT_TIMESTAMP
CURRENT_USER
CURSOR
DATABASE
DATABASES
DAY_HOUR
DAY_MICROSECOND
DAY_MINUTE
DAY_SECOND
DEC
DECIMAL
DECLARE
DEFAULT
DELAYED
DELETE
DELETE_DOMAIN_ID
DESC
DESCRIBE
DETERMINISTIC
DISTINCT
DISTINCTROW
DIV
DO_DOMAIN_IDS
DOUBLE
DROP
DUAL
EACH
ELSE
ELSEIF
ENCLOSED
ESCAPED
EXCEPT
EXISTS
EXIT
EXPLAIN
FALSE
FETCH
FLOAT
FLOAT4
FLOAT8
FOR
FORCE
FOREIGN
FROM
FULLTEXT
GENERAL
GRANT
GROUP
HAVING
HIGH_PRIORITY
HOUR_MICROSECOND
HOUR_MINUTE
HOUR_SECOND
IF
IGNORE
IGNORE_DOMAIN_IDS
IGNORE_SERVER_IDS
IN
INDEX
INFILE
INNER
INOUT
INSENSITIVE
INSERT
INT
INT1
INT2
INT3
INT4
INT8
INTEGER
INTERSECTA
INTERVAL
INTO
IS
ITERATE
JOIN
KEY
KEYS
KILL
LEADING
LEAVE
LEFT
LIKE
LIMIT
LINEAR
LINES
LOAD
LOCALTIME
LOCALTIMESTAMP
LOCK
LONG
LONGBLOB
LONGTEXT
LOOP
LOW_PRIORITY
MASTER_HEARTBEAT_PERIOD
MASTER_SSL_VERIFY_SERVER_CERT
MATCH
MAXVALUE
MEDIUMBLOB
MEDIUMINT
MEDIUMTEXT
MIDDLEINT
MINUTE_MICROSECOND
MINUTE_SECOND
MOD
MODIFIES
NATURAL
NO_WRITE_TO_BINLOG
NOT
NULL
NUMERIC
OFFSET
ON
OPTIMIZE
OPTION
OPTIONALLY
OR
ORDER
OUT
OUTER
OUTFILE
OVER
PAGE_CHECKSUM
PARSE_VCOL_EXPR
PARTITION
POSITION
PRECISION
PRIMARY
PROCEDURE
PURGE
RANGE
READ
READ_WRITE
READS
REAL
RECURSIVE
REF_SYSTEM_ID
REFERENCES
REGEXP
RENAME
REPEAT
REPLACE
REQUIRE
RESIGNAL
RESTRICT
RETURN
RETURNING
REVOKE
RIGHT
RLIKE
ROWS
SCHEMA
SCHEMAS
SECOND_MICROSECOND
SELECT
SENSITIVE
SEPARATOR
SET
SHOW
SIGNAL
SLOW
SMALLINT
SPATIAL
SPECIFIC
SQL
SQL_BIG_RESULT
SQL_CALC_FOUND_ROWS
SQL_SMALL_RESULT
SQLEXCEPTION
SQLSTATE
SQLWARNING
SSL
STARTING
STATS_AUTO_RECALC
STATS_PERSISTENT
STATS_SAMPLE_PAGES
STRAIGHT_JOIN
TABLE
TERMINATED
THEN
TINYBLOB
TINYINT
TINYTEXT
TO
TRAILING
TRIGGER
TRUE
UNDO
UNION
UNIQUE
UNLOCK
UNSIGNED
UPDATE
USAGE
USE
USING
UTC_DATE
UTC_TIME
UTC_TIMESTAMP
VALUES
VARBINARY
VARCHAR
VARCHARACTER
VARYING
WHEN
WHERE
WHILE
WINDOW
WITH
WRITE
XOR
YEAR_MONTH
ZEROFILL
END
];

expr_ident![
CURRENT_DATE
CURRENT_TIME
CURRENT_TIMESTAMP
IF
REPLACE
RIGHT
UTC_DATE
UTC_TIME
UTC_TIMESTAMP
VALUES
];
