//! Star Trek character names and descriptions library.
//! This library provides a collection of Star Trek character names and their descriptions
//! across various series and shows.

use phf::phf_map;

/// Static map containing all Star Trek character descriptions
pub static TREK_DESCRIPTIONS: phf::Map<&'static str, &'static str> = phf_map! {
    // The Original Series
    "Kirk" => "Captain James T. Kirk, charismatic leader of the USS Enterprise, known for his tactical brilliance, diplomatic skills, and tendency to find unconventional solutions to impossible situations",
    "Spock" => "Half-human, half-Vulcan First Officer and Science Officer, characterized by his logical approach and internal struggle between emotion and logic, serving as the Enterprise's conscience",
    "McCoy" => "Dr. Leonard 'Bones' McCoy, passionate Chief Medical Officer, known for his humanistic perspective, Southern charm, and frequent philosophical debates with Spock about logic versus emotion",
    "Sulu" => "Helmsman Hikaru Sulu, skilled pilot and fencer with expertise in botany, later became captain of the USS Excelsior, known for his calm demeanor under pressure",
    "Uhura" => "Communications Officer Nyota Uhura, pioneering officer known for her exceptional linguistic abilities, musical talent, and groundbreaking representation in television history",
    "Scotty" => "Montgomery Scott, miracle-working Chief Engineer famous for his technical ingenuity, deep love for the Enterprise, and ability to exceed expectations under impossible deadlines",
    "Chekov" => "Navigator Pavel Chekov, young enthusiastic officer known for his Russian pride, claiming everything was invented in Russia, and his expertise in navigation and weapons systems",
    "Chapel" => "Nurse Christine Chapel, dedicated medical professional who served under Dr. McCoy, known for her unrequited feelings for Spock and her transition from researcher to medical practitioner",
    "Rand" => "Yeoman Janice Rand, efficient personal assistant to Captain Kirk during the Enterprise's first year of mission, known for her distinctive basketweave hairdo and professional demeanor",
    "Pike" => "Captain Christopher Pike, Kirk's predecessor as captain of the Enterprise, later severely injured saving cadets and confined to a life support chair, embodying self-sacrifice",
    "Number1" => "First Officer under Captain Pike, known for her cool logic and efficiency, setting a precedent for female officers in command positions",
    "Sarek" => "Spock's father, a respected Vulcan ambassador to the Federation, known for his diplomatic skills and complex relationship with his half-human son",
    "Amanda" => "Amanda Grayson, Spock's human mother and wife to Sarek, who balanced Vulcan logic with human emotion and helped bridge two worlds",
    "TKuvma" => "Charismatic Klingon leader who sought to unite the Klingon houses against the Federation, believing in maintaining Klingon purity and traditions",
    "Harry-Mudd" => "Harcourt Fenton Mudd, notorious con man and frequent thorn in Kirk's side, known for his schemes involving androids, love potions, and various illegal enterprises",

    // The Next Generation
    "Picard" => "Captain Jean-Luc Picard, diplomatic and philosophical leader of the Enterprise-D, known for his intellectual approach to command and dedication to Federation principles",
    "Riker" => "Commander William T. Riker, Picard's trusted first officer, known for his leadership skills, tactical expertise, and trademark beard",
    "Data" => "Android officer seeking to understand human nature, serving as Operations Officer and Second Officer, characterized by his quest for humanity",
    "Worf" => "First Klingon Starfleet officer, Chief of Security, struggling between his Klingon heritage and Starfleet duties",
    "Crusher" => "Dr. Beverly Crusher, brilliant Chief Medical Officer, balancing her medical duties with raising her son Wesley and a complex relationship with Picard",
    "Troi" => "Counselor Deanna Troi, half-Betazoid empath who serves as ship's counselor and advisor, helping crew navigate emotional challenges",
    "LaForge" => "Geordi La Forge, innovative Chief Engineer who can solve any technical problem, known for his VISOR and friendship with Data",

    // Deep Space Nine
    "Sisko" => "Commander (later Captain) Benjamin Sisko, Emissary of the Prophets and commanding officer of Deep Space Nine, known for his strategic mind and role in the Dominion War",
    "Dax" => "Jadzia Dax, joined Trill science officer combining the wisdom of multiple lifetimes with a vibrant personality",
    "Bashir" => "Dr. Julian Bashir, brilliant genetically enhanced medical officer who balanced his superior abilities with genuine compassion",
    "Kira" => "Major Kira Nerys, former Bajoran resistance fighter serving as first officer, fierce defender of Bajoran independence",
    "Odo" => "The station's shapeshifting Chief of Security, struggling with his identity as a Changeling while maintaining order",
    "OBrien" => "Chief Miles O'Brien, transferred from the Enterprise-D, keeping the station running through technical expertise and determination",
    "Quark" => "Ferengi bar owner and occasional information broker, balancing profit with an unlikely conscience",

    // Voyager
    "Janeway" => "Captain Kathryn Janeway, determined leader of the USS Voyager, combining scientific curiosity with unwavering dedication to getting her crew home",
    "Chakotay" => "First Officer and former Maquis leader, bringing his spiritual wisdom and tactical experience to support Janeway",
    "Tuvok" => "Vulcan Chief of Security and Janeway's trusted friend, maintaining logic and discipline while serving as a bridge between Starfleet and Maquis crews",
    "Paris" => "Lieutenant Tom Paris, reformed convict turned ace pilot, finding redemption and purpose as Voyager's helmsman",
    "Torres" => "B'Elanna Torres, half-Klingon Chief Engineer who struggled with her dual heritage while keeping Voyager running against impossible odds",
    "Kim" => "Ensign Harry Kim, young operations officer who maintained his optimism despite being stranded far from home",
    "Seven" => "Seven of Nine, former Borg drone reclaiming her humanity while bringing unique perspectives and technological expertise",

    // Enterprise
    "Archer" => "Captain Jonathan Archer, pioneer of deep space exploration and first captain of the Enterprise NX-01, instrumental in forming the Federation",
    "TPol" => "Vulcan science officer who served as Archer's second-in-command, gradually embracing human emotions and customs",
    "Tucker" => "Commander Charles 'Trip' Tucker III, chief engineer and Archer's best friend, known for his innovative solutions and Southern charm",
    "Reed" => "Lieutenant Malcolm Reed, tactical officer obsessed with security protocols and proper military discipline",
    "Sato" => "Ensign Hoshi Sato, linguistics expert who developed the first Universal Translator, overcoming her fear of space travel",
    "Mayweather" => "Ensign Travis Mayweather, skilled pilot born and raised in space on cargo ships, bringing unique 'boomer' perspective",
    "Phlox" => "Dr. Phlox, Denobulan physician combining multiple medical traditions with curiosity about human culture",

    // Discovery
    "Burnham" => "Michael Burnham, raised on Vulcan by Sarek, whose actions sparked a war but later became instrumental in saving the Federation across centuries",
    "Saru" => "Kelpien officer who overcame his species' inherent fear to become one of Starfleet's most distinguished captains",
    "Tilly" => "Sylvia Tilly, enthusiastic officer who grew from insecure cadet to confident leader while maintaining her authentic personality",
    "Stamets" => "Lt. Paul Stamets, astromycologist and pioneer of the spore drive, connected to the mycelial network",
    "Culber" => "Dr. Hugh Culber, compassionate medical officer who returned from death in the mycelial network",

    // Notable Villains
    "Khan" => "Khan Noonien Singh, genetically enhanced superhuman from Earth's past, one of Kirk's most formidable adversaries",
    "Borg-Queen" => "Central nexus of the Borg Collective, embodying their drive for perfection through assimilation",
    "Dukat" => "Former Cardassian prefect of Bajor turned recurring antagonist, complex villain believing himself the hero of his own story",
    "Locutus" => "Picard's identity when assimilated by the Borg, used to facilitate the invasion of Federation space",
    "Lore" => "Data's emotionally unstable android brother who allied with rogue Borg and threatened the Federation"
};

/// Returns a vector of all character names
pub fn get_all_names() -> Vec<&'static str> {
    TREK_DESCRIPTIONS.keys().copied().collect()
}

/// Returns the description for a given character name, if it exists
pub fn get_description(name: &str) -> Option<&'static str> {
    TREK_DESCRIPTIONS.get(name).copied()
}

/// Returns true if the character exists in the database
pub fn character_exists(name: &str) -> bool {
    TREK_DESCRIPTIONS.contains_key(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_description() {
        assert!(get_description("Kirk").is_some());
        assert!(get_description("NonExistentCharacter").is_none());
    }

    #[test]
    fn test_character_exists() {
        assert!(character_exists("Picard"));
        assert!(!character_exists("NonExistentCharacter"));
    }

    #[test]
    fn test_get_all_names() {
        let names = get_all_names();
        assert!(!names.is_empty());
        assert!(names.contains(&"Kirk"));
        assert!(names.contains(&"Picard"));
    }
}
