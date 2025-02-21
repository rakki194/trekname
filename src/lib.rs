#![warn(clippy::all, clippy::pedantic)]

//! Star Trek character names and descriptions library.
//! This library provides a collection of Star Trek character names and their descriptions
//! across various series and shows.

use phf::phf_map;

/// Static map containing all Star Trek character descriptions
pub static TREK_DESCRIPTIONS: phf::Map<&'static str, &'static str> = phf_map! {
    // The Original Series (TOS)
    "Kirk" => "Captain James T. Kirk, charismatic leader of the USS Enterprise, known for his tactical brilliance, diplomatic skills, and tendency to find unconventional solutions to impossible situations",
    "Spock" => "Half-human, half-Vulcan First Officer and Science Officer, characterized by his logical approach and internal struggle between emotion and logic, serving as the Enterprise's conscience",
    "McCoy" => "Dr. Leonard 'Bones' McCoy, passionate Chief Medical Officer, known for his humanistic perspective, Southern charm, and frequent philosophical debates with Spock about logic versus emotion",
    "Sulu" => "Helmsman Hikaru Sulu, skilled pilot and fencer with expertise in botany, later became captain of the USS Excelsior, known for his calm demeanor under pressure",
    "Uhura" => "Communications Officer Nyota Uhura, pioneering officer known for her exceptional linguistic abilities, musical talent, and groundbreaking representation in television history",
    "Scotty" => "Montgomery Scott, miracle-working Chief Engineer famous for his technical ingenuity, deep love for the Enterprise, and ability to exceed expectations under impossible deadlines",
    "Chekov" => "Navigator Pavel Chekov, young enthusiastic officer known for his Russian pride, claiming everything was invented in Russia, and his expertise in navigation and weapons systems",
    "Chapel" => "Nurse Christine Chapel, dedicated medical professional who served under Dr. McCoy, known for her unrequited feelings for Spock and her transition from researcher to medical practitioner",
    "Rand" => "Yeoman Janice Rand, efficient personal assistant to Captain Kirk during the Enterprise's first year of mission, later promoted to transporter chief",
    "Pike" => "Captain Christopher Pike, Kirk's predecessor as captain of the Enterprise, later severely injured saving cadets and confined to a life support chair, embodying self-sacrifice",
    "Sarek" => "Spock's father, a respected Vulcan ambassador to the Federation, known for his diplomatic skills and complex relationship with his half-human son",
    "Amanda" => "Amanda Grayson, Spock's human mother and wife to Sarek, who balanced Vulcan logic with human emotion and helped bridge two worlds",
    "Harry-Mudd" => "Harcourt Fenton Mudd, notorious con man and frequent thorn in Kirk's side, known for his schemes involving androids, love potions, and various illegal enterprises",
    "MBenga" => "Dr. M'Benga, Enterprise medical officer with expertise in Vulcan physiology, serving in sickbay under Dr. McCoy",
    "Kyle" => "Lieutenant Kyle, transporter chief and relief helmsman on the Enterprise, known for his reliability in crisis situations",

    // The Animated Series (TAS)
    "Arex" => "Lieutenant Arex, Edosian navigator with three arms and legs, bringing unique alien perspective to the Enterprise bridge crew",
    "MRess" => "Lieutenant M'Ress, Caitian communications officer who served as relief for Uhura, known for her feline characteristics and loyalty",
    "Gabler" => "Chief Engineer Gabler, who occasionally relieved Scotty in Engineering during the Enterprise's animated adventures",
    "Thelin" => "Commander Thelin, Andorian officer who served as Science Officer in an alternate timeline where Spock died as a child",
    "Carter" => "Lieutenant Carter, Enterprise security officer who appeared in multiple animated missions",
    
    // The Next Generation (TNG)
    "Picard" => "Captain Jean-Luc Picard, diplomatic and philosophical leader of the Enterprise-D, known for his intellectual approach to command and dedication to Federation principles",
    "Riker" => "Commander William T. Riker, Picard's trusted first officer, known for his leadership skills, tactical expertise, and trademark beard",
    "Data" => "Android officer seeking to understand human nature, serving as Operations Officer and Second Officer, characterized by his quest for humanity",
    "Worf" => "First Klingon Starfleet officer, Chief of Security, struggling between his Klingon heritage and Starfleet duties",
    "Crusher" => "Dr. Beverly Crusher, Enterprise-D's Chief Medical Officer, brilliant physician and single mother to Wesley, known for her strong ethical principles and close relationship with Picard",
    "Troi" => "Counselor Deanna Troi, half-Betazoid empath who serves as ship's counselor and advisor, helping crew navigate emotional challenges",
    "LaForge" => "Geordi La Forge, innovative Chief Engineer who can solve any technical problem, known for his VISOR and friendship with Data",
    "Pulaski" => "Dr. Katherine Pulaski, Enterprise-D's Chief Medical Officer during Dr. Crusher's absence, known for her old-school medical approach and initial skepticism of Data",
    "Guinan" => "El-Aurian bartender and confidante to the Enterprise-D crew, centuries-old wisdom figure with a mysterious past and special connection to Picard",
    "Alexander" => "Alexander Rozhenko, Worf's son who struggled to balance his Human and Klingon heritage while living aboard the Enterprise",
    "Lwaxana" => "Lwaxana Troi, Deanna's mother and Betazoid ambassador, flamboyant personality known for her pursuit of Picard and deep love for her daughter",
    "Wesley" => "Wesley Crusher, young prodigy who went from acting ensign to Traveler, guided by his exceptional understanding of space, time, and propulsion",
    "Barclay" => "Lieutenant Reginald Barclay, brilliant but socially anxious engineer who overcame his holodeck addiction to become a valuable crew member",
    "Ro" => "Ensign Ro Laren, Bajoran officer whose strong principles and troubled past brought both conflict and growth to the Enterprise crew",

    // Deep Space Nine (DS9)
    "Sisko" => "Commander (later Captain) Benjamin Sisko, Emissary of the Prophets and commanding officer of Deep Space Nine, known for his strategic mind and role in the Dominion War",
    "Dax" => "Jadzia Dax, joined Trill science officer combining the wisdom of multiple lifetimes with a vibrant personality",
    "Bashir" => "Dr. Julian Bashir, brilliant genetically enhanced medical officer who balanced his superior abilities with genuine compassion",
    "Kira" => "Major Kira Nerys, former Bajoran resistance fighter serving as first officer, fierce defender of Bajoran independence",
    "Odo" => "The station's shapeshifting Chief of Security, struggling with his identity as a Changeling while maintaining order",
    "OBrien" => "Chief Miles O'Brien, transferred from the Enterprise-D, keeping the station running through technical expertise and determination",
    "Quark" => "Ferengi bar owner and occasional information broker, balancing profit with an unlikely conscience",
    "Rom" => "Quark's brother who evolved from a failed Ferengi businessman to a skilled engineer and eventually Grand Nagus of the Ferengi Alliance",
    "Nog" => "Rom's son who became the first Ferengi in Starfleet, showing remarkable growth from juvenile delinquent to respected officer",
    "Garak" => "Elim Garak, exiled Cardassian tailor and former spy, whose complex character balanced genuine friendship with mysterious motives",
    "Martok" => "General Martok, Klingon warrior who rose from humble origins to become Chancellor of the Klingon Empire, known for his honor and leadership",
    "Weyoun" => "Vorta clone serving as the Dominion's diplomatic face, characterized by his unwavering devotion to the Founders",
    "Ezri" => "Ezri Dax, young joined Trill counselor who unexpectedly received the Dax symbiont and had to navigate its multiple lifetimes",
    "Leeta" => "Bajoran Dabo girl who married Rom and became an influential voice for workers' rights and social justice",
    "Dukat" => "Former Cardassian prefect of Bajor whose complex character arc saw him descend from military leader to religious zealot",
    "Winn" => "Kai Winn Adami, ambitious Bajoran religious leader whose desire for power led her down a dark path",
    "Damar" => "Cardassian leader who evolved from Dukat's loyal second to resistance hero fighting for Cardassia's freedom",

    // Voyager
    "Janeway" => "Captain Kathryn Janeway, determined leader of the USS Voyager, combining scientific curiosity with unwavering dedication to getting her crew home",
    "Chakotay" => "First Officer and former Maquis leader, bringing his spiritual wisdom and tactical experience to support Janeway",
    "Tuvok" => "Vulcan Chief of Security and Janeway's trusted friend, maintaining logic and discipline while serving as a bridge between Starfleet and Maquis crews",
    "Paris" => "Lieutenant Tom Paris, reformed criminal and ace pilot, whose redemption arc led him from prison to becoming Voyager's trusted helmsman",
    "Torres" => "B'Elanna Torres, half-Klingon Chief Engineer who struggled with her dual heritage while keeping Voyager running against impossible odds",
    "Kim" => "Ensign Harry Kim, young operations officer who maintained his optimism despite being stranded far from home",
    "Seven" => "Seven of Nine, former Borg drone reclaiming her humanity while bringing unique perspectives and technological expertise",
    "Doctor" => "The Emergency Medical Hologram who evolved beyond his programming to become a valued crew member and friend, developing his own personality and ethics",
    "Neelix" => "Talaxian morale officer and chef, whose knowledge of the Delta Quadrant and optimistic personality helped the crew adapt to their situation",
    "Kes" => "Ocampa with powerful psionic abilities who served as medical assistant, characterized by her short lifespan and rapid personal growth",
    "Icheb" => "Former Borg drone rescued by Voyager who became a valued crew member and protégé to Seven of Nine",
    "Seska" => "Former Cardassian spy infiltrating the Maquis who became one of Voyager's most dangerous adversaries",
    "Wildman" => "Ensign Samantha Wildman, science officer and mother to Naomi, the first child born on Voyager",
    "Naomi" => "Naomi Wildman, half-Ktarian child born on Voyager who appointed herself the captain's assistant",

    // Enterprise
    "Archer" => "Captain Jonathan Archer, pioneer of deep space exploration and first captain of the Enterprise NX-01, instrumental in forming the Federation",
    "TPol" => "Vulcan science officer who served as Archer's second-in-command, gradually embracing human emotions and customs",
    "Tucker" => "Commander Charles 'Trip' Tucker III, chief engineer and Archer's best friend, known for his innovative solutions and Southern charm",
    "Reed" => "Lieutenant Malcolm Reed, tactical officer obsessed with security protocols and proper military discipline",
    "Sato" => "Ensign Hoshi Sato, linguistics expert who developed the first Universal Translator, overcoming her fear of space travel",
    "Mayweather" => "Ensign Travis Mayweather, skilled pilot born and raised in space on cargo ships, bringing unique 'boomer' perspective",
    "Phlox" => "Dr. Phlox, Denobulan physician combining multiple medical traditions with curiosity about human culture",
    "Shran" => "Commander Thy'lek Shran, Andorian Imperial Guard officer who evolved from antagonist to crucial ally in the formation of the Federation",
    "Daniels" => "Temporal agent from the 31st century who recruited Archer in the Temporal Cold War, providing guidance while maintaining temporal integrity",
    "Silik" => "Suliban Cabal leader and primary antagonist in the Temporal Cold War, taking orders from a mysterious benefactor",
    "Hayes" => "Major Hayes, leader of the MACO unit assigned to Enterprise during the Xindi crisis",

    // Discovery
    "Burnham" => "Michael Burnham, raised on Vulcan by Sarek, whose actions sparked a war but later became instrumental in saving the Federation across centuries",
    "Saru" => "Kelpien officer who overcame his species' inherent fear to become one of Starfleet's most distinguished captains",
    "Tilly" => "Sylvia Tilly, enthusiastic officer who grew from insecure cadet to confident leader while maintaining her authentic personality",
    "Stamets" => "Lt. Commander Paul Stamets, astromycologist and pioneer of the spore drive technology, first human to successfully navigate the mycelial network",
    "Culber" => "Dr. Hugh Culber, compassionate medical officer who returned from death in the mycelial network",
    "Georgiou" => "Captain Philippa Georgiou and her Mirror Universe counterpart, representing both Starfleet's highest ideals and ruthless pragmatism",
    "Book" => "Cleveland Booker, empath with ability to communicate with animals, who helped Discovery adapt to the 32nd century",
    "Adira" => "First human host of a Trill symbiont, bringing unique perspective and centuries of knowledge to Discovery's crew",
    "Gray" => "Trill whose consciousness remained visible to Adira after joining, representing a unique manifestation of the symbiotic relationship",
    "TKuvma" => "Charismatic Klingon leader who sought to unite the Klingon houses against the Federation, believing in maintaining Klingon purity and traditions",
    "Lorca" => "Captain Gabriel Lorca, whose true identity as a Mirror Universe imposter shaped Discovery's early missions",
    "Reno" => "Commander Jett Reno, sardonic engineer whose unconventional solutions and dry wit proved invaluable to the crew",

    // Picard
    "Raffi" => "Raffaela Musiker, former Starfleet officer and intelligence analyst who helped Picard uncover the truth about the synthetic attack on Mars",
    "Rios" => "Cristóbal Rios, skilled pilot and former Starfleet officer commanding the La Sirena, dealing with his past through multiple emergency holograms",
    "Jurati" => "Dr. Agnes Jurati, cybernetics expert whose involvement with the Zhat Vash complicated her relationship with synthetic life",
    "Elnor" => "Young Romulan warrior trained in the way of Absolute Candor, serving as Picard's loyal protector",
    "Soji" => "Synthetic daughter of Data, whose discovery of her true nature sparked a crisis for both organic and artificial life",
    "Laris" => "Former Tal Shiar operative who became Picard's trusted friend and housekeeper at Chateau Picard",
    "Shaw" => "Captain Liam Shaw, commanding officer of the USS Titan-A, whose by-the-book approach often clashed with unorthodox solutions",

    // Strange New Worlds
    "Una" => "Commander Una Chin-Riley, Pike's first officer known as Number One, whose genetic modifications made her uniquely qualified for command",
    "Laan" => "La'an Noonien-Singh, security chief carrying the burden of her ancestor Khan's legacy while proving her own worth",
    "Ortegas" => "Lieutenant Erica Ortegas, Enterprise's skilled helmsman known for her quick wit and steady hand in crisis situations",
    "Hemmer" => "Chief Engineer Hemmer, an Aenar whose blindness was compensated by his species' telepathic abilities",
    "Singh" => "Sam Kirk, James T. Kirk's brother and Enterprise science officer, bringing warmth and scientific curiosity to the crew",

    // Lower Decks
    "Mariner" => "Ensign Beckett Mariner, rebellious but highly capable officer whose experience and skills often exceed her rank",
    "Boimler" => "Ensign Brad Boimler, by-the-book officer whose dedication to Starfleet regulations both helps and hinders his career",
    "Tendi" => "Ensign D'Vana Tendi, enthusiastic Orion medical officer whose optimism and scientific curiosity drive her adventures",
    "Rutherford" => "Ensign Sam Rutherford, engineering officer whose cybernetic implant occasionally complicates his daily life and relationships",
    "Freeman" => "Captain Carol Freeman, commanding officer of the USS Cerritos and Mariner's mother, striving to balance family and command",
    "Shaxs" => "Lieutenant Shaxs, tactical officer whose aggressive approach to security often leads to explosive solutions",
    "Billups" => "Commander Andy Billups, chief engineer whose royal heritage conflicts with his Starfleet career",

    // Prodigy
    "Dal" => "Dal R'El, mysterious teenager who becomes captain of the USS Protostar, discovering his own identity while leading a diverse crew",
    "Gwyn" => "Gwyndala, daughter of the Diviner who questions her upbringing and chooses her own path with the Protostar crew",
    "Zero" => "Zero, a Medusan whose containment suit protects others from their overwhelming true form",
    "Rok" => "Rok-Tahk, young Brikar whose physical strength belies her gentle nature and interest in science",
    "Jankom" => "Jankom Pog, young Tellarite engineer whose argumentative nature masks his loyalty to the crew",
    "Murf" => "Murf, an endearing blob-like creature with seemingly indestructible properties",
    "Hologram-Janeway" => "Training hologram of Admiral Janeway, guiding the young crew in Starfleet principles and operations",

    // Notable Villains and Antagonists
    "Khan" => "Khan Noonien Singh, genetically enhanced superhuman from Earth's past, one of Kirk's most formidable adversaries",
    "Queen" => "Central nexus of the Borg Collective, embodying their drive for perfection through assimilation",
    "Locutus" => "Picard's identity when assimilated by the Borg, used to facilitate the invasion of Federation space",
    "Lore" => "Data's emotionally unstable android brother who allied with rogue Borg and threatened the Federation",
    "Shinzon" => "Picard's clone who became Praetor of the Romulan Empire, seeking both revenge and understanding of his own identity",
    "Control" => "AI threat that evolved from Section 31's threat assessment system into an existential danger to all sentient life",
    //"Female-Changeling" => "Leader of the Dominion who orchestrated the war against the Alpha Quadrant",
    "Krall" => "Former Starfleet captain transformed into a bitter enemy seeking revenge against the Federation",
    "Nero" => "Romulan mining captain whose grief-driven vengeance created an alternate timeline",
    "Chang" => "Klingon general who conspired to prevent peace between the Empire and Federation",
    "Gorkon" => "Progressive Klingon Chancellor whose assassination nearly prevented peace with the Federation"
};

/// Returns a vector of all character names
#[must_use]
pub fn get_all_names() -> Vec<&'static str> {
    TREK_DESCRIPTIONS.keys().copied().collect()
}

/// Returns the description for a given character name, if it exists
#[must_use]
pub fn get_description(name: &str) -> Option<&'static str> {
    TREK_DESCRIPTIONS.get(name).copied()
}

/// Returns true if the character exists in the database
#[must_use]
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
