import {UpgradableProgram} from "./upgradables.ts"

export async function getProjectReleases(flags:any, programToUpdate:UpgradableProgram):Promise<Record<string, unknown> | null> {
    const headers = {headers: {'User-Agent':"'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_8_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/29.0.1521.3 Safari/537.36'"}}
    switch(programToUpdate) {
        case UpgradableProgram.Game:
            return await (await fetch(flags.game_releases_url, headers)).json()
        case UpgradableProgram.Launcher:
            return await (await fetch(flags.launcher_releases_url, headers)).json()    
        case UpgradableProgram.Downloader:
            return await (await fetch(flags.downloader_releases_url, headers)).json()
        default:
            return null
    }
}