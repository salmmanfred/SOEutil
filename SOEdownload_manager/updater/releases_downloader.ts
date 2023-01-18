import { getProjectReleases } from "./releases_fetcher.ts"
import { ensureDirSync } from "https://deno.land/std@0.173.0/fs/mod.ts"
import { join } from "https://deno.land/std@0.173.0/path/mod.ts"
import { maxBy } from "https://deno.land/std@0.172.0/collections/max_by.ts"

export async function downloadLatestRelease(url: URL, downloadDir: string, fileName: string): Promise<string | undefined> {
    const data = await getProjectReleases(url)
    if (data.length == 0) return

    // get the latest release
    const assets = maxBy(data, (release) => release.published_at)?.assets
    if(assets == undefined)
        return

    const filePath = join(downloadDir, fileName)

    for(const asset of assets) {
        if (asset.browser_download_url.endsWith(`${Deno.build.os}.zip`)) {
            ensureDirSync(downloadDir)

            const fileResponse = await fetch(asset.browser_download_url)
            const file = Deno.openSync(filePath, {
                write: true,
                create: true,
            })
            await fileResponse.body?.pipeTo(file.writable)

            break
        }
    }
    return filePath
}
