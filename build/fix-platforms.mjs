import { readFileSync, writeFileSync } from "node:fs";

const pkg = JSON.parse(readFileSync("package.json"));

if (pkg.optionalDependencies)
{
    pkg.optionalDependencies = Object.fromEntries(Array.prototype.map.call(
        Object.entries(pkg.optionalDependencies),
        ([key, value]) => {
            let platform = key.match(/@msensis-com\/(.+)/)?.[1];
            if (!platform) return null;
            if (platform === "win32-x64-msvc") {
                platform = "windows-x64-gnu";
            }
            return [`@msensis/hickory-proto-neon-${platform}`, value];
        }
    ).filter(Boolean));
}

writeFileSync("package.json", JSON.stringify(pkg, null, 2));
