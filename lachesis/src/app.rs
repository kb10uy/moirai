use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Arguments {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    /// Registers a new bookmark.
    Register {
        /// Parts.
        #[structopt(flatten)]
        parts: BookmarkOptions,
    },

    /// Updates a bookmark by ID.
    Update {
        /// Bookmark ID to update.
        id: i64,

        /// Parts.
        #[structopt(flatten)]
        parts: BookmarkOptions,
    },

    /// Deletes a bookmark by ID.
    Delete {
        /// Bookmark ID to delete.
        id: i64,
    },

    /// Fetches a bookmark by ID.
    Fetch {
        /// Bookmark ID to fetch.
        id: i64,
    },
}

/// Represents partial bookmark data.
#[derive(Debug, StructOpt)]
pub struct BookmarkOptions {
    /// Title.
    #[structopt(short="t", long)]
    title: Option<String>,

    /// Page URL.
    #[structopt(short="u", long)]
    url: Option<String>,

    /// Description in CommonMark.
    #[structopt(short="d", long)]
    description: Option<String>,

    /// Additional tags.
    #[structopt(short="g", long)]
    tag: Vec<String>,
}
