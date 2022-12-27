import fs from "fs";
// since declarations are also used elsewhere, make node compatible here
// otherwise actor can't be instantiated due to agent config error
const commentOut = 'export const invoice = createActor(canisterId);';
try {
    fs.writeFileSync(process.argv[2], 
        fs.readFileSync(process.argv[2], 'utf8')
            .split('\n')
            .map(line => line.trim() === commentOut ? '// ' + line : line)
            .join('\n'));
} catch (e) {
    console.error("Couldn't comment out actor export that isn't compatible with node, check manually")
}
