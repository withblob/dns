## Blob DNS — global DNS checker CLI 🌐

#### Like `dig` or `dog` but checks DNS records in three regions:
* USA (Ohio)
* Germany (Frankfurt)
* Singapore

Unlike other similar tools that connect to remote nameservers, `blobdns` actually runs the DNS queries globally by utilizing Blob's distributed DNS checking API. 

This approach will actually test the DNS record propagation from the perspective of real users, located in those countries.

It can be useful to see how your DNS records propagate blobally ;) 

## Usage:
```md
blobdns <arguments> [options]

Examples:
  blobdns example.com
  blobdns example.com A
  blobdns foo.example.com bar.example.com CNAME

  # Get AAAA record from 1.1.1.1 Cloudflare server
  blobdns example.com AAAA @1.1.1.1

Arguments:
  <arguments> domains, records, nameservers

Options:
  -v, --version Print version information
  -h, --help Print list of options
```

### Example output

#### `blobdns example.com`

<img width="600" alt="Screenshot 2023-07-24 at 12 51 50" src="https://github.com/withblob/dns/assets/139460414/8bf6e40d-ccfa-4e08-a82f-e934c66bd2e3">


Built in Rust. To use it, you can download the available packages from the releases or compile it yourself (make sure you have Rust installed).

```sh
git clone https://github.com/withblob/dns.git
cd dns
cargo build --release
```
### 💚 Contributions and feature requests are welcome!

Licensed under MIT license.

Copyright (c) withblob.com, 2023.
