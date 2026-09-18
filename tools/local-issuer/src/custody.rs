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
        // SAFETY: readable live fd, static NUL-terminated name, null/zero requests size only.
        let n = unsafe { libc::fgetxattr(fd, name.as_ptr(), std::ptr::null_mut(), 0) };
        if n >= 0 || errno() != libc::ENODATA {
            return Err(Failure::Custody);
        }
    }
    Ok(())
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
        || r == 0
        || r != e
        || r != s
        || gr != ge
        || gr != gs
    {
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
fn lower_limit(resource: libc::__rlimit_resource_t, ceiling: u64) -> Result<()> {
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
    Ok(())
}
pub fn harden_process() -> Result<()> {
    identity()?;
    no_capabilities()?;
    lower_limit(libc::RLIMIT_CORE, 0)?;
    lower_limit(libc::RLIMIT_AS, 512 * 1024 * 1024)?;
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
fn trusted_proc(uid: u32) -> Result<Held> {
    let held = Held::new(open_dir(libc::AT_FDCWD, c"/proc")?, true)?;
    filesystem(held.fd.as_raw_fd(), 0x9fa0)?;
    let mut target = [0_u8; 32];
    // SAFETY: fixed proc link, bounded writable buffer; no NUL termination assumed.
    let n = unsafe {
        libc::readlinkat(
            held.fd.as_raw_fd(),
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
                held.fd.as_raw_fd(),
                c"self".as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
            )
        },
        Failure::Launch,
    )?;
    if stat(self_fd.as_raw_fd())?.st_uid != uid {
        return Err(Failure::Launch);
    }
    Ok(held)
}
pub struct CustodyInputs {
    pub config: ApprovedOperatorConfig,
    launch: Snapshot,
    launch_chain: Vec<Held>,
    proc: Held,
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
        || &b[..21] != b"age-encryption.org/v1\n"
        || &b[21..32] != b"-> scrypt "
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
        let mut mask = std::mem::MaybeUninit::<libc::sigset_t>::uninit();
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
        // SAFETY: sigemptyset initializes the entire valid signal-set object before use.
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
            || unsafe { libc::tcsetattr(self.raw()?, libc::TCSANOW, &raw) } != 0
        {
            return Err(Failure::Terminal);
        }
        Ok(())
    }
    fn restore(&mut self) -> Result<()> {
        let mut failed = false;
        if self.modified {
            let fd = self.raw()?;
            // SAFETY: held terminal and exact saved termios; attempt restore even if flush fails.
            if unsafe { libc::tcflush(fd, libc::TCIFLUSH) } != 0 {
                failed = true
            }
            // SAFETY: original termios copied from this descriptor before modifications.
            if unsafe { libc::tcsetattr(fd, libc::TCSANOW, &self.original) } != 0 {
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
                    Ok(128) => {}
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
pub fn terminal_input(
    prompt: &[u8],
    maximum: usize,
    clock: &mut TrustedClock,
) -> Result<Zeroizing<Vec<u8>>> {
    if maximum > 1024 {
        return Err(Failure::Internal);
    }
    let mut terminal = TerminalSession::open()?;
    let result = (|| {
        let deadline = clock.deadline(60)?;
        terminal.write_prompt(prompt, deadline, clock)?;
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
    })();
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
pub fn write_output(fd: &OwnedFd, bytes: &[u8]) -> Result<()> {
    let mut position = 0;
    while position < bytes.len() {
        // SAFETY: live owned descriptor and immutable input slice; handles short writes explicitly.
        let n = unsafe {
            libc::write(
                fd.as_raw_fd(),
                bytes[position..].as_ptr().cast(),
                bytes.len() - position,
            )
        };
        if n < 0 {
            if errno() == libc::EINTR {
                continue;
            }
            return Err(Failure::Unpublished);
        }
        if n == 0 {
            return Err(Failure::Unpublished);
        }
        position += usize::try_from(n).map_err(|_| Failure::Unpublished)?;
    }
    Ok(())
}
pub fn sync_output(fd: &OwnedFd) -> Result<()> {
    loop {
        // SAFETY: live descriptor; fsync has no pointer/ownership transfer.
        if unsafe { libc::fsync(fd.as_raw_fd()) } == 0 {
            return Ok(());
        }
        if errno() != libc::EINTR {
            return Err(Failure::Unpublished);
        }
    }
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
    // SAFETY: fixed proc self-FD source naming this live owned anonymous file, held output fd,
    // generated basename. Only this operation permits the proc magic-link exception. Never retry.
    if unsafe {
        libc::linkat(
            inputs.proc.fd.as_raw_fd(),
            source.as_ptr(),
            inputs.output_fd().map_err(|_| LinkFailure::Uncertain)?,
            target.as_ptr(),
            libc::AT_SYMLINK_FOLLOW,
        )
    } == 0
    {
        return Ok(());
    }
    if errno() == libc::EEXIST {
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
    // SAFETY: held output directory fd is kept live across synchronization.
    if unsafe { libc::fsync(inputs.output_fd()?) } != 0 {
        return Err(Failure::Uncertain);
    }
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
mod tests {
    use super::*;
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
    fn launch_rejects_truncation() {
        for n in 0..214 {
            assert!(parse_launch(&vec![0; n]).is_err());
        }
    }
}
