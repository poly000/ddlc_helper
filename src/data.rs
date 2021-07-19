//! the words are from [DDLC wiki](https://ddlcwiki.net/wiki/Poem_game)
use std::collections::HashSet;
use std::lazy::SyncLazy;

static SAYORI_WORDS: [&'static str; 88] = [
    "adventure",
    "alone",
    "amazing",
    "awesome",
    "beauty",
    "bed",
    "bliss",
    "broken",
    "calm",
    "charm",
    "cheer",
    "childhood",
    "clumsy",
    "color",
    "comfort",
    "cry",
    "dance",
    "dark",
    "daydream",
    "dazzle",
    "death",
    "defeat",
    "depression",
    "embrace",
    "empty",
    "excitement",
    "extraordinary",
    "family",
    "fear",
    "feather",
    "fireflies",
    "fireworks",
    "flower",
    "flying",
    "forgive",
    "friends",
    "fun",
    "grief",
    "happiness",
    "heart",
    "holiday",
    "hope",
    "hopeless",
    "hurt",
    "joy",
    "laugh",
    "lazy",
    "loud",
    "love",
    "lucky",
    "marriage",
    "memories",
    "misery",
    "misfortune",
    "music",
    "nature",
    "ocean",
    "pain",
    "party",
    "passion",
    "peaceful",
    "play",
    "prayer",
    "precious",
    "promise",
    "rainbow",
    "raincloud",
    "romance",
    "rose",
    "sadness",
    "scars",
    "shame",
    "silly",
    "sing",
    "smile",
    "sparkle",
    "special",
    "sunny",
    "sunset",
    "sweet",
    "tears",
    "together",
    "tragedy",
    "treasure",
    "unrequited",
    "vacation",
    "warm",
    "wonderful",
];

static YURI_WORDS: [&'static str; 78] = [
    "afterimage",
    "agonizing",
    "ambient",
    "analysis",
    "anxiety",
    "atone",
    "aura",
    "breathe",
    "cage",
    "captive",
    "climax",
    "contamination",
    "covet",
    "crimson",
    "desire",
    "despise",
    "destiny",
    "determination",
    "disarray",
    "disaster",
    "disoriented",
    "disown",
    "dream",
    "effulgent",
    "electricity",
    "entropy",
    "essence",
    "eternity",
    "existence",
    "explode",
    "extreme",
    "fester",
    "fickle",
    "flee",
    "frightening",
    "graveyard",
    "heavensent",
    "horror",
    "imagination",
    "incapable",
    "incongruent",
    "infallible",
    "inferno",
    "infinite",
    "insight",
    "intellectual",
    "journey",
    "judgment",
    "landscape",
    "lust",
    "massacre",
    "meager",
    "melancholy",
    "philosophy",
    "pleasure",
    "portrait",
    "question",
    "raindrops",
    "secretive",
    "sensation",
    "starscape",
    "suicide",
    "tenacious",
    "time",
    "uncanny",
    "uncontrollable",
    "unending",
    "universe",
    "unrestrained",
    "unstable",
    "variance",
    "vertigo",
    "vibrant",
    "vitality",
    "vivacious",
    "vivid",
    "whirlwind",
    "wrath",
];

static NATSUKI_WORDS: [&'static str; 62] = [
    "anger",
    "anime",
    "blanket",
    "boop",
    "bouncy",
    "bubbles",
    "bunny",
    "candy",
    "cheeks",
    "chocolate",
    "clouds",
    "cute",
    "doki",
    "email",
    "fantasy",
    "fluffy",
    "games",
    "giggle",
    "hair",
    "headphones",
    "heartbeat",
    "hop",
    "jump",
    "jumpy",
    "kawaii",
    "kiss",
    "kitty",
    "lipstick",
    "lollipop",
    "marshmallow",
    "melody",
    "milk",
    "mouse",
    "nibble",
    "nightgown",
    "papa",
    "parfait",
    "peace",
    "pink",
    "playground",
    "poof",
    "pout",
    "puppy",
    "pure",
    "ribbon",
    "shiny",
    "shopping",
    "skipping",
    "skirt",
    "socks",
    "spinning",
    "sticky",
    "strawberry",
    "sugar",
    "summer",
    "swimsuit",
    "twirl",
    "valentine",
    "vanilla",
    "waterfall",
    "whisper",
    "whistle",
];

pub static SAYORI_WORDS_SET: SyncLazy<HashSet<&'static str>> =
    SyncLazy::new(|| SAYORI_WORDS.iter().map(|refe| *refe).collect());

pub static YURI_WORDS_SET: SyncLazy<HashSet<&'static str>> =
    SyncLazy::new(|| YURI_WORDS.iter().map(|refe| *refe).collect());

pub static NATSUKI_WORDS_SET: SyncLazy<HashSet<&'static str>> =
    SyncLazy::new(|| NATSUKI_WORDS.iter().map(|refe| *refe).collect());
