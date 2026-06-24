# Security Policy

## Supported Versions

The project is developed as part of an academic software quality assignment. Only the latest version available on the main branch is considered supported.

| Version              | Supported |
| -------------------- | --------- |
| Latest (main branch) | ✅         |
| Older versions       | ❌         |

## Reporting a Vulnerability

If you discover a security vulnerability in Moseiik, please report it by opening a GitHub issue or by contacting the project maintainers.

When reporting a vulnerability, please include:

* A description of the issue
* Steps to reproduce the problem
* Potential impact
* Suggested mitigation if available

Please do **not** publicly disclose critical vulnerabilities before maintainers have had an opportunity to investigate and address the issue.

## Security Considerations

Moseiik processes image files provided by users. Users should only execute the software with trusted input files and directories.

The project currently has no network-facing components and does not intentionally collect, transmit, or store personal information.

Dependencies should be kept up to date through Cargo and regularly checked for known vulnerabilities using:

```bash
cargo audit
```

## Responsible Disclosure

All reported vulnerabilities will be reviewed as quickly as possible. The maintainers will assess the severity of the issue and determine the appropriate remediation.
