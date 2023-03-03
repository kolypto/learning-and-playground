# Unlike variables, locals do not change their value
# Unlike variables, locels can use dynamic expressions and resource arguments!
locals {
    # Use it: "${local.container_name}"
    container_name = "hello-${random_pet.dog.id}"
}