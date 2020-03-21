# Niels Style Sheets

Making a new CSS super-set in one quarantine evening.

## Syntax

`test.nss`
```rb
@variable = red
@cool_image = "/assets/cool_image.jpeg"

body, h1
  color: blue
  background: url(@cool_image)!

a, button
  color: @variable
```

This outputs following

`test.css`
```css
body, h1 {
  color: blue;
  background: url("/assets/cool_image.jpeg") !important;
}

a, button {
  color: red;
}
```

## Details

- Start: March 20, 22:03
- End: March 21, 00:55

## License

MIT for now
