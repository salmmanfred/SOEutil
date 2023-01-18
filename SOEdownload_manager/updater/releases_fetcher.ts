import { ReleaseData } from "./release_data.ts"

export async function getProjectReleases(repo: URL): Promise<ReleaseData[]> {
    const init = {
        headers: {
            "User-Agent":
                "'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_8_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/29.0.1521.3 Safari/537.36'",
        },
    }
    return (await fetch(repo, init)).json()
}
