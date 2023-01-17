import {UpgradableProgram} from "./upgradables.ts"
const repos = {
    gameReleasesUrl: "https://api.github.com/repos/symphony-of-empires/symphony-of-empires/releases",
    launcherReleasesUrl: "https://api.github.com/repos/symphony-of-empires/SOEutil/releases",
    downloaderReleasesUrl: "https://api.github.com/repos/symphony-of-empires/SOEutil/releases",
}

export async function getProjectReleases(programToUpdate:UpgradableProgram):Promise<Record<string, unknown> | null> {
    const headers = {headers: {'User-Agent':"'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_8_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/29.0.1521.3 Safari/537.36'"}}
    switch(programToUpdate) {
        case UpgradableProgram.Game:
            return await (await fetch(repos.gameReleasesUrl, headers)).json()
        case UpgradableProgram.Launcher:
            return await (await fetch(repos.launcherReleasesUrl, headers)).json()    
        case UpgradableProgram.Downloader:
            return await (await fetch(repos.downloaderReleasesUrl, headers)).json()
        default:
            return null
    }
}