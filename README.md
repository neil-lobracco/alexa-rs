[![Crates.io Status](http://meritbadge.herokuapp.com/alexa)](https://crates.io/crates/alexa)

**alexa-rs** is a library for quickly developing Alexa skills in Rust.

It exposes an Iron handler that validates and parses the request, hands it to a user-specified trait object that provides a a response, and converts that to appropriate response.

**alexa-rs** does not provide a direct mapping of the JSON interface, but rather a mid-level abstraction that rejects invalid input and makes it difficult to craft an invalid response.

To see more information about the interface Amazon provides, see https://developer.amazon.com/public/solutions/alexa/alexa-skills-kit/docs/alexa-skills-kit-interface-reference

See `examples/double_number.rs` for an example of a simple app developed using this toolkit.
