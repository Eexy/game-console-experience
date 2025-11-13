import { SteamGameInfo, SteamOwnedGame } from "./games";

export type SteamOwnedGameResponse = {
    game_count: number;
    games: SteamOwnedGame[];
};

export type SteamGameInfoResponse = Record<
    number,
    | {
          data: SteamGameInfo;
          success: true;
      }
    | {
          data: null;
          success: false;
      }
>;
