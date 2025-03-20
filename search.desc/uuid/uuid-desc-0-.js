searchState.loadedDescShard("uuid", 0, "Generate and parse universally unique identifiers (UUIDs).\nA builder for creating a UUID.\nA 128-bit (16 byte) buffer containing the UUID.\nVersion 8: Custom.\nVersion 2: DCE Security.\nA general error that can occur when working with UUIDs.\nReserved for future expansion.\nVersion 1: Timestamp and node ID.\nThe “max” (all ones) UUID.\nVersion 3: MD5 hash.\nReserved by Microsoft for backward compatibility.\nUUID namespace for Domain Name System (DNS).\nUUID namespace for ISO Object Identifiers (OIDs).\nUUID namespace for Uniform Resource Locators (URLs).\nUUID namespace for X.500 Distinguished Names (DNs).\nReserved by the NCS for backward compatibility.\nThe “nil” (all zeros) UUID.\nA UUID that is guaranteed not to be the nil UUID.\nAs described in the RFC 9562 Specification (default). (for …\nVersion 4: Random.\nVersion 5: SHA-1 hash.\nVersion 6: Sortable Timestamp and node ID.\nVersion 7: Timestamp and random.\nA Universally Unique Identifier (UUID).\nThe reserved variants of UUIDs.\nThe version of the UUID, denoting the generating algorithm.\nGet a borrowed <code>Braced</code> formatter.\nReturns a slice of 16 octets containing the value.\nReturns the four field values of the UUID.\nGet a borrowed <code>Hyphenated</code> formatter.\nGet a borrowed <code>Simple</code> formatter.\nReturns a 128bit value containing the value.\nReturns two 64bit values containing the value.\nGet a borrowed <code>Urn</code> formatter.\nGet a reference to the underlying <code>Uuid</code>.\nGet a <code>Braced</code> formatter.\nA buffer that can be used for <code>encode_...</code> calls, that is …\nAdapters for alternative string formats.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConverts a <code>NonNilUuid</code> back into a <code>Uuid</code>.\nCreates a <code>Builder</code> using the supplied bytes.\nCreates a UUID using the supplied bytes.\nCreates a <code>Builder</code> using the supplied bytes in little …\nCreates a UUID using the supplied bytes in little endian …\nCreates a reference to a UUID from a reference to the …\nCreates a <code>Builder</code> for a version 8 UUID using the supplied …\nCreates a <code>Builder</code> from four field values.\nCreates a UUID from four field values.\nCreates a <code>Builder</code> from four field values.\nCreates a UUID from four field values in little-endian …\nCreates a <code>Builder</code> for a version 1 UUID using the supplied …\nCreates a <code>Builder</code> for a version 3 UUID using the supplied …\nCreates a <code>Builder</code> for a version 4 UUID using the supplied …\nCreates a <code>Builder</code> for a version 5 UUID using the supplied …\nCreates a <code>Builder</code> using the supplied bytes.\nCreates a UUID using the supplied bytes.\nCreates a <code>Builder</code> using the supplied bytes in little …\nCreates a UUID using the supplied bytes in little endian …\nCreates a <code>Builder</code> for a version 6 UUID using the supplied …\nCreates a <code>Builder</code> from a 128bit value.\nCreates a UUID from a 128bit value.\nCreates a UUID from a 128bit value in little-endian order.\nCreates a UUID from a 128bit value in little-endian order.\nCreates a UUID from two 64bit values.\nCreates a <code>Builder</code> for a version 7 UUID using the supplied …\nGet the underlying <code>Uuid</code> value.\nIf the UUID is the correct version (v1, or v6) this will …\nIf the UUID is the correct version (v1, v6, or v7) this …\nReturns the variant of the UUID structure.\nReturns the version of the UUID.\nReturns the version number of the UUID.\nGet a <code>Hyphenated</code> formatter.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsumes self and returns the underlying byte value of the …\nConvert the builder into a <code>Uuid</code>.\nTests if the UUID is max (all ones).\nTests if the UUID is nil (all zeros).\nThe ‘max UUID’ (all ones).\nCreates a non-nil UUID if the value is non-nil.\nCreates a non-nil without checking whether the value is …\nCreates a random UUID.\nCreates a <code>Builder</code> with an initial <code>Uuid::nil</code>.\nThe ‘nil UUID’ (all zeros).\nParses a <code>Uuid</code> from a string of hexadecimal digits with …\nSpecifies the variant of the UUID.\nSpecifies the version number of the UUID.\nGet a <code>Simple</code> formatter.\nGenerating UUIDs from timestamps.\nReturns the bytes of the UUID in little-endian order.\nReturns the four field values of the UUID in little-endian …\nReturns a 128bit little-endian value containing the value.\nAttempts to convert a <code>Uuid</code> into a <code>NonNilUuid</code>.\nParses a <code>Uuid</code> from a string of hexadecimal digits with …\nParses a <code>Uuid</code> from a string of hexadecimal digits with …\nGet a <code>Urn</code> formatter.\nParse <code>Uuid</code>s from string literals at compile time.\nSpecifies the variant of the UUID.\nSpecifies the version number of the UUID.\nFormat a <code>Uuid</code> as a braced hyphenated string, like …\nFormat a <code>Uuid</code> as a hyphenated string, like …\nThe length of a hyphenated <code>Uuid</code> string.\nThe length of a simple <code>Uuid</code> string.\nThe length of a URN <code>Uuid</code> string.\nThe length of a braced <code>Uuid</code> string.\nFormat a <code>Uuid</code> as a simple string, like …\nFormat a <code>Uuid</code> as a URN string, like …\nGet a reference to the underlying <code>Uuid</code>.\nGet a reference to the underlying <code>Uuid</code>.\nGet a reference to the underlying <code>Uuid</code>.\nGet a reference to the underlying <code>Uuid</code>.\nWrites the <code>Uuid</code> as a lower-case hyphenated string to <code>buffer</code>…\nWrites the <code>Uuid</code> as a lower-case simple string to <code>buffer</code>, …\nWrites the <code>Uuid</code> as a lower-case URN string to <code>buffer</code>, and …\nWrites the <code>Uuid</code> as a lower-case hyphenated string …\nWrites the <code>Uuid</code> as an upper-case hyphenated string to …\nWrites the <code>Uuid</code> as an upper-case simple string to <code>buffer</code>, …\nWrites the <code>Uuid</code> as an upper-case URN string to <code>buffer</code>, and …\nWrites the <code>Uuid</code> as an upper-case hyphenated string …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a <code>Hyphenated</code> from a <code>Uuid</code>.\nCreates a <code>Simple</code> from a <code>Uuid</code>.\nCreates a <code>Urn</code> from a <code>Uuid</code>.\nCreates a <code>Braced</code> from a <code>Uuid</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConsumes the <code>Hyphenated</code>, returning the underlying <code>Uuid</code>.\nConsumes the <code>Simple</code>, returning the underlying <code>Uuid</code>.\nConsumes the <code>Urn</code>, returning the underlying <code>Uuid</code>.\nConsumes the <code>Braced</code>, returning the underlying <code>Uuid</code>.\nA counter that can be used by versions 1 and 6 UUIDs to …\nThe type of sequence returned by this counter.\nA timestamp that can be encoded into a UUID.\nThe number of 100 nanosecond ticks between the RFC 9562 …\nDefault implementations for the <code>ClockSequence</code> trait.\nReturns the argument unchanged.\nConstruct a <code>Timestamp</code> from the number of 100 nanosecond …\nConstruct a <code>Timestamp</code> from a Unix timestamp and up to a …\nConstruct a <code>Timestamp</code> from a Unix timestamp and up to a …\nGet the next value in the sequence to feed into a …\nGet the next value in the sequence, potentially also …\nCalls <code>U::from(self)</code>.\nGet a timestamp representing the current system time and …\nGet the value of the timestamp as the number of 100 …\nGet the value of the timestamp as a Unix timestamp, as …\nThe number of usable bits from the least significant bit …\nAn empty counter that will always return the value <code>0</code>.\nA wrapper for a context that uses thread-local storage.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWrap a thread-local container with a context.")