//! Backup (export) and restore (import) of the `~/passgen` tree as `.zip` or `.tar` / `.tar.gz`.

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Archive;
use tar::Builder;
use tar::EntryType;
use thiserror::Error;
use zip::write::SimpleFileOptions;
use zip::write::ZipWriter;
use zip::CompressionMethod;
use zip::ZipArchive;

use crate::path::passgen::passgen_dir;

/// Counts files written vs. skipped during import.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportStats {
    pub files_added: usize,
    pub files_skipped_existing: usize,
}

#[derive(Debug, Error)]
pub enum ImportExportError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("could not determine archive format; use .zip, .tar, or .tar.gz")]
    UnknownFormat,
}

/// Writes a compressed archive of the entire passgen directory (all files under `~/passgen`).
///
/// Format is inferred from `output`'s extension: `.zip` (deflate), plain `.tar`, or `.tar.gz` / `.tgz`.
/// Unknown or missing extension defaults to ZIP.
pub fn export_passgen(output: &Path) -> Result<(), ImportExportError> {
    export_passgen_from(&passgen_dir(), output)
}

/// Same as [`export_passgen`], but reads from `source_dir` (for tests or custom layouts).
pub fn export_passgen_from(source_dir: &Path, output: &Path) -> Result<(), ImportExportError> {
    fs::create_dir_all(source_dir)?;
    let files = collect_files_recursive(source_dir)?;
    match archive_format_for_output(output) {
        OutputFormat::Zip => write_zip(&files, output),
        OutputFormat::Tar => write_tar(&files, output),
        OutputFormat::TarGz => write_tar_gz(&files, output),
    }
}

/// Extracts an archive into `~/passgen`. Existing files are **not** overwritten; only missing paths
/// are created.
pub fn import_passgen(archive: &Path) -> Result<ImportStats, ImportExportError> {
    import_passgen_into(&passgen_dir(), archive)
}

/// Same as [`import_passgen`], but extracts into `dest_dir`.
pub fn import_passgen_into(
    dest_dir: &Path,
    archive: &Path,
) -> Result<ImportStats, ImportExportError> {
    fs::create_dir_all(dest_dir)?;
    let base = fs::canonicalize(dest_dir)?;
    match detect_archive_format(archive)? {
        ArchiveKind::Zip => import_zip(&base, archive),
        ArchiveKind::Tar => {
            let file = File::open(archive)?;
            import_tar(&base, file)
        }
        ArchiveKind::TarGz => {
            let file = File::open(archive)?;
            import_tar(&base, GzDecoder::new(file))
        }
    }
}

#[derive(Clone, Copy)]
enum OutputFormat {
    Zip,
    Tar,
    TarGz,
}

fn archive_format_for_output(path: &Path) -> OutputFormat {
    let name = path.to_string_lossy().to_lowercase();
    if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        return OutputFormat::TarGz;
    }
    if name.ends_with(".tar") {
        return OutputFormat::Tar;
    }
    OutputFormat::Zip
}

#[derive(Clone, Copy)]
enum ArchiveKind {
    Zip,
    Tar,
    TarGz,
}

fn detect_archive_format(path: &Path) -> Result<ArchiveKind, ImportExportError> {
    let name = path.to_string_lossy().to_lowercase();
    if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        return Ok(ArchiveKind::TarGz);
    }
    if name.ends_with(".zip") {
        return Ok(ArchiveKind::Zip);
    }
    if name.ends_with(".tar") {
        return Ok(ArchiveKind::Tar);
    }

    let mut f = File::open(path)?;
    let mut buf = [0u8; 512];
    let n = f.read(&mut buf)?;
    if n >= 2 && buf[0] == 0x1f && buf[1] == 0x8b {
        return Ok(ArchiveKind::TarGz);
    }
    if n >= 4 && buf[0] == 0x50 && buf[1] == 0x4b {
        return Ok(ArchiveKind::Zip);
    }
    if n >= 262 && ustar_magic(&buf[257..263]) {
        return Ok(ArchiveKind::Tar);
    }
    Err(ImportExportError::UnknownFormat)
}

fn ustar_magic(s: &[u8]) -> bool {
    s == b"ustar\0" || s == b"ustar "
}

fn collect_files_recursive(root: &Path) -> io::Result<Vec<(PathBuf, PathBuf)>> {
    let mut out = Vec::new();
    if !root.exists() {
        return Ok(out);
    }
    collect_files_inner(root, root, &mut out)?;
    Ok(out)
}

fn collect_files_inner(
    dir: &Path,
    root: &Path,
    out: &mut Vec<(PathBuf, PathBuf)>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let ft = entry.file_type()?;
        if ft.is_symlink() {
            continue;
        }
        if ft.is_dir() {
            collect_files_inner(&path, root, out)?;
        } else if ft.is_file() {
            let rel = path
                .strip_prefix(root)
                .expect("walk stays under root")
                .to_path_buf();
            out.push((path, rel));
        }
    }
    Ok(())
}

fn rel_to_archive_path(rel: &Path) -> String {
    rel.components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn write_zip(files: &[(PathBuf, PathBuf)], out: &Path) -> Result<(), ImportExportError> {
    let file = File::create(out)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    for (abs, rel) in files {
        let name = rel_to_archive_path(rel);
        zip.start_file(&name, options)?;
        let mut f = File::open(abs)?;
        io::copy(&mut f, &mut zip)?;
    }
    zip.finish()?;
    Ok(())
}

fn write_tar(files: &[(PathBuf, PathBuf)], out: &Path) -> Result<(), ImportExportError> {
    let file = File::create(out)?;
    let mut tar = Builder::new(file);
    for (abs, rel) in files {
        tar.append_path_with_name(abs, rel)?;
    }
    tar.finish()?;
    Ok(())
}

fn write_tar_gz(files: &[(PathBuf, PathBuf)], out: &Path) -> Result<(), ImportExportError> {
    let file = File::create(out)?;
    let enc = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(enc);
    for (abs, rel) in files {
        tar.append_path_with_name(abs, rel)?;
    }
    tar.finish()?;
    let enc = tar.into_inner()?;
    enc.finish()?;
    Ok(())
}

fn import_zip(base: &Path, archive: &Path) -> Result<ImportStats, ImportExportError> {
    let file = File::open(archive)?;
    let mut archive = ZipArchive::new(file)?;
    let mut files_added = 0;
    let mut files_skipped_existing = 0;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if !file.is_file() {
            continue;
        }
        let Some(rel) = file.enclosed_name() else {
            continue;
        };
        let dest = base.join(&rel);
        if !path_is_within_base(base, &dest) {
            continue;
        }
        if dest.exists() {
            files_skipped_existing += 1;
            continue;
        }
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut out = File::create(&dest)?;
        io::copy(&mut file, &mut out)?;
        files_added += 1;
    }
    Ok(ImportStats {
        files_added,
        files_skipped_existing,
    })
}

fn import_tar<R: Read>(base: &Path, reader: R) -> Result<ImportStats, ImportExportError> {
    let mut archive = Archive::new(reader);
    let mut files_added = 0;
    let mut files_skipped_existing = 0;
    for entry in archive.entries()? {
        let mut entry = entry?;
        let t = entry.header().entry_type();
        if t == EntryType::Directory
            || t.is_symlink()
            || t.is_hard_link()
            || !matches!(t, EntryType::Regular | EntryType::Continuous)
        {
            continue;
        }
        let path = entry.path()?;
        let path = path.as_ref();
        if !is_safe_relative_path(path) {
            continue;
        }
        let dest = base.join(path);
        if !path_is_within_base(base, &dest) {
            continue;
        }
        if dest.exists() {
            files_skipped_existing += 1;
            continue;
        }
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut out = File::create(&dest)?;
        io::copy(&mut entry, &mut out)?;
        files_added += 1;
    }
    Ok(ImportStats {
        files_added,
        files_skipped_existing,
    })
}

fn is_safe_relative_path(path: &Path) -> bool {
    !path.is_absolute() && !path.components().any(|c| matches!(c, Component::ParentDir))
}

/// Ensures `candidate` does not escape `base` (after both are resolved under the same root).
fn path_is_within_base(base: &Path, candidate: &Path) -> bool {
    candidate.starts_with(base)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn merge_import_skips_existing_files() {
        let tmp = tempfile::tempdir().unwrap();
        let store = tmp.path().join("passgen");
        fs::create_dir_all(store.join("alpha")).unwrap();
        fs::write(store.join("alpha/pass"), b"keep").unwrap();

        let store_new = tmp.path().join("newonly");
        fs::create_dir_all(store_new.join("beta")).unwrap();
        fs::write(store_new.join("beta/pass"), b"new").unwrap();
        let archive = tmp.path().join("merge.zip");
        export_passgen_from(&store_new, &archive).unwrap();

        let stats = import_passgen_into(&store, &archive).unwrap();
        assert_eq!(stats.files_added, 1);
        assert_eq!(stats.files_skipped_existing, 0);
        assert_eq!(
            fs::read_to_string(store.join("alpha/pass")).unwrap(),
            "keep"
        );
        assert_eq!(fs::read_to_string(store.join("beta/pass")).unwrap(), "new");
    }

    #[test]
    fn import_skips_when_file_exists() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        fs::create_dir_all(src.join("x")).unwrap();
        fs::write(src.join("x/pass"), b"from_export").unwrap();
        let archive = tmp.path().join("a.tar.gz");
        export_passgen_from(&src, &archive).unwrap();

        let dest = tmp.path().join("dest");
        fs::create_dir_all(dest.join("x")).unwrap();
        fs::write(dest.join("x/pass"), b"original").unwrap();

        let stats = import_passgen_into(&dest, &archive).unwrap();
        assert_eq!(stats.files_added, 0);
        assert_eq!(stats.files_skipped_existing, 1);
        assert_eq!(fs::read_to_string(dest.join("x/pass")).unwrap(), "original");
    }

    #[test]
    fn roundtrip_zip() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("pg");
        fs::create_dir_all(src.join("n")).unwrap();
        fs::write(src.join("n/pass"), b"secret").unwrap();
        let archive = tmp.path().join("r.zip");
        export_passgen_from(&src, &archive).unwrap();

        let dst = tmp.path().join("out");
        let stats = import_passgen_into(&dst, &archive).unwrap();
        assert_eq!(stats.files_added, 1);
        assert_eq!(fs::read_to_string(dst.join("n/pass")).unwrap(), "secret");
    }
}
