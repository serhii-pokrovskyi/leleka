# leleka

fast and simple mobile application delivery

> Very early alpha: nothing is uploaded yet. Commands check their arguments and stop with "not implemented".

## Usage

```
leleka play --key-file key.json upload app.aab --track internal
```

| | |
|---|---|
| `<FILE>` | `.aab` to upload |
| `--track` | `internal`, `alpha`, `beta`, `production` or a custom track (required) |
| `--release-status` | `completed` (default) or `draft` |
| `--key-file` | Google service account JSON key |
| `--package-name` | e.g. `com.example.app` |
| `-v`, `--json` | more output / JSON output (does nothing yet) |

Run `leleka play upload --help` for details.

App Store Connect support is coming later.
