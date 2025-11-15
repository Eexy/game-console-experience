export type SteamOwnedGame = {
    name: string;
    appid: number;
    img_icon_url: string | null;
    img_logo_url: string | null;
    playtime_2weeks: number;
    playtime_forever: number;
};

export type SteamGameGenre = {
    id: string;
    description: string;
};

export type SteamGameInfo = {
    about_the_game: string;
    capsule_image: string;
    genres: SteamGameGenre[];
    header_image: string;
    is_free: boolean;
    name: string;
    required_age: number;
    steam_appid: number;
};
