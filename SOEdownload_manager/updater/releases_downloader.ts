import { getProjectReleases } from "./releases_fetcher.ts"
import { ensureDirSync } from "https://deno.land/std@0.173.0/fs/mod.ts"
import { join } from "https://deno.land/std@0.172.0/path/mod.ts"

export function downloadReleases(url: URL, downloadDir: string, fileName: string) {
    getProjectReleases(url).then((data) => {
        const assets = data[0].assets
        assets.forEach(async (asset) => {
            if (asset.browser_download_url.endsWith(`${Deno.build.os}.zip`)) {
                ensureDirSync(downloadDir)

                const fileResponse = await fetch(asset.browser_download_url)
                const file = Deno.openSync(`${join(downloadDir, fileName)}.zip`, {
                    write: true,
                    create: true,
                })
                await fileResponse.body?.pipeTo(file.writable)
            }
        })
    })
}
