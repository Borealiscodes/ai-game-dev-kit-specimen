import Lake
open Lake DSL

package «enneract-santa» where
  version := v!"0.1.0"

@[default_target]
lean_lib «EnneractSanta» where
  roots := #[`EnneractSanta.Core, `EnneractSanta.PersonaZero]
