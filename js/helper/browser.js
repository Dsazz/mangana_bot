import {promises as fs} from "fs";
import path from "path";

export function getBrowserViewport() {
    return {
        width: 1920 + Math.floor(Math.random() * 100),
        height: 3000 + Math.floor(Math.random() * 100),
        deviceScaleFactor: 1,
        hasTouch: false,
        isLandscape: false,
        isMobile: false,
    };
}

export async function getRandomReferer() {
    const file = await fs.readFile(path.join(__basedir, '/helper/referrers.json'), 'utf8');
    const referrers = JSON.parse(file);
    return referrers[referrers.length * Math.random() | 0];
}