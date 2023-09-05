#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("Entry could not be parsed.")]
    EntryCouldNotBeParsed(String),

    #[error("Entry could not be found.")]
    EntryCouldNotBeFound(String),

    #[error("Entry has wrong length")]
    EntryHasWrongLength(String),

    #[error("Entry id could not be parsed.")]
    EntryIdCouldNotBeParsed(String),

    #[error("File is empty.")]
    FileEmpty(String),

    #[error("File could not be written to.")]
    FileCouldNotBeWrittenTo(String),

    #[error("File could not be set up for processing.")]
    FileCouldNotBeSetUpForProcessing(String),

    #[error("File could not be read.")]
    FileCouldNotRead(String),

    #[error("Incorrect table name.")]
    IncorrectTableName(String),
}
