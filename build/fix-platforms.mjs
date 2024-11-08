import { readFileSync, writeFileSync } from "node:fs";

const pkg = JSON.parse(readFileSync("package.json"));

if (pkg.optionalDependencies)
{
    pkg.optionalDependencies = Object.fromEntries(Array.prototype.map.call(
        Object.entries(pkg.optionalDependencies),
        ([key, value]) => {
            const platform = key.match(/@msensis-com\/(.+)/)?.[1];
            return [`@msensis/hickory-proto-neon-${platform}`, value];
        }
    ));
}

writeFileSync("package.json", JSON.stringify(pkg, null, 2));
