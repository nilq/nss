@variable = red
@cool_image = "/assets/cool_image.jpeg"

body, h1
  color: blue
  background: url(@cool_image)!

a, button
  color: @variable
