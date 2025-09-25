use crate::ui::{
    AURORA_GREEN, CELESTIAL_BLUE, COMET_ORANGE, GALAXY_PINK, METEOR_RED, NEBULA_PURPLE,
    PLASMA_CYAN, SOLAR_YELLOW, STARLIGHT,
};
use rand::prelude::*;
use ratatui::style::Color;
use std::sync::LazyLock;

#[derive(Clone)]
pub struct ColoredMessage {
    pub text: String,
    pub color: Color,
}

static WAITING_MESSAGES: LazyLock<Vec<ColoredMessage>> = LazyLock::new(|| {
    vec![
        ColoredMessage {
            text: "🔮 Consulting the cosmic commit oracle...".to_string(),
            color: NEBULA_PURPLE,
        },
        ColoredMessage {
            text: "🌌 Aligning the celestial code spheres...".to_string(),
            color: CELESTIAL_BLUE,
        },
        ColoredMessage {
            text: "👻 Channeling the spirit of clean commits...".to_string(),
            color: AURORA_GREEN,
        },
        ColoredMessage {
            text: "🚀 Launching commit ideas into the coding cosmos...".to_string(),
            color: METEOR_RED,
        },
        ColoredMessage {
            text: "🌠 Exploring the galaxy of potential messages...".to_string(),
            color: PLASMA_CYAN,
        },
        ColoredMessage {
            text: "🔭 Peering into the commit-verse for inspiration...".to_string(),
            color: SOLAR_YELLOW,
        },
        ColoredMessage {
            text: "🧙 Casting a spell for the perfect commit message...".to_string(),
            color: GALAXY_PINK,
        },
        ColoredMessage {
            text: "✨ Harnessing the power of a thousand code stars...".to_string(),
            color: STARLIGHT,
        },
        ColoredMessage {
            text: "🪐 Orbiting the planet of precise git descriptions...".to_string(),
            color: CELESTIAL_BLUE,
        },
        ColoredMessage {
            text: "🎨 Weaving a tapestry of colorful commit prose...".to_string(),
            color: PLASMA_CYAN,
        },
        ColoredMessage {
            text: "🎇 Igniting the fireworks of code brilliance...".to_string(),
            color: COMET_ORANGE,
        },
        ColoredMessage {
            text: "🧠 Syncing with the collective coding consciousness...".to_string(),
            color: AURORA_GREEN,
        },
        ColoredMessage {
            text: "🌙 Aligning the moon phases for optimal commit clarity...".to_string(),
            color: STARLIGHT,
        },
        ColoredMessage {
            text: "🔬 Analyzing code particles at the quantum level...".to_string(),
            color: NEBULA_PURPLE,
        },
        ColoredMessage {
            text: "🧬 Decoding the DNA of your changes...".to_string(),
            color: GALAXY_PINK,
        },
        ColoredMessage {
            text: "🏺 Summoning the ancient spirits of version control...".to_string(),
            color: METEOR_RED,
        },
        ColoredMessage {
            text: "📡 Tuning into the frequency of flawless commits...".to_string(),
            color: CELESTIAL_BLUE,
        },
        ColoredMessage {
            text: "💎 Charging the commit crystals with cosmic energy...".to_string(),
            color: PLASMA_CYAN,
        },
        ColoredMessage {
            text: "🌍 Translating your changes into universal code...".to_string(),
            color: AURORA_GREEN,
        },
        ColoredMessage {
            text: "🧪 Distilling the essence of your modifications...".to_string(),
            color: SOLAR_YELLOW,
        },
        ColoredMessage {
            text: "🕸️ Unraveling the threads of your code tapestry...".to_string(),
            color: NEBULA_PURPLE,
        },
        ColoredMessage {
            text: "🦉 Consulting the all-knowing git guardians...".to_string(),
            color: CELESTIAL_BLUE,
        },
        ColoredMessage {
            text: "🎵 Harmonizing with the rhythms of the coding universe...".to_string(),
            color: GALAXY_PINK,
        },
        ColoredMessage {
            text: "🌊 Diving into the depths of the code ocean...".to_string(),
            color: PLASMA_CYAN,
        },
        ColoredMessage {
            text: "🧓 Seeking wisdom from the repository sages...".to_string(),
            color: AURORA_GREEN,
        },
        ColoredMessage {
            text: "🧭 Calibrating the commit compass for true north...".to_string(),
            color: SOLAR_YELLOW,
        },
        ColoredMessage {
            text: "🔐 Unlocking the secrets of the commit constellations...".to_string(),
            color: NEBULA_PURPLE,
        },
        ColoredMessage {
            text: "⭐ Gathering stardust for your stellar commit...".to_string(),
            color: STARLIGHT,
        },
        ColoredMessage {
            text: "🔎 Focusing the lens of the code telescope...".to_string(),
            color: CELESTIAL_BLUE,
        },
        ColoredMessage {
            text: "🏄 Riding the waves of inspiration through the code cosmos...".to_string(),
            color: PLASMA_CYAN,
        },
    ]
});

static REVIEW_WAITING_MESSAGES: LazyLock<Vec<ColoredMessage>> = LazyLock::new(|| {
    vec![
        ColoredMessage {
            text: "🔎 Scanning code dimensions for quality signatures...".to_string(),
            color: NEBULA_PURPLE,
        },
        ColoredMessage {
            text: "🌌 Traversing the architecture cosmos for patterns...".to_string(),
            color: CELESTIAL_BLUE,
        },
        ColoredMessage {
            text: "🛡️ Invoking the guardians of code integrity...".to_string(),
            color: AURORA_GREEN,
        },
        ColoredMessage {
            text: "✨ Illuminating shadow bugs with code starlight...".to_string(),
            color: STARLIGHT,
        },
        ColoredMessage {
            text: "🔮 Gazing into the crystal orb of future maintainability...".to_string(),
            color: PLASMA_CYAN,
        },
        ColoredMessage {
            text: "📜 Unrolling the ancient scrolls of best practices...".to_string(),
            color: SOLAR_YELLOW,
        },
        ColoredMessage {
            text: "🧪 Distilling your code into its purest essence...".to_string(),
            color: GALAXY_PINK,
        },
        ColoredMessage {
            text: "⚖️ Weighing your code on the scales of elegance...".to_string(),
            color: CELESTIAL_BLUE,
        },
        ColoredMessage {
            text: "🌈 Tracing the rainbow paths between your functions...".to_string(),
            color: AURORA_GREEN,
        },
        ColoredMessage {
            text: "🔍 Magnifying the subtle harmonies in your algorithms...".to_string(),
            color: NEBULA_PURPLE,
        },
        ColoredMessage {
            text: "🧠 Communing with the collective wisdom of master coders...".to_string(),
            color: METEOR_RED,
        },
        ColoredMessage {
            text: "🌊 Diving into the depths of your code ocean...".to_string(),
            color: PLASMA_CYAN,
        },
        ColoredMessage {
            text: "🗿 Consulting the monoliths of software architecture...".to_string(),
            color: COMET_ORANGE,
        },
        ColoredMessage {
            text: "⏳ Sifting through the time sands of execution paths...".to_string(),
            color: SOLAR_YELLOW,
        },
        ColoredMessage {
            text: "🧩 Assembling the puzzle pieces of your code story...".to_string(),
            color: GALAXY_PINK,
        },
        ColoredMessage {
            text: "🔬 Analyzing code particles at quantum precision...".to_string(),
            color: CELESTIAL_BLUE,
        },
        ColoredMessage {
            text: "🌟 Measuring the brightness of your code stars...".to_string(),
            color: STARLIGHT,
        },
        ColoredMessage {
            text: "🧵 Following the threads of logic throughout your tapestry...".to_string(),
            color: AURORA_GREEN,
        },
        ColoredMessage {
            text: "🔱 Summoning the trident of code quality dimensions...".to_string(),
            color: NEBULA_PURPLE,
        },
        ColoredMessage {
            text: "🌀 Spiraling through nested layers of abstraction...".to_string(),
            color: PLASMA_CYAN,
        },
        ColoredMessage {
            text: "🏺 Examining the ancient artifacts of your repository...".to_string(),
            color: METEOR_RED,
        },
        ColoredMessage {
            text: "🎭 Unmasking the hidden characters in your code drama...".to_string(),
            color: GALAXY_PINK,
        },
        ColoredMessage {
            text: "🧿 Warding off evil bugs with protective insights...".to_string(),
            color: CELESTIAL_BLUE,
        },
        ColoredMessage {
            text: "🔥 Forging stronger code in the flames of analysis...".to_string(),
            color: COMET_ORANGE,
        },
        ColoredMessage {
            text: "🌱 Nurturing the seeds of excellence in your codebase...".to_string(),
            color: AURORA_GREEN,
        },
        ColoredMessage {
            text: "🎯 Pinpointing opportunities for cosmic refinement...".to_string(),
            color: SOLAR_YELLOW,
        },
        ColoredMessage {
            text: "🕸️ Mapping the intricate web of dependencies...".to_string(),
            color: NEBULA_PURPLE,
        },
        ColoredMessage {
            text: "🔧 Calibrating the tools of code enlightenment...".to_string(),
            color: PLASMA_CYAN,
        },
        ColoredMessage {
            text: "🧮 Computing the algorithms of optimal elegance...".to_string(),
            color: STARLIGHT,
        },
        ColoredMessage {
            text: "🌠 Charting the trajectory of your code evolution...".to_string(),
            color: CELESTIAL_BLUE,
        },
    ]
});

static USER_MESSAGES: LazyLock<Vec<ColoredMessage>> = LazyLock::new(|| {
    vec![
        ColoredMessage {
            text: "🚀 Launching commit rocket".to_string(),
            color: METEOR_RED,
        },
        ColoredMessage {
            text: "🌟 Illuminating code cosmos".to_string(),
            color: STARLIGHT,
        },
        ColoredMessage {
            text: "🔭 Observing code constellations".to_string(),
            color: CELESTIAL_BLUE,
        },
        ColoredMessage {
            text: "🧙‍♂️ Weaving code enchantments".to_string(),
            color: GALAXY_PINK,
        },
        ColoredMessage {
            text: "⚛️ Splitting code atoms".to_string(),
            color: PLASMA_CYAN,
        },
        ColoredMessage {
            text: "🌈 Painting commit rainbows".to_string(),
            color: AURORA_GREEN,
        },
        ColoredMessage {
            text: "🔑 Unlocking git portals".to_string(),
            color: SOLAR_YELLOW,
        },
        ColoredMessage {
            text: "🎭 Staging code drama".to_string(),
            color: COMET_ORANGE,
        },
        ColoredMessage {
            text: "🌌 Expanding code universe".to_string(),
            color: NEBULA_PURPLE,
        },
        ColoredMessage {
            text: "🏹 Aiming commit arrows".to_string(),
            color: METEOR_RED,
        },
        ColoredMessage {
            text: "🎨 Brushing commit strokes".to_string(),
            color: PLASMA_CYAN,
        },
        ColoredMessage {
            text: "🌱 Growing code forests".to_string(),
            color: AURORA_GREEN,
        },
        ColoredMessage {
            text: "🧩 Assembling code puzzle".to_string(),
            color: GALAXY_PINK,
        },
        ColoredMessage {
            text: "🎶 Orchestrating commit symphony".to_string(),
            color: CELESTIAL_BLUE,
        },
        ColoredMessage {
            text: "⚖️ Balancing code forces".to_string(),
            color: SOLAR_YELLOW,
        },
    ]
});

pub fn get_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| ColoredMessage {
            text: "Processing your request...".to_string(),
            color: SOLAR_YELLOW,
        })
}

pub fn get_review_waiting_message() -> ColoredMessage {
    let mut rng = rand::rng();
    REVIEW_WAITING_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| ColoredMessage {
            text: "Analyzing your code quality...".to_string(),
            color: NEBULA_PURPLE,
        })
}

pub fn get_user_message() -> ColoredMessage {
    let mut rng = rand::rng();
    USER_MESSAGES
        .choose(&mut rng)
        .cloned()
        .unwrap_or_else(|| ColoredMessage {
            text: "What would you like to do?".to_string(),
            color: CELESTIAL_BLUE,
        })
}
