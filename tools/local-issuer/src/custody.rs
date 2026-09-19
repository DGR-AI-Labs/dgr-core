//! Linux custody, bounded input, terminal and publication primitives. Agent-authored T0.
//! All raw Linux calls are confined here. No operational installation or key creation.
use crate::{Failure, issuance::TrustedClock, profile};
use base64::{Engine as _, engine::general_purpose::STANDARD_NO_PAD};
use sha2::{Digest, Sha256};
use std::{
    ffi::{CStr, CString},
    io::{self, Read},
    os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd},
    time::Duration,
};
use zeroize::{Zeroize, Zeroizing};

type Result<T> = std::result::Result<T, Failure>;
#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Operation {
    Acl,
    Write,
    SyncFile,
    SyncDirectory,
    Link,
    OpenInput,
    SetTermios,
}
#[cfg(test)]
#[derive(Clone, Copy)]
pub(super) enum Inject {
    Error(i32),
    Limit(usize),
    Zero,
    AfterLinkError(i32),
    AfterLinkPanic,
}
#[cfg(test)]
fn injection(operation: Operation) -> Option<Inject> {
    tests::injection(operation)
}

fn errno() -> i32 {
    io::Error::last_os_error().raw_os_error().unwrap_or(0)
}
fn cstring(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| Failure::Internal)
}
fn owned(fd: RawFd, phase: Failure) -> Result<OwnedFd> {
    if fd < 0 {
        return Err(phase);
    }
    // SAFETY: successful open/syscall returns a fresh descriptor; ownership transfers once.
    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}
fn stat(fd: RawFd) -> Result<libc::stat> {
    let mut value = std::mem::MaybeUninit::<libc::stat>::uninit();
    // SAFETY: valid writable stat buffer; fstat never takes ownership of fd.
    if unsafe { libc::fstat(fd, value.as_mut_ptr()) } != 0 {
        return Err(Failure::Custody);
    }
    // SAFETY: successful fstat initialized the output.
    Ok(unsafe { value.assume_init() })
}
fn no_acl(fd: RawFd, directory: bool) -> Result<()> {
    let names: &[&CStr] = if directory {
        &[c"system.posix_acl_access", c"system.posix_acl_default"]
    } else {
        &[c"system.posix_acl_access"]
    };
    for name in names {
        if acl_size(fd, name) != Err(libc::ENODATA) {
            return Err(Failure::Custody);
        }
    }
    Ok(())
}
fn acl_size(fd: RawFd, name: &CStr) -> std::result::Result<usize, i32> {
    #[cfg(test)]
    if let Some(action) = injection(Operation::Acl) {
        return match action {
            Inject::Error(e) => Err(e),
            Inject::Zero => Ok(0),
            Inject::Limit(n) => Ok(n),
            _ => panic!("invalid ACL script"),
        };
    }
    // SAFETY: held readable fd, static NUL-terminated name; null/zero requests size only.
    let n = unsafe { libc::fgetxattr(fd, name.as_ptr(), std::ptr::null_mut(), 0) };
    if n < 0 {
        Err(errno())
    } else {
        usize::try_from(n).map_err(|_| libc::EOVERFLOW)
    }
}
fn mount_id(fd: RawFd) -> Result<u64> {
    let mut value = std::mem::MaybeUninit::<libc::statx>::uninit();
    // SAFETY: statx ABI from pinned libc; empty path with AT_EMPTY_PATH uses held fd.
    if unsafe {
        libc::statx(
            fd,
            c"".as_ptr(),
            libc::AT_EMPTY_PATH | libc::AT_SYMLINK_NOFOLLOW,
            libc::STATX_MNT_ID,
            value.as_mut_ptr(),
        )
    } != 0
    {
        return Err(Failure::Custody);
    }
    // SAFETY: success initialized the buffer; mask is checked before using mount ID.
    let value = unsafe { value.assume_init() };
    if value.stx_mask & libc::STATX_MNT_ID == 0 {
        return Err(Failure::Custody);
    }
    Ok(value.stx_mnt_id)
}
fn filesystem(fd: RawFd, magic: libc::c_long) -> Result<()> {
    let mut info = std::mem::MaybeUninit::<libc::statfs>::uninit();
    // SAFETY: live fd and writable libc output buffer; initialized on success.
    if unsafe { libc::fstatfs(fd, info.as_mut_ptr()) } != 0 {
        return Err(Failure::Custody);
    }
    // SAFETY: preceding success initialized info.
    if unsafe { info.assume_init() }.f_type != magic {
        return Err(Failure::Custody);
    }
    Ok(())
}
fn open_dir(parent: RawFd, name: &CStr) -> Result<OwnedFd> {
    // SAFETY: name is NUL-terminated; openat receives no creation flags, returns owned fd.
    owned(
        unsafe {
            libc::openat(
                parent,
                name.as_ptr(),
                libc::O_RDONLY
                    | libc::O_DIRECTORY
                    | libc::O_CLOEXEC
                    | libc::O_NOFOLLOW
                    | libc::O_NONBLOCK,
            )
        },
        Failure::Custody,
    )
}
#[derive(Clone, PartialEq, Eq)]
struct Metadata {
    dev: u64,
    ino: u64,
    mode: u32,
    uid: u32,
    gid: u32,
    links: u64,
    size: i64,
    mtime: (i64, i64),
    ctime: (i64, i64),
    mount: u64,
}
impl Metadata {
    fn read(fd: RawFd) -> Result<Self> {
        let s = stat(fd)?;
        Ok(Self {
            dev: s.st_dev,
            ino: s.st_ino,
            mode: s.st_mode,
            uid: s.st_uid,
            gid: s.st_gid,
            links: s.st_nlink,
            size: s.st_size,
            mtime: (s.st_mtime, s.st_mtime_nsec),
            ctime: (s.st_ctime, s.st_ctime_nsec),
            mount: mount_id(fd)?,
        })
    }
}
struct Held {
    fd: OwnedFd,
    metadata: Metadata,
    directory: bool,
}
impl Held {
    fn new(fd: OwnedFd, directory: bool) -> Result<Self> {
        let metadata = Metadata::read(fd.as_raw_fd())?;
        no_acl(fd.as_raw_fd(), directory)?;
        Ok(Self {
            fd,
            metadata,
            directory,
        })
    }
    fn recheck(&self) -> Result<()> {
        no_acl(self.fd.as_raw_fd(), self.directory)?;
        let now = Metadata::read(self.fd.as_raw_fd())?;
        // Directory timestamps/link counts can change with legitimate anonymous/output entries.
        let same = if self.directory {
            now.dev == self.metadata.dev
                && now.ino == self.metadata.ino
                && now.mode == self.metadata.mode
                && now.uid == self.metadata.uid
                && now.gid == self.metadata.gid
                && now.mount == self.metadata.mount
        } else {
            now == self.metadata
        };
        if !same {
            return Err(Failure::Custody);
        }
        Ok(())
    }
}
pub struct Snapshot {
    held: Held,
    pub bytes: Vec<u8>,
}
pub struct ApprovedOperatorConfig {
    uid: u32,
    gid: u32,
    mount_ns: (u64, u64),
    user_ns: (u64, u64),
    root_mount: u64,
    registry_hash: [u8; 32],
    pub profile_hash: [u8; 32],
    custody: (u64, u64, u64),
    output: (u64, u64, u64),
    custody_path: String,
    output_path: String,
}
fn parse_launch(b: &[u8]) -> Result<ApprovedOperatorConfig> {
    if !(214..=1236).contains(&b.len()) || &b[..8] != b"DGRLCH1\0" || b[8..12] != [0, 1, 0, 0] {
        return Err(Failure::Launch);
    }
    let u32_at = |i| -> Result<u32> {
        Ok(u32::from_be_bytes(
            b.get(i..i + 4)
                .ok_or(Failure::Launch)?
                .try_into()
                .map_err(|_| Failure::Launch)?,
        ))
    };
    let u64_at = |i| -> Result<u64> {
        Ok(u64::from_be_bytes(
            b.get(i..i + 8)
                .ok_or(Failure::Launch)?
                .try_into()
                .map_err(|_| Failure::Launch)?,
        ))
    };
    let hash = |i| -> Result<[u8; 32]> {
        b.get(i..i + 32)
            .ok_or(Failure::Launch)?
            .try_into()
            .map_err(|_| Failure::Launch)
    };
    let n = usize::from(u16::from_be_bytes([b[208], b[209]]));
    let m = usize::from(u16::from_be_bytes([b[210], b[211]]));
    if u32_at(12)? as usize != b.len()
        || b.len() != 212 + n + m
        || !(1..=512).contains(&n)
        || !(1..=512).contains(&m)
        || u32_at(16)? == 0
        || u32_at(20)? == 0
        || hash(176)? != profile::fixture_inventory_hash()
    {
        return Err(Failure::Launch);
    }
    let path = |p: &[u8]| -> Result<String> {
        if !profile::valid_path(p) {
            return Err(Failure::Launch);
        }
        String::from_utf8(p.to_vec()).map_err(|_| Failure::Launch)
    };
    let custody_path = path(&b[212..212 + n])?;
    let output_path = path(&b[212 + n..])?;
    if custody_path == output_path
        || custody_path.starts_with(&(output_path.clone() + "/"))
        || output_path.starts_with(&(custody_path.clone() + "/"))
    {
        return Err(Failure::Launch);
    }
    let config = ApprovedOperatorConfig {
        uid: u32_at(16)?,
        gid: u32_at(20)?,
        mount_ns: (u64_at(24)?, u64_at(32)?),
        user_ns: (u64_at(40)?, u64_at(48)?),
        root_mount: u64_at(56)?,
        registry_hash: hash(64)?,
        profile_hash: hash(96)?,
        custody: (u64_at(128)?, u64_at(136)?, u64_at(144)?),
        output: (u64_at(152)?, u64_at(160)?, u64_at(168)?),
        custody_path,
        output_path,
    };
    if (config.custody.0, config.custody.1) == (config.output.0, config.output.1) {
        return Err(Failure::Launch);
    }
    Ok(config)
}
fn identity() -> Result<(u32, u32)> {
    let (mut r, mut e, mut s, mut gr, mut ge, mut gs) = (0, 0, 0, 0, 0, 0);
    // SAFETY: writable uid_t/gid_t outputs are distinct and remain live throughout each call.
    if unsafe { libc::getresuid(&mut r, &mut e, &mut s) } != 0
        || unsafe { libc::getresgid(&mut gr, &mut ge, &mut gs) } != 0
    {
        return Err(Failure::Launch);
    }
    validate_identity(r, e, s, gr, ge, gs)
}
fn validate_identity(r: u32, e: u32, s: u32, gr: u32, ge: u32, gs: u32) -> Result<(u32, u32)> {
    if r == 0 || gr == 0 || r != e || r != s || gr != ge || gr != gs {
        return Err(Failure::Launch);
    }
    Ok((r, gr))
}
#[repr(C)]
struct CapHeader {
    version: u32,
    pid: i32,
}
#[repr(C)]
#[derive(Default)]
struct CapData {
    effective: u32,
    permitted: u32,
    inheritable: u32,
}
fn no_capabilities() -> Result<()> {
    let mut header = CapHeader {
        version: 0x20080522,
        pid: 0,
    };
    let mut data = [CapData::default(), CapData::default()];
    // SAFETY: Linux v3 capability ABI uses this header and two initialized 12-byte records.
    if unsafe {
        libc::syscall(
            libc::SYS_capget,
            &mut header as *mut CapHeader,
            data.as_mut_ptr(),
        )
    } != 0
        || data
            .iter()
            .any(|d| d.effective != 0 || d.permitted != 0 || d.inheritable != 0)
    {
        return Err(Failure::Resource);
    }
    for cap in 0..=40 {
        // SAFETY: scalar PR_CAP_AMBIENT query, unused variadic arguments explicitly zero.
        if unsafe { libc::prctl(libc::PR_CAP_AMBIENT, libc::PR_CAP_AMBIENT_IS_SET, cap, 0, 0) } != 0
        {
            return Err(Failure::Resource);
        }
    }
    Ok(())
}
fn lower_limit(resource: libc::__rlimit_resource_t, ceiling: u64) -> Result<u64> {
    let mut limit = std::mem::MaybeUninit::<libc::rlimit>::uninit();
    // SAFETY: valid output buffer and supported Linux resource selector.
    if unsafe { libc::getrlimit(resource, limit.as_mut_ptr()) } != 0 {
        return Err(Failure::Resource);
    }
    // SAFETY: successful getrlimit initialized limit.
    let old = unsafe { limit.assume_init() };
    let bound = old.rlim_cur.min(old.rlim_max).min(ceiling);
    let new = libc::rlimit {
        rlim_cur: bound,
        rlim_max: bound,
    };
    // SAFETY: pointer to initialized rlimit; lower-only values.
    if unsafe { libc::setrlimit(resource, &new) } != 0 {
        return Err(Failure::Resource);
    }
    Ok(bound)
}
pub fn harden_process() -> Result<()> {
    identity()?;
    no_capabilities()?;
    lower_limit(libc::RLIMIT_CORE, 0)?;
    let address_limit = lower_limit(libc::RLIMIT_AS, 512 * 1024 * 1024)?;
    // Frozen scrypt N=2^18, r=8 allocates a 256 MiB V buffer alone. Reject an
    // inherited limit that cannot accommodate it plus any process overhead, before
    // input or secrets. A larger limit is not a guarantee against later allocation abort.
    if address_limit <= 256 * 1024 * 1024 {
        return Err(Failure::Resource);
    }
    lower_limit(libc::RLIMIT_CPU, 10)?;
    // SAFETY: scalar prctl operations and umask; this single-threaded CLI owns process policy.
    if unsafe { libc::prctl(libc::PR_SET_DUMPABLE, 0, 0, 0, 0) } != 0
        || unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) } != 0
    {
        return Err(Failure::Resource);
    }
    // SAFETY: changes only this process's creation mask.
    unsafe { libc::umask(0o077) };
    // SAFETY: checked Linux close_range closes inherited descriptors before any owned fd exists.
    if unsafe { libc::syscall(libc::SYS_close_range, 3_u32, u32::MAX, 0_u32) } != 0 {
        return Err(Failure::Resource);
    }
    Ok(())
}
pub fn clock_pair() -> Result<(Duration, Duration)> {
    fn sample(id: libc::clockid_t) -> Result<Duration> {
        let mut t = std::mem::MaybeUninit::<libc::timespec>::uninit();
        // SAFETY: initialized on successful clock_gettime; no pointers retained.
        if unsafe { libc::clock_gettime(id, t.as_mut_ptr()) } != 0 {
            return Err(Failure::Clock);
        }
        // SAFETY: successful call initialized t.
        let t = unsafe { t.assume_init() };
        if t.tv_sec < 0 || !(0..1_000_000_000).contains(&t.tv_nsec) {
            return Err(Failure::Clock);
        }
        Ok(Duration::new(
            u64::try_from(t.tv_sec).map_err(|_| Failure::Clock)?,
            u32::try_from(t.tv_nsec).map_err(|_| Failure::Clock)?,
        ))
    }
    Ok((
        sample(libc::CLOCK_REALTIME)?,
        sample(libc::CLOCK_MONOTONIC)?,
    ))
}
fn poll_read(
    fd: RawFd,
    signal: Option<RawFd>,
    deadline: Duration,
    clock: &mut TrustedClock,
    phase: Failure,
) -> Result<()> {
    loop {
        let timeout = clock.remaining_ms(deadline)?;
        if timeout == 0 {
            return Err(phase);
        }
        let mut fds = [
            libc::pollfd {
                fd,
                events: libc::POLLIN,
                revents: 0,
            },
            libc::pollfd {
                fd: signal.unwrap_or(-1),
                events: libc::POLLIN,
                revents: 0,
            },
        ];
        // SAFETY: two writable pollfd entries; bounded millisecond timeout.
        let result = unsafe { libc::poll(fds.as_mut_ptr(), 2, timeout) };
        if result < 0 {
            if errno() == libc::EINTR {
                continue;
            }
            return Err(phase);
        }
        if result == 0 {
            return Err(phase);
        }
        if fds[1].revents != 0 {
            return Err(Failure::Cancelled);
        }
        if fds[0].revents & (libc::POLLERR | libc::POLLNVAL) != 0 {
            return Err(phase);
        }
        if fds[0].revents & (libc::POLLIN | libc::POLLHUP) != 0 {
            if clock.remaining_ms(deadline)? == 0 {
                return Err(phase);
            }
            return Ok(());
        }
    }
}
fn read_raw(fd: RawFd, buffer: &mut [u8]) -> io::Result<usize> {
    // SAFETY: mutable slice describes writable memory; read borrows descriptor without ownership.
    let n = unsafe { libc::read(fd, buffer.as_mut_ptr().cast(), buffer.len()) };
    if n < 0 {
        Err(io::Error::last_os_error())
    } else {
        usize::try_from(n).map_err(io::Error::other)
    }
}
pub fn read_request(clock: &mut TrustedClock) -> Result<Vec<u8>> {
    let s = stat(0).map_err(|_| Failure::Request)?;
    if s.st_mode & libc::S_IFMT != libc::S_IFREG && s.st_mode & libc::S_IFMT != libc::S_IFIFO {
        return Err(Failure::Request);
    }
    // SAFETY: fcntl scalar flag query on stdin.
    let flags = unsafe { libc::fcntl(0, libc::F_GETFL) };
    if flags < 0 {
        return Err(Failure::Request);
    }
    // SAFETY: preserve all flags and set nonblocking for bounded reads.
    if unsafe { libc::fcntl(0, libc::F_SETFL, flags | libc::O_NONBLOCK) } < 0 {
        return Err(Failure::Request);
    }
    let result = (|| {
        let deadline = clock.deadline(10)?;
        let mut buffer = [0_u8; 246];
        let mut length = 0;
        loop {
            poll_read(0, None, deadline, clock, Failure::Request)?;
            match read_raw(0, &mut buffer[length..]) {
                Ok(0) => return Ok(buffer[..length].to_vec()),
                Ok(n) => {
                    length += n;
                    if length > crate::request::MAX_REQUEST {
                        return Err(Failure::Request);
                    }
                }
                Err(e) if matches!(e.raw_os_error(), Some(libc::EINTR | libc::EAGAIN)) => {}
                Err(_) => return Err(Failure::Request),
            }
        }
    })();
    // SAFETY: restore exact original shared-description flags on every ordinary result.
    if unsafe { libc::fcntl(0, libc::F_SETFL, flags) } < 0 {
        return Err(Failure::Request);
    }
    result
}
fn bounded_snapshot(fd: OwnedFd, max: usize, owner: u32, mode: u32) -> Result<Snapshot> {
    let held = Held::new(fd, false)?;
    let m = &held.metadata;
    if m.mode != libc::S_IFREG | mode
        || m.uid != owner
        || m.links != 1
        || m.size < 0
        || usize::try_from(m.size).map_err(|_| Failure::Custody)? > max
    {
        return Err(Failure::Custody);
    }
    let mut bytes = vec![0; max + 1];
    let mut length = 0;
    loop {
        match read_raw(held.fd.as_raw_fd(), &mut bytes[length..]) {
            Ok(0) => break,
            Ok(n) => {
                length += n;
                if length > max {
                    return Err(Failure::Custody);
                }
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => return Err(Failure::Custody),
        }
    }
    bytes.truncate(length);
    if i64::try_from(length).map_err(|_| Failure::Custody)? != m.size {
        return Err(Failure::Custody);
    }
    held.recheck()?;
    Ok(Snapshot { held, bytes })
}
#[repr(C)]
struct OpenHow {
    flags: u64,
    mode: u64,
    resolve: u64,
}
fn inside(root: RawFd, name: &CStr) -> Result<OwnedFd> {
    #[cfg(test)]
    if let Some(action) = injection(Operation::OpenInput) {
        match action {
            Inject::Error(_) => return Err(Failure::Custody),
            _ => panic!("invalid open script"),
        }
    }
    let how = OpenHow {
        flags: (libc::O_RDONLY | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK) as u64,
        mode: 0,
        resolve: 0x08 | 0x04 | 0x02 | 0x01,
    };
    // SAFETY: x86_64 open_how ABI is three u64; fixed relative name; live directory descriptor.
    let fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            root,
            name.as_ptr(),
            &how as *const OpenHow,
            std::mem::size_of::<OpenHow>(),
        )
    };
    owned(
        i32::try_from(fd).map_err(|_| Failure::Custody)?,
        Failure::Custody,
    )
}
fn ancestor(fd: OwnedFd, uid: u32) -> Result<Held> {
    let held = Held::new(fd, true)?;
    let m = &held.metadata;
    if m.mode & libc::S_IFMT != libc::S_IFDIR
        || (m.uid != 0 && m.uid != uid)
        || m.mode & 0o6022 != 0
    {
        return Err(Failure::Custody);
    }
    Ok(held)
}
fn walk_root(
    path: &str,
    expected: (u64, u64, u64),
    config: &ApprovedOperatorConfig,
) -> Result<Vec<Held>> {
    let mut chain = vec![ancestor(open_dir(libc::AT_FDCWD, c"/")?, config.uid)?];
    if chain[0].metadata.mount != config.root_mount {
        return Err(Failure::Custody);
    }
    let mut transitioned = false;
    let mut previous = config.root_mount;
    for component in path[1..].split('/') {
        let next = ancestor(
            open_dir(
                chain.last().ok_or(Failure::Internal)?.fd.as_raw_fd(),
                &cstring(component)?,
            )?,
            config.uid,
        )?;
        let id = next.metadata.mount;
        if id != previous {
            if transitioned || previous != config.root_mount || id != expected.2 {
                return Err(Failure::Custody);
            }
            transitioned = true;
        }
        previous = id;
        chain.push(next);
    }
    let last = chain.last().ok_or(Failure::Internal)?;
    let m = &last.metadata;
    if (m.dev, m.ino, m.mount) != expected || m.uid != config.uid || m.mode != libc::S_IFDIR | 0o700
    {
        return Err(Failure::Custody);
    }
    filesystem(last.fd.as_raw_fd(), 0xef53)?;
    Ok(chain)
}
fn namespace(proc: RawFd, name: &CStr) -> Result<(u64, u64)> {
    // SAFETY: only fixed self namespace paths may follow proc namespace links. No caller path.
    let fd = owned(
        unsafe { libc::openat(proc, name.as_ptr(), libc::O_RDONLY | libc::O_CLOEXEC) },
        Failure::Launch,
    )?;
    let s = stat(fd.as_raw_fd())?;
    Ok((s.st_dev, s.st_ino))
}
// Approved P1: kernel procfs descriptors are a separate class, not an ACL-error fallback.
struct ProcDescriptor {
    fd: OwnedFd,
    metadata: Metadata,
}
impl ProcDescriptor {
    fn recheck(&self, uid: u32) -> Result<()> {
        let now = Metadata::read(self.fd.as_raw_fd())?;
        if now.dev != self.metadata.dev
            || now.ino != self.metadata.ino
            || now.mount != self.metadata.mount
            || now.mode != self.metadata.mode
            || now.uid != self.metadata.uid
            || now.gid != self.metadata.gid
        {
            return Err(Failure::Launch);
        }
        validate_proc(self.fd.as_raw_fd(), &now, uid)
    }
}
fn trusted_proc(uid: u32) -> Result<ProcDescriptor> {
    let fd = open_dir(libc::AT_FDCWD, c"/proc")?;
    let metadata = Metadata::read(fd.as_raw_fd())?;
    validate_proc(fd.as_raw_fd(), &metadata, uid)?;
    Ok(ProcDescriptor { fd, metadata })
}
fn validate_proc(fd: RawFd, metadata: &Metadata, uid: u32) -> Result<()> {
    if metadata.mode & libc::S_IFMT != libc::S_IFDIR
        || metadata.uid != 0
        || metadata.mode & 0o6022 != 0
    {
        return Err(Failure::Launch);
    }
    filesystem(fd, 0x9fa0)?;
    let mut target = [0_u8; 32];
    // SAFETY: fixed proc link, bounded writable buffer; no NUL termination assumed.
    let n = unsafe {
        libc::readlinkat(
            fd,
            c"self".as_ptr(),
            target.as_mut_ptr().cast(),
            target.len(),
        )
    };
    // SAFETY: getpid has no pointer arguments or ownership effects.
    let pid = unsafe { libc::getpid() }.to_string();
    if n < 0
        || usize::try_from(n).map_err(|_| Failure::Launch)? != pid.len()
        || &target[..pid.len()] != pid.as_bytes()
    {
        return Err(Failure::Launch);
    }
    // SAFETY: fixed self path intentionally follows current-process procfs link.
    let self_fd = owned(
        unsafe {
            libc::openat(
                fd,
                c"self".as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
            )
        },
        Failure::Launch,
    )?;
    if stat(self_fd.as_raw_fd())?.st_uid != uid {
        return Err(Failure::Launch);
    }
    Ok(())
}
pub struct CustodyInputs {
    pub config: ApprovedOperatorConfig,
    launch: Snapshot,
    launch_chain: Vec<Held>,
    proc: ProcDescriptor,
    custody_chain: Vec<Held>,
    output_chain: Vec<Held>,
    pub registry: Snapshot,
    pub profile: Snapshot,
    pub envelope: Snapshot,
}
pub fn open_custody_inputs() -> Result<CustodyInputs> {
    let (launch_chain, launch, config, proc) = (|| {
        let mut chain = vec![ancestor(open_dir(libc::AT_FDCWD, c"/")?, 0)?];
        for name in [c"etc", c"dgr", c"local-issuer"] {
            chain.push(ancestor(
                open_dir(chain.last().ok_or(Failure::Internal)?.fd.as_raw_fd(), name)?,
                0,
            )?);
        }
        // SAFETY: fixed file under held validated directory; nonblocking avoids FIFO open hang.
        let fd = owned(
            unsafe {
                libc::openat(
                    chain.last().ok_or(Failure::Internal)?.fd.as_raw_fd(),
                    c"launch.bin".as_ptr(),
                    libc::O_RDONLY | libc::O_NONBLOCK | libc::O_CLOEXEC | libc::O_NOFOLLOW,
                )
            },
            Failure::Launch,
        )?;
        let launch = bounded_snapshot(fd, 1236, 0, 0o444)?;
        let config = parse_launch(&launch.bytes)?;
        if identity()? != (config.uid, config.gid) || chain[0].metadata.mount != config.root_mount {
            return Err(Failure::Launch);
        }
        let proc = trusted_proc(config.uid)?;
        if namespace(proc.fd.as_raw_fd(), c"self/ns/mnt")? != config.mount_ns
            || namespace(proc.fd.as_raw_fd(), c"self/ns/user")? != config.user_ns
        {
            return Err(Failure::Launch);
        }
        Ok((chain, launch, config, proc))
    })()
    .map_err(|_: Failure| Failure::Launch)?;
    let custody_chain = walk_root(&config.custody_path, config.custody, &config)?;
    let output_chain = walk_root(&config.output_path, config.output, &config)?;
    let root = custody_chain
        .last()
        .ok_or(Failure::Internal)?
        .fd
        .as_raw_fd();
    let registry = bounded_snapshot(
        inside(root, c"registrations.bin")?,
        28688,
        config.uid,
        0o400,
    )?;
    let profile = bounded_snapshot(inside(root, c"profile.bin")?, 16384, config.uid, 0o400)?;
    let envelope = bounded_snapshot(inside(root, c"signing-key.age")?, 4096, config.uid, 0o600)?;
    if <[u8; 32]>::from(Sha256::digest(&registry.bytes)) != config.registry_hash
        || <[u8; 32]>::from(Sha256::digest(&profile.bytes)) != config.profile_hash
    {
        return Err(Failure::Custody);
    }
    Ok(CustodyInputs {
        config,
        launch,
        launch_chain,
        proc,
        custody_chain,
        output_chain,
        registry,
        profile,
        envelope,
    })
}
impl CustodyInputs {
    fn output_fd(&self) -> Result<RawFd> {
        Ok(self
            .output_chain
            .last()
            .ok_or(Failure::Internal)?
            .fd
            .as_raw_fd())
    }
    pub fn recheck(&self) -> Result<()> {
        if identity()? != (self.config.uid, self.config.gid) {
            return Err(Failure::Custody);
        }
        for held in self
            .launch_chain
            .iter()
            .chain(&self.custody_chain)
            .chain(&self.output_chain)
        {
            held.recheck()?;
        }
        for snapshot in [&self.launch, &self.registry, &self.profile, &self.envelope] {
            snapshot.held.recheck()?;
        }
        self.proc.recheck(self.config.uid)?;
        // Held snapshots are immutable; metadata and anchored hashes were checked at capture.
        if namespace(self.proc.fd.as_raw_fd(), c"self/ns/mnt")? != self.config.mount_ns
            || namespace(self.proc.fd.as_raw_fd(), c"self/ns/user")? != self.config.user_ns
        {
            return Err(Failure::Custody);
        }
        Ok(())
    }
}
pub fn verify_envelope_binding(bytes: &[u8], expected: &[u8; 32]) -> Result<()> {
    if &<[u8; 32]>::from(Sha256::digest(bytes)) != expected {
        return Err(Failure::Custody);
    }
    Ok(())
}
fn envelope_shape(b: &[u8]) -> Result<()> {
    if b.len() != 214
        || &b[..22] != b"age-encryption.org/v1\n"
        || &b[22..32] != b"-> scrypt "
        || &b[54..58] != b" 18\n"
        || b[101] != b'\n'
        || &b[102..106] != b"--- "
        || b[149] != b'\n'
    {
        return Err(Failure::Unlock);
    }
    // Header length is verified independently below, before any KDF/identity work.
    for (part, n) in [(&b[32..54], 16), (&b[58..101], 32), (&b[106..149], 32)] {
        let decoded = STANDARD_NO_PAD.decode(part).map_err(|_| Failure::Unlock)?;
        if decoded.len() != n || STANDARD_NO_PAD.encode(&decoded).as_bytes() != part {
            return Err(Failure::Unlock);
        }
    }
    Ok(())
}
pub fn unlock_once(
    envelope: &[u8],
    password: Zeroizing<Vec<u8>>,
) -> Result<ed25519_dalek::SigningKey> {
    envelope_shape(envelope)?;
    let text = std::str::from_utf8(&password).map_err(|_| Failure::Unlock)?;
    let pass = age::secrecy::SecretString::from(text.to_owned());
    drop(password);
    let decryptor = age::Decryptor::new(envelope).map_err(|_| Failure::Unlock)?;
    let mut identity = age::scrypt::Identity::new(pass);
    identity.set_max_work_factor(18);
    let mut reader = decryptor
        .decrypt(std::iter::once(&identity as &dyn age::Identity))
        .map_err(|_| Failure::Unlock)?;
    let mut seed = Zeroizing::new([0_u8; 32]);
    reader
        .read_exact(&mut seed[..])
        .map_err(|_| Failure::Unlock)?;
    let mut extra = Zeroizing::new([0_u8; 1]);
    if reader.read(&mut extra[..]).map_err(|_| Failure::Unlock)? != 0 {
        return Err(Failure::Unlock);
    }
    Ok(ed25519_dalek::SigningKey::from_bytes(&seed))
}

fn set_termios(fd: RawFd, value: &libc::termios) -> Result<()> {
    #[cfg(test)]
    if let Some(action) = injection(Operation::SetTermios) {
        match action {
            Inject::Error(_) => return Err(Failure::Terminal),
            _ => panic!("invalid termios script"),
        }
    }
    // SAFETY: held terminal descriptor and initialized termios; synchronous copy by libc.
    if unsafe { libc::tcsetattr(fd, libc::TCSANOW, value) } != 0 {
        Err(Failure::Terminal)
    } else {
        Ok(())
    }
}
struct TerminalSession {
    fd: Option<OwnedFd>,
    signal: Option<OwnedFd>,
    original: libc::termios,
    mask: libc::sigset_t,
    modified: bool,
    masked: bool,
}
fn termios_equal(a: &libc::termios, b: &libc::termios) -> bool {
    a.c_iflag == b.c_iflag
        && a.c_oflag == b.c_oflag
        && a.c_cflag == b.c_cflag
        && a.c_lflag == b.c_lflag
        && a.c_line == b.c_line
        && a.c_cc == b.c_cc
        && a.c_ispeed == b.c_ispeed
        && a.c_ospeed == b.c_ospeed
}
impl TerminalSession {
    fn open() -> Result<Self> {
        // SAFETY: fixed trusted device path; no creation; nonblocking for deadline-bounded I/O.
        let fd = owned(
            unsafe {
                libc::open(
                    c"/dev/tty".as_ptr(),
                    libc::O_RDWR | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NONBLOCK,
                )
            },
            Failure::Terminal,
        )?;
        // SAFETY: terminal query and scalar process-group observation.
        if unsafe { libc::isatty(fd.as_raw_fd()) } != 1
            || unsafe { libc::tcgetpgrp(fd.as_raw_fd()) } != unsafe { libc::getpgrp() }
        {
            return Err(Failure::Terminal);
        }
        let mut original = std::mem::MaybeUninit::<libc::termios>::uninit();
        let mut mask = std::mem::MaybeUninit::<libc::sigset_t>::zeroed();
        // SAFETY: writable outputs initialized by successful calls; null set means query only.
        if unsafe { libc::tcgetattr(fd.as_raw_fd(), original.as_mut_ptr()) } != 0
            || unsafe {
                libc::pthread_sigmask(libc::SIG_SETMASK, std::ptr::null(), mask.as_mut_ptr())
            } != 0
        {
            return Err(Failure::Terminal);
        }
        // SAFETY: preceding successful calls initialized both outputs.
        let (original, mask) = unsafe { (original.assume_init(), mask.assume_init()) };
        let mut session = Self {
            fd: Some(fd),
            signal: None,
            original,
            mask,
            modified: false,
            masked: false,
        };
        session.configure()?;
        Ok(session)
    }
    fn raw(&self) -> Result<RawFd> {
        self.fd
            .as_ref()
            .map(AsRawFd::as_raw_fd)
            .ok_or(Failure::Terminal)
    }
    fn configure(&mut self) -> Result<()> {
        // SAFETY: on the supported Linux x86_64 ABI, sigset_t contains integer words;
        // all-zero is a valid initialized representation. sigemptyset establishes the empty set.
        let mut signals = unsafe { std::mem::zeroed::<libc::sigset_t>() };
        // SAFETY: live initialized sigset_t pointer.
        if unsafe { libc::sigemptyset(&mut signals) } != 0 {
            return Err(Failure::Terminal);
        }
        for signal in [
            libc::SIGINT,
            libc::SIGTERM,
            libc::SIGHUP,
            libc::SIGQUIT,
            libc::SIGTSTP,
        ] {
            // SAFETY: supported signal number and writable initialized signal set.
            if unsafe { libc::sigaddset(&mut signals, signal) } != 0 {
                return Err(Failure::Terminal);
            }
        }
        // SAFETY: thread-local mask, single-threaded issue path; original mask already saved.
        if unsafe { libc::pthread_sigmask(libc::SIG_BLOCK, &signals, std::ptr::null_mut()) } != 0 {
            return Err(Failure::Terminal);
        }
        self.masked = true;
        // SAFETY: signalfd creates a fresh owned descriptor for the blocked set.
        self.signal = Some(owned(
            unsafe { libc::signalfd(-1, &signals, libc::SFD_CLOEXEC | libc::SFD_NONBLOCK) },
            Failure::Terminal,
        )?);
        let mut raw = self.original;
        raw.c_lflag &= !(libc::ECHO | libc::ECHONL | libc::ICANON | libc::IEXTEN);
        raw.c_iflag &= !(libc::ICRNL
            | libc::INLCR
            | libc::IGNCR
            | libc::ISTRIP
            | libc::IXON
            | libc::IXOFF
            | libc::PARMRK);
        raw.c_cc[libc::VMIN] = 1;
        raw.c_cc[libc::VTIME] = 0;
        self.modified = true;
        // SAFETY: fd is the held controlling terminal; termios is initialized; flush queued input.
        if unsafe { libc::tcflush(self.raw()?, libc::TCIFLUSH) } != 0
            || set_termios(self.raw()?, &raw).is_err()
        {
            return Err(Failure::Terminal);
        }
        Ok(())
    }
    fn restore(&mut self) -> Result<()> {
        let mut failed = false;
        let mut cancelled = false;
        if self.modified {
            let fd = self.raw()?;
            // SAFETY: held terminal and exact saved termios; attempt restore even if flush fails.
            if unsafe { libc::tcflush(fd, libc::TCIFLUSH) } != 0 {
                failed = true
            }
            // SAFETY: original termios copied from this descriptor before modifications.
            if set_termios(fd, &self.original).is_err() {
                failed = true
            }
            let mut now = std::mem::MaybeUninit::<libc::termios>::uninit();
            // SAFETY: writable output buffer; only inspect on success.
            if unsafe { libc::tcgetattr(fd, now.as_mut_ptr()) } != 0 {
                failed = true
            } else {
                // SAFETY: tcgetattr succeeded.
                if !termios_equal(&unsafe { now.assume_init() }, &self.original) {
                    failed = true
                }
            }
            self.modified = false;
        }
        // Drain already pending handled notifications while still blocked, then close input/signal
        // descriptors before restoring the exact mask. A new signal may terminate after restoration.
        if let Some(signal) = &self.signal {
            let mut buffer = [0_u8; 128];
            loop {
                match read_raw(signal.as_raw_fd(), &mut buffer) {
                    Ok(128) => {
                        cancelled = true;
                    }
                    Err(e) if e.raw_os_error() == Some(libc::EAGAIN) => break,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    _ => {
                        failed = true;
                        break;
                    }
                }
            }
        }
        self.signal.take();
        self.fd.take();
        if self.masked {
            // SAFETY: restore same issuing thread's original initialized mask after terminal cleanup.
            if unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, &self.mask, std::ptr::null_mut()) }
                != 0
            {
                failed = true
            }
            self.masked = false;
        }
        if failed {
            Err(Failure::Terminal)
        } else if cancelled {
            Err(Failure::Cancelled)
        } else {
            Ok(())
        }
    }
    fn write_prompt(
        &self,
        bytes: &[u8],
        deadline: Duration,
        clock: &mut TrustedClock,
    ) -> Result<()> {
        let mut position = 0;
        while position < bytes.len() {
            let remaining = clock.remaining_ms(deadline)?;
            if remaining == 0 {
                return Err(Failure::Cancelled);
            }
            let mut fds = [
                libc::pollfd {
                    fd: self.raw()?,
                    events: libc::POLLOUT,
                    revents: 0,
                },
                libc::pollfd {
                    fd: self.signal.as_ref().ok_or(Failure::Terminal)?.as_raw_fd(),
                    events: libc::POLLIN,
                    revents: 0,
                },
            ];
            // SAFETY: initialized writable two-entry poll array; deadline-derived timeout.
            let n = unsafe { libc::poll(fds.as_mut_ptr(), 2, remaining) };
            if n < 0 && errno() == libc::EINTR {
                continue;
            }
            if n < 0 {
                return Err(Failure::Terminal);
            }
            if n == 0 || fds[1].revents != 0 {
                return Err(Failure::Cancelled);
            }
            if fds[0].revents & (libc::POLLERR | libc::POLLHUP | libc::POLLNVAL) != 0 {
                return Err(Failure::Terminal);
            }
            // SAFETY: live terminal fd and immutable slice; no pointers retained.
            let count = unsafe {
                libc::write(
                    self.raw()?,
                    bytes[position..].as_ptr().cast(),
                    bytes.len() - position,
                )
            };
            if count < 0 {
                if matches!(errno(), libc::EAGAIN | libc::EINTR) {
                    continue;
                }
                return Err(Failure::Terminal);
            }
            if count == 0 {
                return Err(Failure::Terminal);
            }
            position += usize::try_from(count).map_err(|_| Failure::Terminal)?;
        }
        Ok(())
    }
}
impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}
fn edit_input(buffer: &mut [u8; 1024], length: &mut usize, byte: u8, max: usize) -> Result<bool> {
    if byte == b'\r' || byte == b'\n' {
        let text = std::str::from_utf8(&buffer[..*length]).map_err(|_| Failure::Cancelled)?;
        if text.is_empty() || text.chars().any(char::is_control) {
            return Err(Failure::Cancelled);
        }
        return Ok(true);
    }
    if byte == 8 || byte == 127 {
        let text = std::str::from_utf8(&buffer[..*length]).map_err(|_| Failure::Cancelled)?;
        if let Some((start, _)) = text.char_indices().next_back() {
            buffer[start..*length].zeroize();
            *length = start;
        }
        return Ok(false);
    }
    if byte < 32 || *length >= max {
        return Err(Failure::Cancelled);
    }
    buffer[*length] = byte;
    *length += 1;
    match std::str::from_utf8(&buffer[..*length]) {
        Ok(text) => {
            if text.chars().any(char::is_control) {
                return Err(Failure::Cancelled);
            }
        }
        Err(error) if error.error_len().is_some() => return Err(Failure::Cancelled),
        Err(_) => {}
    }
    Ok(false)
}
fn read_terminal_line(
    terminal: &TerminalSession,
    maximum: usize,
    deadline: Duration,
    clock: &mut TrustedClock,
) -> Result<Zeroizing<Vec<u8>>> {
    let mut buffer = Zeroizing::new([0_u8; 1024]);
    let mut length = 0;
    let mut byte = Zeroizing::new([0_u8; 1]);
    loop {
        poll_read(
            terminal.raw()?,
            terminal.signal.as_ref().map(AsRawFd::as_raw_fd),
            deadline,
            clock,
            Failure::Cancelled,
        )?;
        match read_raw(terminal.raw()?, &mut byte[..]) {
            Ok(1) => {
                if edit_input(&mut buffer, &mut length, byte[0], maximum)? {
                    return Ok(Zeroizing::new(buffer[..length].to_vec()));
                }
            }
            Err(e) if matches!(e.raw_os_error(), Some(libc::EAGAIN | libc::EINTR)) => {}
            _ => return Err(Failure::Cancelled),
        }
    }
}
pub fn terminal_input(
    prompt: &[u8],
    maximum: usize,
    clock: &mut TrustedClock,
) -> Result<Zeroizing<Vec<u8>>> {
    if maximum > 1024 {
        return Err(Failure::Internal);
    }
    let mut terminal = TerminalSession::open()?;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let deadline = clock.deadline(60)?;
        terminal.write_prompt(prompt, deadline, clock)?;
        read_terminal_line(&terminal, maximum, deadline, clock)
    }))
    .unwrap_or(Err(Failure::Internal));
    terminal.restore()?;
    result
}

pub fn anonymous_output(inputs: &CustodyInputs) -> Result<OwnedFd> {
    // SAFETY: fixed '.' under retained directory; mode provided for O_TMPFILE creation.
    let fd = owned(
        unsafe {
            libc::openat(
                inputs.output_fd()?,
                c".".as_ptr(),
                libc::O_TMPFILE | libc::O_RDWR | libc::O_CLOEXEC,
                0o600,
            )
        },
        Failure::Unpublished,
    )?;
    // SAFETY: own anonymous inode; explicit mode establishes 0600 even under a tighter umask.
    if unsafe { libc::fchmod(fd.as_raw_fd(), 0o600) } != 0 {
        return Err(Failure::Unpublished);
    }
    let m = Metadata::read(fd.as_raw_fd()).map_err(|_| Failure::Unpublished)?;
    if m.mode != libc::S_IFREG | 0o600
        || m.uid != inputs.config.uid
        || m.dev != inputs.config.output.0
        || m.mount != inputs.config.output.2
        || m.links != 0
    {
        return Err(Failure::Unpublished);
    }
    no_acl(fd.as_raw_fd(), false).map_err(|_| Failure::Unpublished)?;
    Ok(fd)
}
fn output_write(fd: RawFd, bytes: &[u8]) -> io::Result<usize> {
    #[cfg(test)]
    let bytes = match injection(Operation::Write) {
        Some(Inject::Error(e)) => return Err(io::Error::from_raw_os_error(e)),
        Some(Inject::Zero) => return Ok(0),
        Some(Inject::Limit(n)) => &bytes[..n.min(bytes.len())],
        None => bytes,
        _ => panic!("invalid write script"),
    };
    // SAFETY: held owned fd and immutable bounded slice; caller handles partial I/O.
    let n = unsafe { libc::write(fd, bytes.as_ptr().cast(), bytes.len()) };
    if n < 0 {
        Err(io::Error::last_os_error())
    } else {
        usize::try_from(n).map_err(io::Error::other)
    }
}
pub fn write_output(fd: &OwnedFd, bytes: &[u8]) -> Result<()> {
    let mut position = 0;
    while position < bytes.len() {
        match output_write(fd.as_raw_fd(), &bytes[position..]) {
            Ok(0) => return Err(Failure::Unpublished),
            Ok(n) => position += n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => return Err(Failure::Unpublished),
        }
    }
    Ok(())
}
fn sync_once(fd: RawFd, directory: bool) -> std::result::Result<(), i32> {
    #[cfg(not(test))]
    let _ = directory;
    #[cfg(test)]
    if let Some(action) = injection(if directory {
        Operation::SyncDirectory
    } else {
        Operation::SyncFile
    }) {
        return match action {
            Inject::Error(error) => Err(error),
            _ => panic!("invalid sync script"),
        };
    }
    // SAFETY: fsync borrows the descriptor, with no pointers or ownership transfer.
    // An invalid descriptor is rejected by the kernel and is used only in negative tests.
    if unsafe { libc::fsync(fd) } == 0 {
        Ok(())
    } else {
        Err(errno())
    }
}
fn sync_fd(fd: RawFd, directory: bool) -> Result<()> {
    loop {
        match sync_once(fd, directory) {
            Ok(()) => return Ok(()),
            Err(libc::EINTR) => continue,
            Err(_) => {
                return Err(if directory {
                    Failure::Uncertain
                } else {
                    Failure::Unpublished
                });
            }
        }
    }
}
pub fn sync_output(fd: &OwnedFd) -> Result<()> {
    sync_fd(fd.as_raw_fd(), false)
}
pub enum LinkFailure {
    Exists,
    Uncertain,
}
pub fn link_output(
    inputs: &CustodyInputs,
    fd: &OwnedFd,
    name: &str,
) -> std::result::Result<(), LinkFailure> {
    let source =
        cstring(&format!("self/fd/{}", fd.as_raw_fd())).map_err(|_| LinkFailure::Uncertain)?;
    let target = cstring(name).map_err(|_| LinkFailure::Uncertain)?;
    #[cfg(test)]
    let action = injection(Operation::Link);
    #[cfg(test)]
    if let Some(Inject::Error(e)) = action {
        return Err(if e == libc::EEXIST {
            LinkFailure::Exists
        } else {
            LinkFailure::Uncertain
        });
    }
    // SAFETY: fixed proc self-FD source naming this live owned anonymous file, held output fd,
    // generated basename. This is the sole proc publication exception; no syscall retry.
    let result = unsafe {
        libc::linkat(
            inputs.proc.fd.as_raw_fd(),
            source.as_ptr(),
            inputs.output_fd().map_err(|_| LinkFailure::Uncertain)?,
            target.as_ptr(),
            libc::AT_SYMLINK_FOLLOW,
        )
    };
    #[cfg(test)]
    match action {
        Some(Inject::AfterLinkError(e)) => {
            assert_eq!(result, 0);
            assert_ne!(e, libc::EEXIST);
            return Err(LinkFailure::Uncertain);
        }
        Some(Inject::AfterLinkPanic) => {
            assert_eq!(result, 0);
            panic!("scripted post-link panic");
        }
        None => {}
        _ => panic!("invalid link script"),
    }
    if result == 0 {
        Ok(())
    } else if errno() == libc::EEXIST {
        Err(LinkFailure::Exists)
    } else {
        Err(LinkFailure::Uncertain)
    }
}

pub fn verify_published(inputs: &CustodyInputs, file: &OwnedFd, name: &str) -> Result<()> {
    let destination = inside(inputs.output_fd()?, &cstring(name)?)?;
    let a = Metadata::read(file.as_raw_fd())?;
    let b = Metadata::read(destination.as_raw_fd())?;
    if a != b || a.links != 1 || a.mode != libc::S_IFREG | 0o600 || a.size != 291 {
        return Err(Failure::Uncertain);
    }
    no_acl(destination.as_raw_fd(), false)?;
    sync_fd(inputs.output_fd()?, true)?;
    let reached = walk_root(
        &inputs.config.output_path,
        inputs.config.output,
        &inputs.config,
    )?;
    let retained = inputs.output_chain.last().ok_or(Failure::Internal)?;
    let current = reached.last().ok_or(Failure::Internal)?;
    if current.metadata.dev != retained.metadata.dev
        || current.metadata.ino != retained.metadata.ino
        || current.metadata.mount != retained.metadata.mount
    {
        return Err(Failure::Uncertain);
    }
    Ok(())
}
#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use std::{
        cell::RefCell,
        collections::VecDeque,
        os::unix::fs::{DirBuilderExt, PermissionsExt},
        path::PathBuf,
    };
    struct Script {
        operations: Vec<Operation>,
        queue: VecDeque<(Operation, Inject)>,
    }
    thread_local! { static SCRIPT: RefCell<Option<Script>> = const { RefCell::new(None) }; }
    pub(crate) struct ScriptGuard;
    pub(crate) fn script(entries: &[(Operation, Inject)]) -> ScriptGuard {
        SCRIPT.with(|slot| {
            let mut slot = slot.borrow_mut();
            assert!(slot.is_none());
            *slot = Some(Script {
                operations: entries.iter().map(|e| e.0).collect(),
                queue: entries.iter().copied().collect(),
            });
        });
        ScriptGuard
    }
    pub(super) fn injection(operation: Operation) -> Option<Inject> {
        SCRIPT.with(|slot| {
            let mut slot = slot.borrow_mut();
            let state = slot.as_mut()?;
            // Unselected operations always use the real kernel. Every selected operation must
            // consume the exact next declared event, including real short-write requests.
            if !state.operations.contains(&operation) {
                return None;
            }
            let (expected, action) = state
                .queue
                .pop_front()
                .expect("unexpected scripted operation");
            assert_eq!(operation, expected, "script order mismatch");
            Some(action)
        })
    }
    impl Drop for ScriptGuard {
        fn drop(&mut self) {
            let state = SCRIPT
                .with(|s| s.borrow_mut().take())
                .expect("script disappeared");
            if !std::thread::panicking() {
                assert!(state.queue.is_empty(), "unconsumed scripted operations");
            }
        }
    }
    pub(crate) struct Fixture {
        pub(crate) inputs: CustodyInputs,
        pub(crate) path: PathBuf,
    }
    impl Fixture {
        pub(crate) fn new() -> Self {
            // Public disposable test files, never an installed launch record or real key.
            // A trusted-path positive syscall test needs owner-controlled ext4 ancestors;
            // /tmp's world-writable ancestor intentionally cannot pass production walk_root.
            let home = PathBuf::from(std::env::var_os("HOME").expect("test host HOME required"));
            let mut nonce = [0_u8; 16];
            getrandom::getrandom(&mut nonce).unwrap();
            let path = home.join(format!(".dgr-issuer-test-{}", crate::hex(&nonce)));
            std::fs::DirBuilder::new()
                .mode(0o700)
                .create(&path)
                .unwrap();
            let custody = path.join("custody");
            let output = path.join("output");
            for p in [&custody, &output] {
                std::fs::DirBuilder::new().mode(0o700).create(p).unwrap();
            }
            let (uid, gid) = identity().unwrap();
            fn root(path: &std::path::Path, uid: u32) -> Held {
                ancestor(
                    open_dir(libc::AT_FDCWD, &cstring(path.to_str().unwrap()).unwrap()).unwrap(),
                    uid,
                )
                .unwrap()
            }
            fn snapshot(root: &std::path::Path, name: &str, uid: u32) -> Snapshot {
                let p = root.join(name);
                std::fs::write(&p, b"PUBLIC-TEST-MARKER").unwrap();
                std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o400)).unwrap();
                let fd = std::fs::File::open(p).unwrap().into();
                bounded_snapshot(fd, 64, uid, 0o400).unwrap()
            }
            let launch = snapshot(&custody, "launch.fixture", uid);
            let registry = snapshot(&custody, "registry.fixture", uid);
            let profile = snapshot(&custody, "profile.fixture", uid);
            let envelope = snapshot(&custody, "envelope.fixture", uid);
            let custody_root = root(&custody, uid);
            let output_root = root(&output, uid);
            let triple = |m: &Metadata| (m.dev, m.ino, m.mount);
            let proc = trusted_proc(uid).unwrap();
            let config = ApprovedOperatorConfig {
                uid,
                gid,
                mount_ns: namespace(proc.fd.as_raw_fd(), c"self/ns/mnt").unwrap(),
                user_ns: namespace(proc.fd.as_raw_fd(), c"self/ns/user").unwrap(),
                root_mount: mount_id(open_dir(libc::AT_FDCWD, c"/").unwrap().as_raw_fd()).unwrap(),
                registry_hash: Sha256::digest(&registry.bytes).into(),
                profile_hash: Sha256::digest(&profile.bytes).into(),
                custody: triple(&custody_root.metadata),
                output: triple(&output_root.metadata),
                custody_path: custody.to_str().unwrap().to_owned(),
                output_path: output.to_str().unwrap().to_owned(),
            };
            Self {
                inputs: CustodyInputs {
                    config,
                    launch,
                    launch_chain: vec![],
                    proc,
                    custody_chain: vec![custody_root],
                    output_chain: vec![output_root],
                    registry,
                    profile,
                    envelope,
                },
                path,
            }
        }
        pub(crate) fn output(&self) -> PathBuf {
            self.path.join("output")
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.path).unwrap();
        }
    }

    fn public_pty() -> (OwnedFd, OwnedFd, TerminalSession) {
        let (mut master, mut slave) = (-1, -1);
        // SAFETY: writable descriptor outputs; null name/termios/winsize request system defaults.
        assert_eq!(
            unsafe {
                libc::openpty(
                    &mut master,
                    &mut slave,
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    std::ptr::null(),
                )
            },
            0
        );
        let master = owned(master, Failure::Terminal).unwrap();
        let slave = owned(slave, Failure::Terminal).unwrap();
        let observer = slave.try_clone().unwrap();
        let mut original = std::mem::MaybeUninit::<libc::termios>::zeroed();
        let mut mask = std::mem::MaybeUninit::<libc::sigset_t>::zeroed();
        // SAFETY: output buffers for live test-owned PTY and current thread's signal mask.
        assert_eq!(
            unsafe { libc::tcgetattr(slave.as_raw_fd(), original.as_mut_ptr()) },
            0
        );
        // SAFETY: query only, initialized sigset output; no signal-mask modification yet.
        assert_eq!(
            unsafe {
                libc::pthread_sigmask(libc::SIG_SETMASK, std::ptr::null(), mask.as_mut_ptr())
            },
            0
        );
        // SAFETY: both preceding queries succeeded.
        let (original, mask) = unsafe { (original.assume_init(), mask.assume_init()) };
        let mut terminal = TerminalSession {
            fd: Some(slave),
            signal: None,
            original,
            mask,
            modified: false,
            masked: false,
        };
        terminal.configure().unwrap();
        (master, observer, terminal)
    }
    fn observed_termios(fd: &OwnedFd) -> libc::termios {
        let mut value = std::mem::MaybeUninit::<libc::termios>::zeroed();
        // SAFETY: writable output for test-owned terminal; initialized on success.
        assert_eq!(
            unsafe { libc::tcgetattr(fd.as_raw_fd(), value.as_mut_ptr()) },
            0
        );
        // SAFETY: tcgetattr succeeded.
        unsafe { value.assume_init() }
    }
    #[test]
    fn real_pty_read_flush_restore_and_deadline() {
        use std::io::Write;
        let (master, observer, mut terminal) = public_pty();
        let saved = terminal.original;
        let raw = observed_termios(&observer);
        assert_eq!(raw.c_lflag & (libc::ECHO | libc::ICANON), 0);
        assert_eq!(raw.c_iflag & libc::ICRNL, 0);
        let mut writer = std::fs::File::from(master);
        writer
            .write_all("PUBLIC-é\u{7f}X\rqueued-confirmation\n".as_bytes())
            .unwrap();
        let mut clock = TrustedClock::new().unwrap();
        let deadline = clock.deadline(2).unwrap();
        let line = read_terminal_line(&terminal, 1024, deadline, &mut clock).unwrap();
        assert_eq!(&line[..], b"PUBLIC-X");
        terminal.restore().unwrap();
        assert!(termios_equal(&observed_termios(&observer), &saved));
        let (_master, _observer, mut terminal) = public_pty();
        assert!(matches!(
            read_terminal_line(&terminal, 1024, Duration::ZERO, &mut clock),
            Err(Failure::Cancelled)
        ));
        terminal.restore().unwrap();
    }
    #[test]
    fn handled_signal_and_late_signal_cancel_before_unlock() {
        for late in [false, true] {
            let (_master, observer, mut terminal) = public_pty();
            let saved = terminal.original;
            // SAFETY: target is this test thread, with SIGTERM blocked and signalfd installed.
            assert_eq!(
                unsafe { libc::pthread_kill(libc::pthread_self(), libc::SIGTERM) },
                0
            );
            if !late {
                let mut clock = TrustedClock::new().unwrap();
                let deadline = clock.deadline(2).unwrap();
                assert!(matches!(
                    read_terminal_line(&terminal, 1024, deadline, &mut clock),
                    Err(Failure::Cancelled)
                ));
            }
            assert_eq!(terminal.restore(), Err(Failure::Cancelled));
            assert!(termios_equal(&observed_termios(&observer), &saved));
        }
    }
    #[test]
    fn restoration_failure_overrides_cancellation() {
        let (_master, observer, mut terminal) = public_pty();
        let saved = terminal.original;
        // SAFETY: this thread owns the configured terminal and blocks SIGTERM for signalfd.
        assert_eq!(
            unsafe { libc::pthread_kill(libc::pthread_self(), libc::SIGTERM) },
            0
        );
        {
            let _guard = script(&[(Operation::SetTermios, Inject::Error(libc::EIO))]);
            assert_eq!(terminal.restore(), Err(Failure::Terminal));
        }
        // Explicit recovery of a public disposable PTY, never the operator's terminal.
        set_termios(observer.as_raw_fd(), &saved).unwrap();
    }
    #[test]
    fn actual_acl_and_fifo_are_rejected() {
        let f = Fixture::new();
        let fd = f.inputs.registry.held.fd.as_raw_fd();
        let mut acl = 2_u32.to_le_bytes().to_vec();
        for (tag, perm, id) in [
            (1_u16, 4_u16, u32::MAX),
            (2, 4, f.inputs.config.uid + 1),
            (4, 0, u32::MAX),
            (16, 4, u32::MAX),
            (32, 0, u32::MAX),
        ] {
            acl.extend_from_slice(&tag.to_le_bytes());
            acl.extend_from_slice(&perm.to_le_bytes());
            acl.extend_from_slice(&id.to_le_bytes());
        }
        // SAFETY: test-owned regular fixture fd, fixed ACL name, bounded initialized public bytes.
        assert_eq!(
            unsafe {
                libc::fsetxattr(
                    fd,
                    c"system.posix_acl_access".as_ptr(),
                    acl.as_ptr().cast(),
                    acl.len(),
                    0,
                )
            },
            0
        );
        assert_eq!(no_acl(fd, false), Err(Failure::Custody));
        let directory = f.inputs.custody_chain[0].fd.as_raw_fd();
        // SAFETY: test-owned directory fd; same complete public ACL value.
        assert_eq!(
            unsafe {
                libc::fsetxattr(
                    directory,
                    c"system.posix_acl_default".as_ptr(),
                    acl.as_ptr().cast(),
                    acl.len(),
                    0,
                )
            },
            0
        );
        assert_eq!(no_acl(directory, true), Err(Failure::Custody));
        let name = cstring(f.path.join("output/fifo").to_str().unwrap()).unwrap();
        // SAFETY: unique fixture path under owned directory, creates only a disposable FIFO.
        assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);
        let fifo = inside(f.inputs.output_fd().unwrap(), c"fifo").unwrap();
        assert!(bounded_snapshot(fifo, 64, f.inputs.config.uid, 0o600).is_err());
    }
    fn public_envelope(length: usize) -> Vec<u8> {
        use std::io::Write;
        // Only the already disclosed 0x11 seed; encrypted public fixture, not enrollment.
        let mut recipient = age::scrypt::Recipient::new(age::secrecy::SecretString::from(
            "PUBLIC-FIXTURE-NOT-A-CREDENTIAL".to_owned(),
        ));
        recipient.set_work_factor(18);
        let encryptor =
            age::Encryptor::with_recipients(std::iter::once(&recipient as &dyn age::Recipient))
                .unwrap();
        let mut stream = encryptor.wrap_output(Vec::new()).unwrap();
        stream.write_all(&vec![0x11; length]).unwrap();
        stream.finish().unwrap()
    }
    #[test]
    fn authenticated_age_length_password_and_ciphertext() {
        let good = public_envelope(32);
        let password = || Zeroizing::new(b"PUBLIC-FIXTURE-NOT-A-CREDENTIAL".to_vec());
        let key = unlock_once(&good, password()).unwrap();
        assert_eq!(
            key.verifying_key(),
            ed25519_dalek::SigningKey::from_bytes(&[0x11; 32]).verifying_key()
        );
        assert!(profile::accept_public_key(key.verifying_key().as_bytes()).is_err());
        assert!(unlock_once(&public_envelope(31), password()).is_err());
        assert!(unlock_once(&public_envelope(33), password()).is_err());
        assert!(unlock_once(&good, Zeroizing::new(b"PUBLIC-WRONG-PASSWORD".to_vec())).is_err());
        let mut tampered = good;
        tampered[213] ^= 1;
        assert!(unlock_once(&tampered, password()).is_err());
    }
    #[test]
    fn procfs_exception_does_not_relax_acl_errors() {
        let (uid, _) = identity().unwrap();
        let proc = trusted_proc(uid).unwrap();
        proc.recheck(uid).unwrap();
        assert_eq!(
            acl_size(proc.fd.as_raw_fd(), c"system.posix_acl_access"),
            Err(libc::EOPNOTSUPP)
        );
        assert_eq!(no_acl(proc.fd.as_raw_fd(), true), Err(Failure::Custody));
        for error in [libc::EOPNOTSUPP, libc::EPERM, libc::EBADF, libc::ERANGE] {
            let _guard = script(&[(Operation::Acl, Inject::Error(error))]);
            assert_eq!(no_acl(proc.fd.as_raw_fd(), false), Err(Failure::Custody));
        }
        for outcome in [Inject::Zero, Inject::Limit(24)] {
            let _guard = script(&[(Operation::Acl, outcome)]);
            assert_eq!(no_acl(proc.fd.as_raw_fd(), false), Err(Failure::Custody));
        }
        let _guard = script(&[
            (Operation::Acl, Inject::Error(libc::ENODATA)),
            (Operation::Acl, Inject::Error(libc::ENODATA)),
        ]);
        assert!(no_acl(-1, true).is_ok()); // Injected ABI observations; real invalid FDs are rejected above.
    }
    #[test]
    fn procfs_rejects_wrong_owner_mode_type_identity_and_filesystem() {
        let (uid, _) = identity().unwrap();
        let proc = trusted_proc(uid).unwrap();
        assert!(proc.recheck(uid + 1).is_err());
        for (mode, owner) in [
            (libc::S_IFDIR | 0o777, 0),
            (libc::S_IFREG | 0o555, 0),
            (libc::S_IFDIR | 0o555, uid),
        ] {
            let mut m = proc.metadata.clone();
            m.mode = mode;
            m.uid = owner;
            assert!(validate_proc(proc.fd.as_raw_fd(), &m, uid).is_err());
        }
        let root = open_dir(libc::AT_FDCWD, c"/").unwrap();
        let m = Metadata::read(root.as_raw_fd()).unwrap();
        assert!(validate_proc(root.as_raw_fd(), &m, uid).is_err());
        let mut proc = proc;
        proc.metadata.ino += 1;
        assert!(proc.recheck(uid).is_err());
    }
    #[test]
    fn real_custody_syscalls_reject_aliases_and_mutation() {
        let f = Fixture::new();
        f.inputs.recheck().unwrap();
        let root = f.inputs.custody_chain[0].fd.as_raw_fd();
        let p = f.path.join("custody");
        std::os::unix::fs::symlink("profile.fixture", p.join("symlink")).unwrap();
        assert!(inside(root, c"symlink").is_err());
        assert!(inside(root, c"../output").is_err());
        std::fs::hard_link(p.join("profile.fixture"), p.join("hardlink")).unwrap();
        assert!(
            bounded_snapshot(
                inside(root, c"hardlink").unwrap(),
                64,
                f.inputs.config.uid,
                0o400
            )
            .is_err()
        );
        assert!(f.inputs.recheck().is_err());
        std::fs::remove_file(p.join("hardlink")).unwrap();
        // Retained metadata catches content/mode changes even when held descriptors survive rename.
        std::fs::set_permissions(
            p.join("profile.fixture"),
            std::fs::Permissions::from_mode(0o600),
        )
        .unwrap();
        assert!(f.inputs.recheck().is_err());
    }
    #[test]
    fn envelope_exact_shape_and_canonical_base64() {
        let mut envelope = format!(
            "age-encryption.org/v1\n-> scrypt {} 18\n{}\n--- {}\n",
            STANDARD_NO_PAD.encode([0; 16]),
            STANDARD_NO_PAD.encode([0; 32]),
            STANDARD_NO_PAD.encode([0; 32])
        )
        .into_bytes();
        envelope.extend_from_slice(&[0; 64]);
        assert_eq!(envelope.len(), 214);
        assert!(envelope_shape(&envelope).is_ok());
        for n in 0..214 {
            assert!(envelope_shape(&envelope[..n]).is_err());
        }
        for index in [0, 21, 22, 31, 54, 55, 56, 57, 101, 102, 105, 149] {
            let mut bad = envelope.clone();
            bad[index] = b'!';
            assert!(envelope_shape(&bad).is_err(), "offset {index}");
        }
        for index in [53, 100, 148] {
            let mut bad = envelope.clone();
            bad[index] = b'B';
            assert!(envelope_shape(&bad).is_err());
        }
        let mut extra = envelope.clone();
        extra.push(0);
        assert!(envelope_shape(&extra).is_err());
    }
    #[test]
    fn launch_encoding_namespace_mount_and_envelope_binding() {
        let mut f = Fixture::new();
        let c = &f.inputs.config;
        let mut bytes = b"DGRLCH1\0".to_vec();
        bytes.extend_from_slice(&1_u16.to_be_bytes());
        bytes.extend_from_slice(&0_u16.to_be_bytes());
        let length = 212 + c.custody_path.len() + c.output_path.len();
        bytes.extend_from_slice(&u32::try_from(length).unwrap().to_be_bytes());
        bytes.extend_from_slice(&c.uid.to_be_bytes());
        bytes.extend_from_slice(&c.gid.to_be_bytes());
        for n in [
            c.mount_ns.0,
            c.mount_ns.1,
            c.user_ns.0,
            c.user_ns.1,
            c.root_mount,
        ] {
            bytes.extend_from_slice(&n.to_be_bytes());
        }
        bytes.extend_from_slice(&c.registry_hash);
        bytes.extend_from_slice(&c.profile_hash);
        for n in [
            c.custody.0,
            c.custody.1,
            c.custody.2,
            c.output.0,
            c.output.1,
            c.output.2,
        ] {
            bytes.extend_from_slice(&n.to_be_bytes());
        }
        bytes.extend_from_slice(&profile::fixture_inventory_hash());
        bytes.extend_from_slice(&u16::try_from(c.custody_path.len()).unwrap().to_be_bytes());
        bytes.extend_from_slice(&u16::try_from(c.output_path.len()).unwrap().to_be_bytes());
        bytes.extend_from_slice(c.custody_path.as_bytes());
        bytes.extend_from_slice(c.output_path.as_bytes());
        assert_eq!(bytes.len(), length);
        assert!(parse_launch(&bytes).is_ok());
        for offset in [16, 20] {
            let mut bad = bytes.clone();
            bad[offset..offset + 4].copy_from_slice(&0_u32.to_be_bytes());
            assert!(matches!(parse_launch(&bad), Err(Failure::Launch)));
        }
        for index in [0, 8, 10, 12, 176, 208, 210] {
            let mut bad = bytes.clone();
            bad[index] ^= 1;
            assert!(parse_launch(&bad).is_err());
        }
        for n in 0..bytes.len() {
            assert!(parse_launch(&bytes[..n]).is_err());
        }
        assert!(verify_envelope_binding(b"PUBLIC", &Sha256::digest(b"PUBLIC").into()).is_ok());
        assert_eq!(
            verify_envelope_binding(b"OTHER", &Sha256::digest(b"PUBLIC").into()),
            Err(Failure::Custody)
        );
        f.inputs.config.mount_ns.1 += 1;
        assert!(f.inputs.recheck().is_err());
        f.inputs.config.mount_ns.1 -= 1;
        f.inputs.config.user_ns.1 += 1;
        assert!(f.inputs.recheck().is_err());
        f.inputs.config.user_ns.1 -= 1;
        f.inputs.config.root_mount += 1;
        assert!(
            walk_root(
                &f.inputs.config.output_path,
                f.inputs.config.output,
                &f.inputs.config
            )
            .is_err()
        );
    }
    #[test]
    fn passphrase_bounds_utf8_and_erase() {
        let mut buffer = [0_u8; 1024];
        let mut n = 0;
        for b in "aé".bytes() {
            assert!(!edit_input(&mut buffer, &mut n, b, 1024).unwrap());
        }
        assert!(!edit_input(&mut buffer, &mut n, 127, 1024).unwrap());
        assert_eq!(&buffer[..n], b"a");
        assert!(edit_input(&mut buffer, &mut n, b'\n', 1024).unwrap());
        n = 0;
        for _ in 0..1024 {
            edit_input(&mut buffer, &mut n, b'x', 1024).unwrap();
        }
        assert!(edit_input(&mut buffer, &mut n, b'\r', 1024).unwrap());
        assert_eq!(
            edit_input(&mut buffer, &mut n, b'x', 1024),
            Err(Failure::Cancelled)
        );
        for b in [0, 4, 27, 0xff] {
            n = 0;
            assert_eq!(
                edit_input(&mut buffer, &mut n, b, 1024),
                Err(Failure::Cancelled)
            );
        }
    }
    #[test]
    fn identity_rejects_root_primary_group_and_mismatched_ids() {
        assert_eq!(
            validate_identity(1000, 1000, 1000, 1000, 1000, 1000),
            Ok((1000, 1000))
        );
        for ids in [
            (0, 0, 0, 1000, 1000, 1000),
            (1000, 1000, 1000, 0, 0, 0),
            (1000, 1001, 1000, 1000, 1000, 1000),
            (1000, 1000, 1001, 1000, 1000, 1000),
            (1000, 1000, 1000, 1000, 1001, 1000),
            (1000, 1000, 1000, 1000, 1000, 1001),
        ] {
            assert_eq!(
                validate_identity(ids.0, ids.1, ids.2, ids.3, ids.4, ids.5),
                Err(Failure::Launch)
            );
        }
    }
    #[test]
    fn sync_errors_match_real_syscalls_and_retry_interruption() {
        for (directory, op, expected) in [
            (false, Operation::SyncFile, Failure::Unpublished),
            (true, Operation::SyncDirectory, Failure::Uncertain),
        ] {
            // A real EBADF exercises production classification, without the outer publication map.
            assert_eq!(sync_fd(-1, directory), Err(expected));
            for error in [libc::EBADF, libc::EIO] {
                let _guard =
                    script(&[(op, Inject::Error(libc::EINTR)), (op, Inject::Error(error))]);
                assert_eq!(sync_fd(-1, directory), Err(expected));
            }
        }
    }
    #[test]
    fn launch_rejects_truncation() {
        for n in 0..214 {
            assert!(parse_launch(&vec![0; n]).is_err());
        }
    }
}
