# My sobriety calculator

## Requirements

1. Generate a visual representation of a sobriety coin with roman numerals
2. Display the number of days of continuous sobriety (_how many twenty-four-hourses_)

## Implementation

Leverage [Yew framework](https://yew.rs) to create a simple web page with a _React-like_ component to display the above requirements. _Use state_ to wake up the app every few seconds and process a (potentially) new value of **now**.

I leveraged my own [roman.rs](https://github.com/glyphrider/roman.rs) kata project to do the conversion from arabic (rust: `u16`) to roman (rust: `String`) numerals.

The background _art_ image is my own: a photo taken with my [Pixel 7 Pro](https://store.google.com/product/pixel_7_pro) of a page in my [Amazon Basics notebook](https://smile.amazon.com/gp/product/B01DN8TEA2).

The content is produced by [trunk](https://trunkrs.dev) (specifically `trunk build`). This requires a bit of setup. Firstly install trunk (globally) with `cargo install --locked trunk`, then add the rust wasm target with `rustup target add wasm32-unknown-unknown`. I also needed a wasm-binding which I installed via `cargo install --locked wasm-bindgen-cli`.

Once the dist directory is populated, the files were copied to an Amazon S3 bucket using the aws-cli-v2 (`aws s3 sync --delete dist/ s3://sobriety --profile sobriety`). There is a CloudFront distribution in front of that. See "AWS setup" below for how the bucket, CloudFront, DNS, and deploy credentials are all wired together.

## AWS setup

Everything below assumes an AWS account with a Route 53 hosted zone for the domain you want to serve from (this deployment uses `wardtalks.com`, serving the site from `sobriety.wardtalks.com`). Replace `<account-id>` and the domain with your own if rebuilding this in a new account — resource IDs (bucket name, distribution ID, OAI ID, cert ARN) are all freshly generated per account/region and aren't reusable across accounts. Note that S3 bucket names are globally unique across *all* AWS accounts, so `sobriety` may need to change if it's already taken elsewhere.

### Hosting: S3 + CloudFront

The site is a fully private S3 bucket — no static website hosting, no public access — read only by CloudFront via an Origin Access Identity (OAI). Steps to recreate:

1. Create the S3 bucket (`sobriety`, region `us-east-2` in the original setup), with all four "Block Public Access" settings left **on**.
2. Request an ACM certificate **in `us-east-1`** (required for CloudFront regardless of where the bucket lives) for the domain, e.g. `sobriety.wardtalks.com`, validated via DNS (adds a CNAME record to the hosted zone).
3. Create a CloudFront Origin Access Identity and set the S3 bucket policy to allow that OAI's `s3:GetObject` on `arn:aws:s3:::sobriety/*` (and nothing else/no one else — public access stays blocked).
4. Create a CloudFront distribution:
   - Origin: the S3 bucket, via the OAI from step 3
   - Alternate domain name (CNAME): `sobriety.wardtalks.com`
   - Viewer certificate: the ACM cert from step 2
   - Default root object: `index.html`
   - Cache policy: `Managed-CachingOptimized`
   - Response headers policy: `Managed-SimpleCORS`
5. In Route 53, add an `A` record (alias) for `sobriety.wardtalks.com` pointing at the CloudFront distribution's domain name (alias target hosted zone is CloudFront's fixed zone ID `Z2FDTNDATAQYW2`).

### Deploy credentials: IAM role + group

Deploys use a dedicated IAM role rather than a long-lived access key, following the same pattern as this account's other projects (`fieldnotes-deploy`, `mothdeck-role`, etc.):

1. **Role** `sobriety-deploy` — trust policy lets the account assume it:
   ```json
   {
       "Version": "2012-10-17",
       "Statement": [
           {
               "Effect": "Allow",
               "Principal": { "AWS": "arn:aws:iam::<account-id>:root" },
               "Action": "sts:AssumeRole"
           }
       ]
   }
   ```
2. **Inline policy** on that role, `sobriety-deploy-s3`, scoped to just the one bucket:
   ```json
   {
       "Version": "2012-10-17",
       "Statement": [
           {
               "Effect": "Allow",
               "Action": ["s3:PutObject", "s3:GetObject", "s3:DeleteObject", "s3:ListBucket"],
               "Resource": ["arn:aws:s3:::sobriety", "arn:aws:s3:::sobriety/*"]
           }
       ]
   }
   ```
   (`DeleteObject` is needed because deploys use `aws s3 sync --delete`.)
3. **Group** `sobriety-deployers` with an inline policy `sobriety-deploy-assume-role` granting `sts:AssumeRole` on `arn:aws:iam::<account-id>:role/sobriety-deploy` — an IAM user needs both the role's trust policy *and* an identity-based policy like this to actually assume it.
4. Add your IAM user to the `sobriety-deployers` group.
5. Add a profile to `~/.aws/config` so the role is easy to use locally:
   ```ini
   [profile sobriety]
   source_profile = <your base profile>
   role_arn = arn:aws:iam::<account-id>:role/sobriety-deploy
   ```

With that in place, `aws s3 sync --delete dist/ s3://sobriety --profile sobriety` (or `awsume sobriety`, available via `shell.nix`) deploys without needing broader account permissions.

## Development environment

Rather than installing trunk, the wasm target, and wasm-bindgen-cli by hand, there's a `shell.nix` that provisions all of it (plus the test tooling below). Drop into it with:

```
nix-shell
```

From inside the shell, `trunk serve` and `trunk build` work as described above.

## Tests

There are two test suites, since testing rendered components needs a real browser:

- Pure calculation logic (the roman-numeral and day-count math in `src/lib.rs`) is covered by plain unit tests, run with:
  ```
  cargo test
  ```
- The `Coin` and `ContinuousSobriety` components are covered by rendering them into a headless browser and checking the output, run with:
  ```
  wasm-pack test --headless --firefox
  ```
  (`shell.nix` includes `wasm-pack`, `geckodriver`, and `firefox` for this.)