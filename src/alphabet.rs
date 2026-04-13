/// URL safe symbols.
///
/// An array of characters which can be safely used in urls.
/// Alphabet default for nanoid. Is alphabet by default for nanoid
///
/// # Example
///
/// ```
/// let id = nanoid::nanoid!(10, &nanoid::alphabet::SAFE);
/// ```
pub const SAFE: [char; 64] = [
    '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Lowercase hexadecimal alphabet (`0-9`, `a-f`).
///
/// # Example
///
/// ```
/// let id = nanoid::nanoid!(10, &nanoid::alphabet::HEX_LOWERCASE);
/// ```
pub const HEX_LOWERCASE: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Uppercase hexadecimal alphabet (`0-9`, `A-F`).
///
/// # Example
///
/// ```
/// let id = nanoid::nanoid!(10, &nanoid::alphabet::HEX_UPPERCASE);
/// ```
pub const HEX_UPPERCASE: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];
