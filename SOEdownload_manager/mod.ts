import { parse } from "https://deno.land/std@0.173.0/flags/mod.ts"
import { getProjectReleases } from "./github/fetch_releases.ts"
import { UpgradableProgram } from "./github/upgradables.ts"

const flags = parse(Deno.args, {
    boolean: ["update_game", "update_launcher", "restart_game", "restart_launcher", "self_update"],
    string: ["download_mod"],
})
