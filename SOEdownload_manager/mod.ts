import { parse } from "https://deno.land/std@0.173.0/flags/mod.ts"

const flags = parse(Deno.args,
    {
        boolean:["update_game", "update_launcher", "restart_game", "restart_launcher"],
        string:["download_mod"]
    }
)

console.log(flags)