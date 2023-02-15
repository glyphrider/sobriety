# My sobriety calculator

## Requirements

1. Generate a visual representation of a sobriety coin with roman numerals
2. Display the number of days of continuous sobriety (_how many twenty-four-hourses_)

## Implementation

Leverage [Yew framework](https://yew.rs) to create a simple web page with a _React-like_ component to display the above requirements. _Use state_ to wake up the app every few seconds and process a (potentially) new value of **now**.

I leveraged my own [roman.rs](https://github.com/glyphrider/roman.rs) kata project to do the conversion from arabic (rust: `u16`) to roman (rust: `String`) numerals.

The background _art_ image is my own: a photo taken with my [Pixel 7 Pro](https://store.google.com/product/pixel_7_pro) of a page in my [Amazon Basics notebook](https://smile.amazon.com/gp/product/B01DN8TEA2).

The content, produced by [trunk](https://trunkrs.dev) (specifically `trunk build`), and copied to an Amazon S3 bucket using the aws-cli-v2 (aws s3 sync --delete dist/ s3://_bucket-name_). There is a CloudFront distribution in front of that. I should have some details in here about how that's setup....