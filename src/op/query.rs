/*
 * Copyright (C) 2015 Benjamin Fry <benjaminfry@me.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use ::rr::domain::Name;
use ::rr::record_type::RecordType;
use ::rr::dns_class::DNSClass;
use ::serialize::binary::*;
use ::error::*;


/*
 * RFC 1035        Domain Implementation and Specification    November 1987
 *
 * 4.1.2. Question section format
 *
 * The question section is used to carry the "question" in most queries,
 * i.e., the parameters that define what is being asked.  The section
 * contains QDCOUNT (usually 1) entries, each of the following format:
 *
 *                                     1  1  1  1  1  1
 *       0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
 *     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
 *     |                                               |
 *     /                     QNAME / ZNAME             /
 *     /                                               /
 *     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
 *     |                     QTYPE / ZTYPE             |
 *     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
 *     |                     QCLASS / ZCLASS           |
 *     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
 *
 * where:
 *
 * QNAME           a domain name represented as a sequence of labels, where
 *                 each label consists of a length octet followed by that
 *                 number of octets.  The domain name terminates with the
 *                 zero length octet for the null label of the root.  Note
 *                 that this field may be an odd number of octets; no
 *                 padding is used.
 *
 * QTYPE           a two octet code which specifies the type of the query.
 *                 The values for this field include all codes valid for a
 *                 TYPE field, together with some more general codes which
 *                 can match more than one type of RR.
 *
 * QCLASS          a two octet code that specifies the class of the query.
 *                 For example, the QCLASS field is IN for the Internet.
 *
 *
 */
#[derive(PartialEq, Debug)]
pub struct Query {
  name: Name, query_type: RecordType, query_class: DNSClass
}

impl Query {
  /// return a default query with an empty name and A, IN for the query_type and query_class
  pub fn new() -> Self {
    Query { name: Name::new(), query_type: RecordType::A, query_class: DNSClass::IN }
  }

  /// replaces name with the new name
  pub fn name(&mut self, name: Name) -> &mut Self { self.name = name; self }
  pub fn query_type(&mut self, query_type: RecordType) -> &mut Self { self.query_type = query_type; self }
  pub fn query_class(&mut self, query_class: DNSClass) -> &mut Self { self.query_class = query_class; self }

  pub fn get_name(&self) -> &Name { &self.name }
  pub fn get_query_type(&self) -> RecordType { self.query_type }
  pub fn get_query_class(&self) -> DNSClass { self.query_class }

}

impl BinSerializable for Query {
  fn read(decoder: &mut BinDecoder) -> DecodeResult<Self> {
    let name = try!(Name::read(decoder));
    let query_type = try!(RecordType::read(decoder));
    let query_class = try!(DNSClass::read(decoder));

    Ok(Query { name: name, query_type: query_type, query_class: query_class})
  }

  fn emit(&self, encoder: &mut BinEncoder) -> EncodeResult {
    try!(self.name.emit(encoder));
    try!(self.query_type.emit(encoder));
    try!(self.query_class.emit(encoder));

    Ok(())
  }
}

#[test]
fn test_read_and_emit() {
  let expect = Query { name: Name::with_labels(vec!["www".to_string(),"example".to_string(),"com".to_string()]),
                       query_type: RecordType::AAAA, query_class: DNSClass::IN };

  let mut encoder = BinEncoder::new();
  expect.emit(&mut encoder).unwrap();

  let byte_vec = encoder.as_bytes();

  let mut decoder = BinDecoder::new(&byte_vec);
  let got = Query::read(&mut decoder).unwrap();
  assert_eq!(got, expect);
}