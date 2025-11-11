import { SteamOwnedGame } from "./games";

export type SteamOwnedGameResponse = {
    game_count: number;
    games: SteamOwnedGame[];
};
