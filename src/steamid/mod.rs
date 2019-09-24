mod SteamID {
    pub enum IDType {
        SteamID2,
        SteamID3,
        SteamID64,
        SteamIDUnknown,
    }

    pub struct Info {
        universe: u8,
        instance: u32,
        accountID: u32,
    }

    fn determine_type(id: String) -> IDType {
        if id.parse::<u64>().is_ok() {
            return IDType::SteamID64;
        }

        if id.contains("STEAM") {
            return IDType::SteamID2;
        }

        if id.contains("[") {
            return IDType::SteamID3;
        }

        return IDType::SteamIDUnknown;
    }

    // pub fn parse_id(id: String) -> Info {
    //     let type = determine_type(id);
    // }
}