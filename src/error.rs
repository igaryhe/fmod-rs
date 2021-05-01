use thiserror::Error;

pub enum FModError {
    BadCommand,
    ChannelAlloc,
    Dma,
    DspConnection,
    DspDontProcess,
    DspFormat,
    DspInUse,
    DspNotFound,
    DspReserved,
    DspSilence,
    DspType,
    FileBad,
    FileCouldnotSeek,
}

pub enum ChannelError {
    Alloc,
    Stolen,
}

pub enum DspError {
    Connection,
    DontProcess,
    Format,
    InUse,
    NotFound,
    Reserved,
    Silence,
    Type,
}

pub enum FileError {
    Bad,
    CouldnotSeek,
    DiskEjected,
    Eof,
    EndOfData,
    NotFound,
}

pub enum HttpError {
    Http,
    Access,
    ProxyAuth,
    ServerError,
    Timeout,
}

pub enum InvalidError {
    Float,
    Handle,
    Param,
    Position,
    Speaker,
    SyncPoint,
    Thread,
    Vector,
}

pub enum NetError {
    Connect,
    SocketError,
    Url,
    WouldBlock,
}

pub enum OutputError {
    Allocated,
    CreateBuffer,
    DriverCall,
    Format,
    Init,
    NoDrivers,
}

pub enum PluginError {
    Plugin,
    Missing,
    Resource,
    Version,
}

pub enum ReverbError {
    ChannelGroup,
    Instance,
}

pub enum SubsoundError {
    Subsounds,
    Allocated,
    CantMove,
}

pub enum EventError {
    AlreadyLoaded,
    LiveupdateBusy,
    LiveupdateMismatch,
    LiveupdateTimeout,
    NotFound,
}

pub enum StudioError {
    Uninitialized,
    NotLoaded,
}
