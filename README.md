## Blob DNS — blobal DNS checker CLI 🌐

#### Like `dig` or `dog` but checks DNS records in three regions:
* USA (Ohio)
* Germany (Frankfurt)
* Singapore

Unlike other similar tools that connect to remote nameservers, `blobdns` actually runs the DNS queries globally by utilizing Blob's distributed DNS checking API. 

This approach will actually test the DNS record propagation from the perspective of real users, located in those countries.

It can be useful to see how your DNS records propagate blobally ;) 

```
Usage:
  blobdns <arguments> [options]

Examples:
  blobdns example.com
  blobdns example.com A
  blobdns foo.example.com bar.example.com CNAME

Arguments:
  <arguments> domains, records

Options:
  -v, --version Print version information
  -h, --help Print list of options
```

### Example output

#### `blobdns example.com`

<img width="528" alt="image" src="https://github.com/withblob/dns/assets/139460414/296b80c0-e35a-490c-8efb-e92fd6d2245f">


Built in Rust. To use it, you will need to compile it yourself (for now). Pre-built packages will come soon!

Licensed under MIT license.

Copyright withblob.com
