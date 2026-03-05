use std::{
    ffi::OsStr,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::{
    fs,
    sync::{RwLock, RwLockReadGuard, broadcast},
};

use tracing::info;

use crate::UpdateEvent;
