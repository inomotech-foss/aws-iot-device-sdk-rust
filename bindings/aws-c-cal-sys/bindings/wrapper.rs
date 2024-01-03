/* automatically generated by rust-bindgen 0.69.1 */

pub type aws_cal_errors = ::core::ffi::c_uint;
pub type aws_cal_log_subject = ::core::ffi::c_uint;
pub type aws_ecc_curve_name = ::core::ffi::c_uint;
pub type aws_ecc_key_pair_destroy_fn =
    ::core::option::Option<unsafe extern "C" fn(key_pair: *mut aws_ecc_key_pair)>;
pub type aws_ecc_key_pair_sign_message_fn = ::core::option::Option<
    unsafe extern "C" fn(
        key_pair: *const aws_ecc_key_pair,
        message: *const aws_byte_cursor,
        signature_output: *mut aws_byte_buf,
    ) -> ::core::ffi::c_int,
>;
pub type aws_ecc_key_pair_derive_public_key_fn = ::core::option::Option<
    unsafe extern "C" fn(key_pair: *mut aws_ecc_key_pair) -> ::core::ffi::c_int,
>;
pub type aws_ecc_key_pair_verify_signature_fn = ::core::option::Option<
    unsafe extern "C" fn(
        signer: *const aws_ecc_key_pair,
        message: *const aws_byte_cursor,
        signature: *const aws_byte_cursor,
    ) -> ::core::ffi::c_int,
>;
pub type aws_ecc_key_pair_signature_length_fn =
    ::core::option::Option<unsafe extern "C" fn(signer: *const aws_ecc_key_pair) -> usize>;
pub type aws_hash_new_fn =
    ::core::option::Option<unsafe extern "C" fn(allocator: *mut aws_allocator) -> *mut aws_hash>;
pub type aws_hmac_new_fn = ::core::option::Option<
    unsafe extern "C" fn(
        allocator: *mut aws_allocator,
        secret: *const aws_byte_cursor,
    ) -> *mut aws_hmac,
>;
pub type aws_rsa_encryption_algorithm = ::core::ffi::c_uint;
pub type aws_rsa_signature_algorithm = ::core::ffi::c_uint;
pub type aws_rsa_key_export_format = ::core::ffi::c_uint;
pub type aws_aes_cbc_256_new_fn = ::core::option::Option<
    unsafe extern "C" fn(
        allocator: *mut aws_allocator,
        key: *const aws_byte_cursor,
        iv: *const aws_byte_cursor,
    ) -> *mut aws_symmetric_cipher,
>;
pub type aws_aes_ctr_256_new_fn = ::core::option::Option<
    unsafe extern "C" fn(
        allocator: *mut aws_allocator,
        key: *const aws_byte_cursor,
        iv: *const aws_byte_cursor,
    ) -> *mut aws_symmetric_cipher,
>;
pub type aws_aes_gcm_256_new_fn = ::core::option::Option<
    unsafe extern "C" fn(
        allocator: *mut aws_allocator,
        key: *const aws_byte_cursor,
        iv: *const aws_byte_cursor,
        aad: *const aws_byte_cursor,
        decryption_tag: *const aws_byte_cursor,
    ) -> *mut aws_symmetric_cipher,
>;
pub type aws_aes_keywrap_256_new_fn = ::core::option::Option<
    unsafe extern "C" fn(
        allocator: *mut aws_allocator,
        key: *const aws_byte_cursor,
    ) -> *mut aws_symmetric_cipher,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aws_ecc_key_pair_vtable {
    pub destroy: aws_ecc_key_pair_destroy_fn,
    pub derive_pub_key: aws_ecc_key_pair_derive_public_key_fn,
    pub sign_message: aws_ecc_key_pair_sign_message_fn,
    pub verify_signature: aws_ecc_key_pair_verify_signature_fn,
    pub signature_length: aws_ecc_key_pair_signature_length_fn,
}
#[repr(C)]
pub struct aws_ecc_key_pair {
    pub allocator: *mut aws_allocator,
    pub ref_count: aws_atomic_var,
    pub curve_name: aws_ecc_curve_name,
    pub key_buf: aws_byte_buf,
    pub pub_x: aws_byte_buf,
    pub pub_y: aws_byte_buf,
    pub priv_d: aws_byte_buf,
    pub vtable: *mut aws_ecc_key_pair_vtable,
    pub impl_: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aws_hash_vtable {
    pub alg_name: *const ::core::ffi::c_char,
    pub provider: *const ::core::ffi::c_char,
    pub destroy: ::core::option::Option<unsafe extern "C" fn(hash: *mut aws_hash)>,
    pub update: ::core::option::Option<
        unsafe extern "C" fn(
            hash: *mut aws_hash,
            buf: *const aws_byte_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub finalize: ::core::option::Option<
        unsafe extern "C" fn(hash: *mut aws_hash, out: *mut aws_byte_buf) -> ::core::ffi::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aws_hash {
    pub allocator: *mut aws_allocator,
    pub vtable: *mut aws_hash_vtable,
    pub digest_size: usize,
    pub good: bool,
    pub impl_: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aws_hmac_vtable {
    pub alg_name: *const ::core::ffi::c_char,
    pub provider: *const ::core::ffi::c_char,
    pub destroy: ::core::option::Option<unsafe extern "C" fn(hmac: *mut aws_hmac)>,
    pub update: ::core::option::Option<
        unsafe extern "C" fn(
            hmac: *mut aws_hmac,
            buf: *const aws_byte_cursor,
        ) -> ::core::ffi::c_int,
    >,
    pub finalize: ::core::option::Option<
        unsafe extern "C" fn(hmac: *mut aws_hmac, out: *mut aws_byte_buf) -> ::core::ffi::c_int,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aws_hmac {
    pub allocator: *mut aws_allocator,
    pub vtable: *mut aws_hmac_vtable,
    pub digest_size: usize,
    pub good: bool,
    pub impl_: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aws_rsa_key_pair {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct aws_symmetric_cipher {
    _unused: [u8; 0],
}
pub const AWS_C_CAL_PACKAGE_ID: u32 = 7;
pub const AWS_SHA256_LEN: u32 = 32;
pub const AWS_SHA1_LEN: u32 = 20;
pub const AWS_MD5_LEN: u32 = 16;
pub const AWS_SHA256_HMAC_LEN: u32 = 32;
pub const AWS_AES_256_CIPHER_BLOCK_SIZE: u32 = 16;
pub const AWS_AES_256_KEY_BIT_LEN: u32 = 256;
pub const AWS_AES_256_KEY_BYTE_LEN: u32 = 32;
pub const AWS_ERROR_CAL_SIGNATURE_VALIDATION_FAILED: aws_cal_errors = 7168;
pub const AWS_ERROR_CAL_MISSING_REQUIRED_KEY_COMPONENT: aws_cal_errors = 7169;
pub const AWS_ERROR_CAL_INVALID_KEY_LENGTH_FOR_ALGORITHM: aws_cal_errors = 7170;
pub const AWS_ERROR_CAL_UNKNOWN_OBJECT_IDENTIFIER: aws_cal_errors = 7171;
pub const AWS_ERROR_CAL_MALFORMED_ASN1_ENCOUNTERED: aws_cal_errors = 7172;
pub const AWS_ERROR_CAL_MISMATCHED_DER_TYPE: aws_cal_errors = 7173;
pub const AWS_ERROR_CAL_UNSUPPORTED_ALGORITHM: aws_cal_errors = 7174;
pub const AWS_ERROR_CAL_BUFFER_TOO_LARGE_FOR_ALGORITHM: aws_cal_errors = 7175;
pub const AWS_ERROR_CAL_INVALID_CIPHER_MATERIAL_SIZE_FOR_ALGORITHM: aws_cal_errors = 7176;
pub const AWS_ERROR_CAL_DER_UNSUPPORTED_NEGATIVE_INT: aws_cal_errors = 7177;
pub const AWS_ERROR_CAL_UNSUPPORTED_KEY_FORMAT: aws_cal_errors = 7178;
pub const AWS_ERROR_CAL_CRYPTO_OPERATION_FAILED: aws_cal_errors = 7179;
pub const AWS_ERROR_CAL_END_RANGE: aws_cal_errors = 8191;
pub const AWS_LS_CAL_GENERAL: aws_cal_log_subject = 7168;
pub const AWS_LS_CAL_ECC: aws_cal_log_subject = 7169;
pub const AWS_LS_CAL_HASH: aws_cal_log_subject = 7170;
pub const AWS_LS_CAL_HMAC: aws_cal_log_subject = 7171;
pub const AWS_LS_CAL_DER: aws_cal_log_subject = 7172;
pub const AWS_LS_CAL_LIBCRYPTO_RESOLVE: aws_cal_log_subject = 7173;
pub const AWS_LS_CAL_RSA: aws_cal_log_subject = 7174;
pub const AWS_LS_CAL_LAST: aws_cal_log_subject = 8191;
pub const AWS_CAL_ECDSA_P256: aws_ecc_curve_name = 0;
pub const AWS_CAL_ECDSA_P384: aws_ecc_curve_name = 1;
pub const AWS_CAL_RSA_ENCRYPTION_PKCS1_5: aws_rsa_encryption_algorithm = 0;
pub const AWS_CAL_RSA_ENCRYPTION_OAEP_SHA256: aws_rsa_encryption_algorithm = 1;
pub const AWS_CAL_RSA_ENCRYPTION_OAEP_SHA512: aws_rsa_encryption_algorithm = 2;
pub const AWS_CAL_RSA_SIGNATURE_PKCS1_5_SHA256: aws_rsa_signature_algorithm = 0;
pub const AWS_CAL_RSA_SIGNATURE_PSS_SHA256: aws_rsa_signature_algorithm = 1;
pub const AWS_CAL_RSA_KEY_EXPORT_PKCS1: aws_rsa_key_export_format = 0;
extern "C" {
    #[doc = " Case-insensitive hash function for array containing ASCII or UTF-8 text."]
    pub fn aws_hash_array_ignore_case(array: *const ::core::ffi::c_void, len: usize) -> u64;
    #[doc = " Case-insensitive hash function for aws_byte_cursors stored in an aws_hash_table.\n For case-sensitive hashing, use aws_hash_byte_cursor_ptr()."]
    pub fn aws_hash_byte_cursor_ptr_ignore_case(item: *const ::core::ffi::c_void) -> u64;
    pub fn aws_cal_library_init(allocator: *mut aws_allocator);
    pub fn aws_cal_library_clean_up();
    pub fn aws_cal_thread_clean_up();
    #[doc = " Adds one to an ecc key pair's ref count."]
    pub fn aws_ecc_key_pair_acquire(key_pair: *mut aws_ecc_key_pair);
    #[doc = " Subtracts one from an ecc key pair's ref count.  If ref count reaches zero, the key pair is destroyed."]
    pub fn aws_ecc_key_pair_release(key_pair: *mut aws_ecc_key_pair);
    #[doc = " Creates an Elliptic Curve private key that can be used for signing.\n Returns a new instance of aws_ecc_key_pair if the key was successfully built.\n Otherwise returns NULL. Note: priv_key::len must match the appropriate length\n for the selected curve_name."]
    pub fn aws_ecc_key_pair_new_from_private_key(
        allocator: *mut aws_allocator,
        curve_name: aws_ecc_curve_name,
        priv_key: *const aws_byte_cursor,
    ) -> *mut aws_ecc_key_pair;
    #[doc = " Creates an Elliptic Curve public/private key pair that can be used for signing and verifying.\n Returns a new instance of aws_ecc_key_pair if the key was successfully built.\n Otherwise returns NULL.\n Note: On Apple platforms this function is only supported on MacOS. This is\n due to usage of SecItemExport, which is only available on MacOS 10.7+\n (yes, MacOS only and no other Apple platforms). There are alternatives for\n ios and other platforms, but they are ugly to use. Hence for now it only\n supports this call on MacOS."]
    pub fn aws_ecc_key_pair_new_generate_random(
        allocator: *mut aws_allocator,
        curve_name: aws_ecc_curve_name,
    ) -> *mut aws_ecc_key_pair;
    #[doc = " Creates an Elliptic Curve public key that can be used for verifying.\n Returns a new instance of aws_ecc_key_pair if the key was successfully built.\n Otherwise returns NULL. Note: public_key_x::len and public_key_y::len must\n match the appropriate length for the selected curve_name."]
    pub fn aws_ecc_key_pair_new_from_public_key(
        allocator: *mut aws_allocator,
        curve_name: aws_ecc_curve_name,
        public_key_x: *const aws_byte_cursor,
        public_key_y: *const aws_byte_cursor,
    ) -> *mut aws_ecc_key_pair;
    #[doc = " Creates an Elliptic Curve public/private key pair from a DER encoded key pair.\n Returns a new instance of aws_ecc_key_pair if the key was successfully built.\n Otherwise returns NULL. Whether or not signing or verification can be perform depends\n on if encoded_keys is a public/private pair or a public key."]
    pub fn aws_ecc_key_pair_new_from_asn1(
        allocator: *mut aws_allocator,
        encoded_keys: *const aws_byte_cursor,
    ) -> *mut aws_ecc_key_pair;
    #[doc = " Creates an Elliptic curve public key from x and y coordinates encoded as hex strings\n Returns a new instance of aws_ecc_key_pair if the key was successfully built.\n Otherwise returns NULL."]
    pub fn aws_ecc_key_new_from_hex_coordinates(
        allocator: *mut aws_allocator,
        curve_name: aws_ecc_curve_name,
        pub_x_hex_cursor: aws_byte_cursor,
        pub_y_hex_cursor: aws_byte_cursor,
    ) -> *mut aws_ecc_key_pair;
    #[doc = " Derives a public key from the private key if supported by this operating system (not supported on OSX).\n key_pair::pub_x and key_pair::pub_y will be set with the raw key buffers."]
    pub fn aws_ecc_key_pair_derive_public_key(
        key_pair: *mut aws_ecc_key_pair,
    ) -> ::core::ffi::c_int;
    #[doc = " Get the curve name from the oid. OID here is the payload of the DER encoded ASN.1 part (doesn't include\n type specifier or length. On success, the value of curve_name will be set."]
    pub fn aws_ecc_curve_name_from_oid(
        oid: *mut aws_byte_cursor,
        curve_name: *mut aws_ecc_curve_name,
    ) -> ::core::ffi::c_int;
    #[doc = " Get the DER encoded OID from the curve_name. The OID in this case will not contain the type or the length specifier."]
    pub fn aws_ecc_oid_from_curve_name(
        curve_name: aws_ecc_curve_name,
        oid: *mut aws_byte_cursor,
    ) -> ::core::ffi::c_int;
    #[doc = " Uses the key_pair's private key to sign message. The output will be in signature. Signature must be large enough\n to hold the signature. Check aws_ecc_key_pair_signature_length() for the appropriate size. Signature will be DER\n encoded.\n\n It is the callers job to make sure message is the appropriate cryptographic digest for this operation. It's usually\n something like a SHA256."]
    pub fn aws_ecc_key_pair_sign_message(
        key_pair: *const aws_ecc_key_pair,
        message: *const aws_byte_cursor,
        signature: *mut aws_byte_buf,
    ) -> ::core::ffi::c_int;
    #[doc = " Uses the key_pair's public key to verify signature of message. Signature should be DER\n encoded.\n\n It is the callers job to make sure message is the appropriate cryptographic digest for this operation. It's usually\n something like a SHA256.\n\n returns AWS_OP_SUCCESS if the signature is valid."]
    pub fn aws_ecc_key_pair_verify_signature(
        key_pair: *const aws_ecc_key_pair,
        message: *const aws_byte_cursor,
        signature: *const aws_byte_cursor,
    ) -> ::core::ffi::c_int;
    pub fn aws_ecc_key_pair_signature_length(key_pair: *const aws_ecc_key_pair) -> usize;
    pub fn aws_ecc_key_pair_get_public_key(
        key_pair: *const aws_ecc_key_pair,
        pub_x: *mut aws_byte_cursor,
        pub_y: *mut aws_byte_cursor,
    );
    pub fn aws_ecc_key_pair_get_private_key(
        key_pair: *const aws_ecc_key_pair,
        private_d: *mut aws_byte_cursor,
    );
    pub fn aws_ecc_key_coordinate_byte_size_from_curve_name(
        curve_name: aws_ecc_curve_name,
    ) -> usize;
    #[doc = " Allocates and initializes a sha256 hash instance."]
    pub fn aws_sha256_new(allocator: *mut aws_allocator) -> *mut aws_hash;
    #[doc = " Allocates and initializes a sha1 hash instance."]
    pub fn aws_sha1_new(allocator: *mut aws_allocator) -> *mut aws_hash;
    #[doc = " Allocates and initializes an md5 hash instance."]
    pub fn aws_md5_new(allocator: *mut aws_allocator) -> *mut aws_hash;
    #[doc = " Cleans up and deallocates hash."]
    pub fn aws_hash_destroy(hash: *mut aws_hash);
    #[doc = " Updates the running hash with to_hash. this can be called multiple times."]
    pub fn aws_hash_update(
        hash: *mut aws_hash,
        to_hash: *const aws_byte_cursor,
    ) -> ::core::ffi::c_int;
    #[doc = " Completes the hash computation and writes the final digest to output.\n Allocation of output is the caller's responsibility. If you specify\n truncate_to to something other than 0, the output will be truncated to that\n number of bytes. For example, if you want a SHA256 digest as the first 16\n bytes, set truncate_to to 16. If you want the full digest size, just set this\n to 0."]
    pub fn aws_hash_finalize(
        hash: *mut aws_hash,
        output: *mut aws_byte_buf,
        truncate_to: usize,
    ) -> ::core::ffi::c_int;
    #[doc = " Computes the md5 hash over input and writes the digest output to 'output'.\n Use this if you don't need to stream the data you're hashing and you can load\n the entire input to hash into memory."]
    pub fn aws_md5_compute(
        allocator: *mut aws_allocator,
        input: *const aws_byte_cursor,
        output: *mut aws_byte_buf,
        truncate_to: usize,
    ) -> ::core::ffi::c_int;
    #[doc = " Computes the sha256 hash over input and writes the digest output to 'output'.\n Use this if you don't need to stream the data you're hashing and you can load\n the entire input to hash into memory. If you specify truncate_to to something\n other than 0, the output will be truncated to that number of bytes. For\n example, if you want a SHA256 digest as the first 16 bytes, set truncate_to\n to 16. If you want the full digest size, just set this to 0."]
    pub fn aws_sha256_compute(
        allocator: *mut aws_allocator,
        input: *const aws_byte_cursor,
        output: *mut aws_byte_buf,
        truncate_to: usize,
    ) -> ::core::ffi::c_int;
    #[doc = " Computes the sha1 hash over input and writes the digest output to 'output'.\n Use this if you don't need to stream the data you're hashing and you can load\n the entire input to hash into memory. If you specify truncate_to to something\n other than 0, the output will be truncated to that number of bytes. For\n example, if you want a SHA1 digest as the first 16 bytes, set truncate_to\n to 16. If you want the full digest size, just set this to 0."]
    pub fn aws_sha1_compute(
        allocator: *mut aws_allocator,
        input: *const aws_byte_cursor,
        output: *mut aws_byte_buf,
        truncate_to: usize,
    ) -> ::core::ffi::c_int;
    #[doc = " Set the implementation of md5 to use. If you compiled without BYO_CRYPTO,\n you do not need to call this. However, if use this, we will honor it,\n regardless of compile options. This may be useful for testing purposes. If\n you did set BYO_CRYPTO, and you do not call this function you will\n segfault."]
    pub fn aws_set_md5_new_fn(fn_: aws_hash_new_fn);
    #[doc = " Set the implementation of sha256 to use. If you compiled without\n BYO_CRYPTO, you do not need to call this. However, if use this, we will\n honor it, regardless of compile options. This may be useful for testing\n purposes. If you did set BYO_CRYPTO, and you do not call this function\n you will segfault."]
    pub fn aws_set_sha256_new_fn(fn_: aws_hash_new_fn);
    #[doc = " Set the implementation of sha1 to use. If you compiled without\n BYO_CRYPTO, you do not need to call this. However, if use this, we will\n honor it, regardless of compile options. This may be useful for testing\n purposes. If you did set BYO_CRYPTO, and you do not call this function\n you will segfault."]
    pub fn aws_set_sha1_new_fn(fn_: aws_hash_new_fn);
    #[doc = " Allocates and initializes a sha256 hmac instance. Secret is the key to be\n used for the hmac process."]
    pub fn aws_sha256_hmac_new(
        allocator: *mut aws_allocator,
        secret: *const aws_byte_cursor,
    ) -> *mut aws_hmac;
    #[doc = " Cleans up and deallocates hmac."]
    pub fn aws_hmac_destroy(hmac: *mut aws_hmac);
    #[doc = " Updates the running hmac with to_hash. this can be called multiple times."]
    pub fn aws_hmac_update(
        hmac: *mut aws_hmac,
        to_hmac: *const aws_byte_cursor,
    ) -> ::core::ffi::c_int;
    #[doc = " Completes the hmac computation and writes the final digest to output.\n Allocation of output is the caller's responsibility. If you specify\n truncate_to to something other than 0, the output will be truncated to that\n number of bytes. For example if you want a SHA256 digest as the first 16\n bytes, set truncate_to to 16. If you want the full digest size, just set this\n to 0."]
    pub fn aws_hmac_finalize(
        hmac: *mut aws_hmac,
        output: *mut aws_byte_buf,
        truncate_to: usize,
    ) -> ::core::ffi::c_int;
    #[doc = " Computes the sha256 hmac over input and writes the digest output to 'output'.\n Use this if you don't need to stream the data you're hashing and you can load\n the entire input to hash into memory. If you specify truncate_to to something\n other than 0, the output will be truncated to that number of bytes. For\n example if you want a SHA256 HMAC digest as the first 16 bytes, set\n truncate_to to 16. If you want the full digest size, just set this to 0."]
    pub fn aws_sha256_hmac_compute(
        allocator: *mut aws_allocator,
        secret: *const aws_byte_cursor,
        to_hmac: *const aws_byte_cursor,
        output: *mut aws_byte_buf,
        truncate_to: usize,
    ) -> ::core::ffi::c_int;
    #[doc = " Set the implementation of sha256 hmac to use. If you compiled without\n BYO_CRYPTO, you do not need to call this. However, if use this, we will\n honor it, regardless of compile options. This may be useful for testing\n purposes. If you did set BYO_CRYPTO, and you do not call this function\n you will segfault."]
    pub fn aws_set_sha256_hmac_new_fn(fn_: aws_hmac_new_fn);
    #[doc = " Creates an RSA public key from RSAPublicKey as defined in rfc 8017 (aka PKCS1).\n Returns a new instance of aws_rsa_key_pair if the key was successfully built.\n Otherwise returns NULL."]
    pub fn aws_rsa_key_pair_new_from_public_key_pkcs1(
        allocator: *mut aws_allocator,
        key: aws_byte_cursor,
    ) -> *mut aws_rsa_key_pair;
    #[doc = " Creates an RSA private key from RSAPrivateKey as defined in rfc 8017 (aka PKCS1).\n Returns a new instance of aws_rsa_key_pair if the key was successfully built.\n Otherwise returns NULL."]
    pub fn aws_rsa_key_pair_new_from_private_key_pkcs1(
        allocator: *mut aws_allocator,
        key: aws_byte_cursor,
    ) -> *mut aws_rsa_key_pair;
    #[doc = " Adds one to an RSA key pair's ref count.\n Returns key_pair pointer."]
    pub fn aws_rsa_key_pair_acquire(key_pair: *mut aws_rsa_key_pair) -> *mut aws_rsa_key_pair;
    #[doc = " Subtracts one from an RSA key pair's ref count. If ref count reaches zero, the key pair is destroyed.\n Always returns NULL."]
    pub fn aws_rsa_key_pair_release(key_pair: *mut aws_rsa_key_pair) -> *mut aws_rsa_key_pair;
    #[doc = " Max plaintext size that can be encrypted by the key (i.e. max data size\n supported by the key - bytes needed for padding)."]
    pub fn aws_rsa_key_pair_max_encrypt_plaintext_size(
        key_pair: *const aws_rsa_key_pair,
        algorithm: aws_rsa_encryption_algorithm,
    ) -> usize;
    pub fn aws_rsa_key_pair_encrypt(
        key_pair: *const aws_rsa_key_pair,
        algorithm: aws_rsa_encryption_algorithm,
        plaintext: aws_byte_cursor,
        out: *mut aws_byte_buf,
    ) -> ::core::ffi::c_int;
    pub fn aws_rsa_key_pair_decrypt(
        key_pair: *const aws_rsa_key_pair,
        algorithm: aws_rsa_encryption_algorithm,
        ciphertext: aws_byte_cursor,
        out: *mut aws_byte_buf,
    ) -> ::core::ffi::c_int;
    pub fn aws_rsa_key_pair_block_length(key_pair: *const aws_rsa_key_pair) -> usize;
    #[doc = " Uses the key_pair's private key to sign message. The output will be in out. out must be large enough\n to hold the signature. Check aws_rsa_key_pair_signature_length() for the appropriate size.\n\n It is the callers job to make sure message is the appropriate cryptographic digest for this operation. It's usually\n something like a SHA256."]
    pub fn aws_rsa_key_pair_sign_message(
        key_pair: *const aws_rsa_key_pair,
        algorithm: aws_rsa_signature_algorithm,
        digest: aws_byte_cursor,
        out: *mut aws_byte_buf,
    ) -> ::core::ffi::c_int;
    #[doc = " Uses the key_pair's public key to verify signature of message.\n\n It is the callers job to make sure message is the appropriate cryptographic digest for this operation. It's usually\n something like a SHA256.\n\n returns AWS_OP_SUCCESS if the signature is valid.\n raises AWS_ERROR_CAL_SIGNATURE_VALIDATION_FAILED if signature validation failed"]
    pub fn aws_rsa_key_pair_verify_signature(
        key_pair: *const aws_rsa_key_pair,
        algorithm: aws_rsa_signature_algorithm,
        digest: aws_byte_cursor,
        signature: aws_byte_cursor,
    ) -> ::core::ffi::c_int;
    pub fn aws_rsa_key_pair_signature_length(key_pair: *const aws_rsa_key_pair) -> usize;
    pub fn aws_rsa_key_pair_get_public_key(
        key_pair: *const aws_rsa_key_pair,
        format: aws_rsa_key_export_format,
        out: *mut aws_byte_buf,
    ) -> ::core::ffi::c_int;
    pub fn aws_rsa_key_pair_get_private_key(
        key_pair: *const aws_rsa_key_pair,
        format: aws_rsa_key_export_format,
        out: *mut aws_byte_buf,
    ) -> ::core::ffi::c_int;
    #[doc = " Creates an instance of AES CBC with 256-bit key.\n If key and iv are NULL, they will be generated internally.\n You can get the generated key and iv back by calling:\n\n aws_symmetric_cipher_get_key() and\n aws_symmetric_cipher_get_initialization_vector()\n\n respectively.\n\n If they are set, that key and iv will be copied internally and used by the cipher.\n\n Returns NULL on failure. You can check aws_last_error() to get the error code indicating the failure cause."]
    pub fn aws_aes_cbc_256_new(
        allocator: *mut aws_allocator,
        key: *const aws_byte_cursor,
        iv: *const aws_byte_cursor,
    ) -> *mut aws_symmetric_cipher;
    #[doc = " Creates an instance of AES CTR with 256-bit key.\n If key and iv are NULL, they will be generated internally.\n You can get the generated key and iv back by calling:\n\n aws_symmetric_cipher_get_key() and\n aws_symmetric_cipher_get_initialization_vector()\n\n respectively.\n\n If they are set, that key and iv will be copied internally and used by the cipher.\n\n Returns NULL on failure. You can check aws_last_error() to get the error code indicating the failure cause."]
    pub fn aws_aes_ctr_256_new(
        allocator: *mut aws_allocator,
        key: *const aws_byte_cursor,
        iv: *const aws_byte_cursor,
    ) -> *mut aws_symmetric_cipher;
    #[doc = " Creates an instance of AES GCM with 256-bit key.\n If key, iv are NULL, they will be generated internally.\n You can get the generated key and iv back by calling:\n\n aws_symmetric_cipher_get_key() and\n aws_symmetric_cipher_get_initialization_vector()\n\n respectively.\n\n If they are set, that key and iv will be copied internally and used by the cipher.\n\n If tag and aad are set they will be copied internally and used by the cipher.\n decryption_tag would most likely be used for a decrypt operation to detect tampering or corruption.\n The Tag for the most recent encrypt operation will be available in:\n\n aws_symmetric_cipher_get_tag()\n\n If aad is set it will be copied and applied to the cipher.\n\n Returns NULL on failure. You can check aws_last_error() to get the error code indicating the failure cause."]
    pub fn aws_aes_gcm_256_new(
        allocator: *mut aws_allocator,
        key: *const aws_byte_cursor,
        iv: *const aws_byte_cursor,
        aad: *const aws_byte_cursor,
        decryption_tag: *const aws_byte_cursor,
    ) -> *mut aws_symmetric_cipher;
    #[doc = " Creates an instance of AES Keywrap with 256-bit key.\n If key is NULL, it will be generated internally.\n You can get the generated key back by calling:\n\n aws_symmetric_cipher_get_key()\n\n If key is set, that key will be copied internally and used by the cipher.\n\n Returns NULL on failure. You can check aws_last_error() to get the error code indicating the failure cause."]
    pub fn aws_aes_keywrap_256_new(
        allocator: *mut aws_allocator,
        key: *const aws_byte_cursor,
    ) -> *mut aws_symmetric_cipher;
    #[doc = " Cleans up internal resources and state for cipher and then deallocates it."]
    pub fn aws_symmetric_cipher_destroy(cipher: *mut aws_symmetric_cipher);
    #[doc = " Encrypts the value in to_encrypt and writes the encrypted data into out.\n If out is dynamic it will be expanded. If it is not, and out is not large enough to handle\n the encrypted output, the call will fail. If you're trying to optimize to use a stack based array\n or something, make sure it's at least as large as the size of to_encrypt + an extra BLOCK to account for\n padding etc...\n\n returns AWS_OP_SUCCESS on success. Call aws_last_error() to determine the failure cause if it returns\n AWS_OP_ERR;"]
    pub fn aws_symmetric_cipher_encrypt(
        cipher: *mut aws_symmetric_cipher,
        to_encrypt: aws_byte_cursor,
        out: *mut aws_byte_buf,
    ) -> ::core::ffi::c_int;
    #[doc = " Decrypts the value in to_decrypt and writes the decrypted data into out.\n If out is dynamic it will be expanded. If it is not, and out is not large enough to handle\n the decrypted output, the call will fail. If you're trying to optimize to use a stack based array\n or something, make sure it's at least as large as the size of to_decrypt + an extra BLOCK to account for\n padding etc...\n\n returns AWS_OP_SUCCESS on success. Call aws_last_error() to determine the failure cause if it returns\n AWS_OP_ERR;"]
    pub fn aws_symmetric_cipher_decrypt(
        cipher: *mut aws_symmetric_cipher,
        to_decrypt: aws_byte_cursor,
        out: *mut aws_byte_buf,
    ) -> ::core::ffi::c_int;
    #[doc = " Encrypts any remaining data that was reserved for final padding, loads GMACs etc... and if there is any\n writes any remaining encrypted data to out. If out is dynamic it will be expanded. If it is not, and\n out is not large enough to handle the decrypted output, the call will fail. If you're trying to optimize\n  to use a stack based array or something, make sure it's at least as large as the size of 2 BLOCKs to account for\n padding etc...\n\n After invoking this function, you MUST call aws_symmetric_cipher_reset() before invoking any encrypt/decrypt\n operations on this cipher again.\n\n returns AWS_OP_SUCCESS on success. Call aws_last_error() to determine the failure cause if it returns\n AWS_OP_ERR;"]
    pub fn aws_symmetric_cipher_finalize_encryption(
        cipher: *mut aws_symmetric_cipher,
        out: *mut aws_byte_buf,
    ) -> ::core::ffi::c_int;
    #[doc = " Decrypts any remaining data that was reserved for final padding, loads GMACs etc... and if there is any\n writes any remaining decrypted data to out. If out is dynamic it will be expanded. If it is not, and\n out is not large enough to handle the decrypted output, the call will fail. If you're trying to optimize\n to use a stack based array or something, make sure it's at least as large as the size of 2 BLOCKs to account for\n padding etc...\n\n After invoking this function, you MUST call aws_symmetric_cipher_reset() before invoking any encrypt/decrypt\n operations on this cipher again.\n\n returns AWS_OP_SUCCESS on success. Call aws_last_error() to determine the failure cause if it returns\n AWS_OP_ERR;"]
    pub fn aws_symmetric_cipher_finalize_decryption(
        cipher: *mut aws_symmetric_cipher,
        out: *mut aws_byte_buf,
    ) -> ::core::ffi::c_int;
    #[doc = " Resets the cipher state for starting a new encrypt or decrypt operation. Note encrypt/decrypt cannot be mixed on the\n same cipher without a call to reset in between them. However, this leaves the key, iv etc... materials setup for\n immediate reuse.\n\n returns AWS_OP_SUCCESS on success. Call aws_last_error() to determine the failure cause if it returns\n AWS_OP_ERR;"]
    pub fn aws_symmetric_cipher_reset(cipher: *mut aws_symmetric_cipher) -> ::core::ffi::c_int;
    #[doc = " Gets the current GMAC tag. If not AES GCM, this function will just return an empty cursor.\n The memory in this cursor is unsafe as it refers to the internal buffer.\n This was done because the use case doesn't require fetching these during an\n encryption or decryption operation and it dramatically simplifies the API.\n Only use this function between other calls to this API as any function call can alter the value of this tag.\n\n If you need to access it in a different pattern, copy the values to your own buffer first."]
    pub fn aws_symmetric_cipher_get_tag(cipher: *const aws_symmetric_cipher) -> aws_byte_cursor;
    #[doc = " Gets the original initialization vector as a cursor.\n The memory in this cursor is unsafe as it refers to the internal buffer.\n This was done because the use case doesn't require fetching these during an\n encryption or decryption operation and it dramatically simplifies the API.\n\n Unlike some other fields, this value does not change after the inital construction of the cipher.\n\n For some algorithms, such as AES Keywrap, this will return an empty cursor."]
    pub fn aws_symmetric_cipher_get_initialization_vector(
        cipher: *const aws_symmetric_cipher,
    ) -> aws_byte_cursor;
    #[doc = " Gets the original key.\n\n The memory in this cursor is unsafe as it refers to the internal buffer.\n This was done because the use case doesn't require fetching these during an\n encryption or decryption operation and it dramatically simplifies the API.\n\n Unlike some other fields, this value does not change after the inital construction of the cipher."]
    pub fn aws_symmetric_cipher_get_key(cipher: *const aws_symmetric_cipher) -> aws_byte_cursor;
    #[doc = " Returns true if the state of the cipher is good, and otherwise returns false.\n Most operations, other than aws_symmetric_cipher_reset() will fail if this function is returning false.\n aws_symmetric_cipher_reset() will reset the state to a good state if possible."]
    pub fn aws_symmetric_cipher_is_good(cipher: *const aws_symmetric_cipher) -> bool;
}
