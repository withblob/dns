## Blob DNS — blobal DNS checker CLI 🌐

#### Like `dig` or `dog` but checks DNS records in three regions:
* USA (Ohio)
* Germany (Frankfurt)
* Singapore

Can be useful to see how your DNS records propagate blobally. 

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

<img width="535" alt="image" src="https://github.com/withblob/dns/assets/139460414/548e1772-a98c-40e1-9825-e6fd6cd79ee3">


Built in Rust. To use it you will need to compile it yourself (for now), pre-built packages will come soon!

Licensed under MIT license.

Copyright withblob.com
