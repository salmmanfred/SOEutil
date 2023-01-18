import { getProjectReleases } from "./releases_fetcher.ts"
import { ensureDirSync } from "https://deno.land/std@0.173.0/fs/mod.ts"

export function downloadReleases(url: URL) {
    getProjectReleases(url).then((data) => {

        const assets = data[0].assets
        assets.forEach(async (asset) => {

            if (asset.browser_download_url.endsWith(`${Deno.build.os}.zip`)) {
                ensureDirSync("./cache/update")

                const fileResponse = await fetch(asset.browser_download_url)
                const file = Deno.openSync("./cache/update/game.zip", { write: true, create: true })
                fileResponse.body?.pipeTo(file.writable)
            }

        })

    })
}
