using Libdl
using Test

# Core Q16.16 constants and C-ABI structures
const Q16 = Int32
const Q16_ZERO = Q16(0)
const Q16_HALF = Q16(1 << 15)
const Q16_ONE  = Q16(1 << 16)

struct SymplecticStateQ16
    q::NTuple{40, Q16}
    p::NTuple{40, Q16}
end

# Semantic boundary slices
const WORLD_RULES_START       = 1
const CHAR_AFFINITY_START     = 21
const QUEST_COORDINATES_START = 41
const ENV_TENSION_START       = 61
