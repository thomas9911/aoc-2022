cargo install bindgen-cli
bindgen -o "src/wrapper/bindings.rs" --generate "functions,methods,constructors,destructors" "lib/wrapper.h"
