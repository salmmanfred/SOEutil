import { parse } from "https://deno.land/std@0.173.0/flags/mod.ts"
import { downloadLatestRelease } from "./updater/releases_downloader.ts"
import { decompress } from "https://deno.land/x/zip@v1.2.3/mod.ts"

const flags = parse(Deno.args, {
    boolean: [
        "update_game",
        "update_launcher",
        "restart_game",
        "restart_launcher",
        "self_update",
        "test",
    ],
    string: [
        "download_mod",
        "game_releases_url",
        "launcher_releases_url",
        "downloader_releases_url",
        "download_dir",
        "test_game_releases_url",
        "test_launcher_releases_url",
    ],
    default: {
        game_releases_url:
            "https://api.github.com/repos/symphony-of-empires/symphony-of-empires/releases",
        launcher_releases_url: "https://api.github.com/repos/symphony-of-empires/SOEutil/releases",
        downloader_releases_url:
            "https://api.github.com/repos/symphony-of-empires/SOEutil/releases",
        download_dir: ".cache/updater",
        test_game_releases_url: "https://api.github.com/repos/yrenum/symphony-of-empires/releases",
        test_launcher_releases_url: "https://api.github.com/repos/yrenum/SOEutil/releases",
    },
})

if (flags.update_game) {
    const url = !flags.test
        ? new URL(flags.game_releases_url)
        : new URL(flags.test_game_releases_url)

    const archivePath = await downloadLatestRelease(url, flags.download_dir, "game.zip")
    if (archivePath != undefined) decompress(archivePath, "./", { overwrite: true })
}

if (flags.update_launcher) {
    const url = !flags.test
        ? new URL(flags.launcher_releases_url)
        : new URL(flags.test_launcher_releases_url)

    const archivePath = await downloadLatestRelease(url, flags.download_dir, "launcher.zip")
    if (archivePath != undefined) decompress(archivePath, "./", { overwrite: true })
}
