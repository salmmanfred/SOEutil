import { parse } from "https://deno.land/std@0.173.0/flags/mod.ts"
import { downloadReleases } from "./updater/releases_downloader.ts"

const flags = parse(Deno.args, {
    boolean: ["update_game", "update_launcher", "restart_game", "restart_launcher", "self_update"],
    string: ["download_mod", "game_releases_url", "launcher_releases_url", "downloader_releases_url"],
    default:{
        game_releases_url: "https://api.github.com/repos/symphony-of-empires/symphony-of-empires/releases",
        launcher_releases_url: "https://api.github.com/repos/symphony-of-empires/SOEutil/releases",
        downloader_releases_url: "https://api.github.com/repos/symphony-of-empires/SOEutil/releases",
    }
})

if(flags.update_game) {
    const url = new URL(flags.game_releases_url)
    downloadReleases(url)
}