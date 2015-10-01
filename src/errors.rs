// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

/// Intended for converting NFS Errors into numeric codes for propagating some error information
/// across FFI boundaries and specially to C.
pub const NFS_ERROR_START_RANGE: i32 = ::safe_client::errors::CLIENT_ERROR_START_RANGE - 500;

/// NFS Errors
#[allow(variant_size_differences)]
pub enum NfsError {
    /// Client Error
    ClientError(::safe_client::errors::ClientError),
    /// If Directory already exists with the same name in the same level
    DirectoryAlreadyExistsWithSameName,
    /// Destination is Same as the Source
    DestinationAndSourceAreSame,
    /// Directory not found
    DirectoryNotFound,
    /// File Already exists with the same name in a directory
    FileAlreadyExistsWithSameName,
    /// File does not match with the existing file in the directory listing
    FileDoesNotMatch,
    /// File not found
    FileNotFound,
    /// Invalid byte range specified
    InvalidRangeSpecified,
    /// Validation error - if the field passed as parameter is not valid
    ParameterIsNotValid,
    /// Unexpected error
    Unexpected(String),
}

impl From<::safe_client::errors::ClientError> for NfsError {
    fn from(error: ::safe_client::errors::ClientError) -> NfsError {
        NfsError::ClientError(error)
    }
}

impl<'a> From<&'a str> for NfsError {
    fn from(error: &'a str) -> NfsError {
        NfsError::Unexpected(error.to_string())
    }
}

impl Into<i32> for NfsError {
    fn into(self) -> i32 {
        match self {
            NfsError::ClientError(error)                    => error.into(),
            NfsError::DirectoryAlreadyExistsWithSameName    => NFS_ERROR_START_RANGE - 1,
            NfsError::DestinationAndSourceAreSame           => NFS_ERROR_START_RANGE - 2,
            NfsError::DirectoryNotFound                     => NFS_ERROR_START_RANGE - 3,
            NfsError::FileAlreadyExistsWithSameName         => NFS_ERROR_START_RANGE - 4,
            NfsError::FileDoesNotMatch                      => NFS_ERROR_START_RANGE - 5,
            NfsError::FileNotFound                          => NFS_ERROR_START_RANGE - 6,
            NfsError::InvalidRangeSpecified                 => NFS_ERROR_START_RANGE - 7,
            NfsError::ParameterIsNotValid                   => NFS_ERROR_START_RANGE - 8,
            NfsError::Unexpected(_)                         => NFS_ERROR_START_RANGE - 9,
        }
    }
}

impl ::std::fmt::Debug for NfsError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            NfsError::ClientError(ref error)                => write!(f, "NfsError::ClientError -> {:?}", error),
            NfsError::DirectoryAlreadyExistsWithSameName    => write!(f, "NfsError::DirectoryAlreadyExistsWithSameName"),
            NfsError::DestinationAndSourceAreSame           => write!(f, "NfsError::DestinationAndSourceAreSame"),
            NfsError::DirectoryNotFound                     => write!(f, "NfsError::DirectoryNotFound"),
            NfsError::FileAlreadyExistsWithSameName         => write!(f, "NfsError::FileAlreadyExistsWithSameName"),
            NfsError::FileDoesNotMatch                      => write!(f, "NfsError::FileDoesNotMatch"),
            NfsError::FileNotFound                          => write!(f, "NfsError::FileNotFound"),
            NfsError::InvalidRangeSpecified                 => write!(f, "NfsError::InvalidRangeSpecified"),
            NfsError::ParameterIsNotValid                   => write!(f, "NfsError::ParameterIsNotValid"),
            NfsError::Unexpected(ref error)                 => write!(f, "NfsError::Unexpected -> {:?}", error),
        }
    }
}
