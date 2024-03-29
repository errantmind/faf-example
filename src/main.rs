#![feature(core_intrinsics)]
#![feature(start, lang_items)]
#![allow(clippy::missing_safety_doc, unused_imports, dead_code)]

use core::intrinsics::likely;
use faf::const_concat_bytes;
use faf::const_http::*;
use faf::epoll;
use faf::util::memcmp;

const ROUTE_PLAINTEXT: &[u8] = b"/plaintext";
const ROUTE_PLAINTEXT_LEN: usize = ROUTE_PLAINTEXT.len();

const TEXT_PLAIN_CONTENT_TYPE: &[u8] = b"Content-Type: text/plain";
const CONTENT_LENGTH: &[u8] = b"Content-Length: ";
const PLAINTEXT_BODY: &[u8] = b"Hello, World!";
const PLAINTEXT_BODY_LEN: usize = PLAINTEXT_BODY.len();
const PLAINTEXT_BODY_SIZE: &[u8] = b"13";

const PLAINTEXT_BASE: &[u8] = const_concat_bytes!(
   HTTP_200_OK,
   CRLF,
   SERVER,
   CRLF,
   TEXT_PLAIN_CONTENT_TYPE,
   CRLF,
   CONTENT_LENGTH,
   PLAINTEXT_BODY_SIZE,
   CRLF
);

const PLAINTEXT_BASE_LEN: usize = PLAINTEXT_BASE.len();

#[inline(always)]
fn cb(
   method: *const u8,
   method_len: usize,
   path: *const u8,
   path_len: usize,
   response_buffer: *mut u8,
   date_buff: *const u8,
) -> usize {
   unsafe {
      if likely(method_len == GET_LEN && path_len == ROUTE_PLAINTEXT_LEN) {
         if likely(memcmp(GET.as_ptr(), method, GET_LEN) == 0) {
            if likely(memcmp(ROUTE_PLAINTEXT.as_ptr(), path, ROUTE_PLAINTEXT_LEN) == 0) {
               core::ptr::copy_nonoverlapping(PLAINTEXT_BASE.as_ptr(), response_buffer, PLAINTEXT_BASE_LEN);
               core::ptr::copy_nonoverlapping(date_buff, response_buffer.add(PLAINTEXT_BASE_LEN), DATE_LEN);
               core::ptr::copy_nonoverlapping(
                  CRLFCRLF.as_ptr(),
                  response_buffer.add(PLAINTEXT_BASE_LEN + DATE_LEN),
                  CRLFCRLF_LEN,
               );
               core::ptr::copy_nonoverlapping(
                  PLAINTEXT_BODY.as_ptr(),
                  response_buffer.add(PLAINTEXT_BASE_LEN + DATE_LEN + CRLFCRLF_LEN),
                  PLAINTEXT_BODY_LEN,
               );

               PLAINTEXT_BASE_LEN + DATE_LEN + CRLFCRLF_LEN + PLAINTEXT_BODY_LEN
            } else {
               core::ptr::copy_nonoverlapping(HTTP_404_NOTFOUND.as_ptr(), response_buffer, HTTP_404_NOTFOUND_LEN);
               HTTP_404_NOTFOUND_LEN
            }
         } else {
            core::ptr::copy_nonoverlapping(HTTP_405_NOTALLOWED.as_ptr(), response_buffer, HTTP_405_NOTALLOWED_LEN);
            HTTP_405_NOTALLOWED_LEN
         }
      } else {
         0
      }
   }
}

#[inline(always)]
pub fn main() {
   epoll::go(8080, cb);
}
