import randomUseragent from 'random-useragent';

import puppeteer from 'puppeteer-extra';
import StealthPlugin from 'puppeteer-extra-plugin-stealth';
import {getBrowserViewport, getRandomReferer} from "./helper/browser.js";
import {validatePageContent} from "./helper/page-validator.js";
puppeteer.use(StealthPlugin());


const EMPTY_CONTENT = "";
const MAX_CONNECTION_ATTEMPTS = 5;
const PAGE_WAIT_TIMEOUT = 5000;//ms
export async function extract(url) {
    // console.log("[extractor] start initializing virtual browser")
    let browser = null;
    let page = null;
    try {
        browser = await puppeteer.launch({
            headless: true,
            // executablePath: process.env.CHROMIUM_PATH,
             executablePath: '/usr/bin/chromium-browser',
            args: ['--no-sandbox'],

        });
        page = await browser.newPage();

        const viewport = getBrowserViewport();
        // console.log(`  |> viewport (height: ${viewport.height}px | width: ${viewport.width}px)`);
        await page.setViewport(viewport);

        // console.log("  |> javascript: enabled")
        await page.setJavaScriptEnabled(true);
        // console.log("  |> browser infinity timeout: enabled")
        await page.setDefaultNavigationTimeout(0);

    } catch (err) {
        console.log("request error: ", err);
        return EMPTY_CONTENT;
    }

    if (!browser || !page) {
        console.log("request error: empty page or browser", page, browser);
    }

    let attempt = 0;
    while (attempt <= MAX_CONNECTION_ATTEMPTS) {
        try {
            // console.log(`  |> trying to send a request to ${url}`)
            const userAgent = randomUseragent.getRandom();
            // console.log("  |> with user agent:", userAgent);
            await page.setUserAgent(userAgent);

            const referer = await getRandomReferer();
            // console.log("  |> with referer:", referer);
            await page.setExtraHTTPHeaders({referer: referer});

            await page.goto(url);
            let isNotFirstAttempt = attempt !== 0;
            if (isNotFirstAttempt) {
                // console.log(`  |> waiting ${PAGE_WAIT_TIMEOUT} (ms)`);
                await page.waitForTimeout(PAGE_WAIT_TIMEOUT)
            }

            const response = await page.goto(url);
            if (!response.ok()) {
                throw new Error(`invalid status ${response.status()}`);
            }

            // console.log('  |> validate page content...')
            const content = await page.content();
            validatePageContent(content);

            // console.log('  |> access has been successfully granted!')
            await browser.close();
            // console.log('  |> browser has been closed');

            console.log(content);
            return content;

        } catch (err) {
            console.log('attempt error:', err.message);
            attempt+=1;
        }
    }

    if (browser.isConnected()) {
        await browser.close();
        // console.log('  |> browser has been closed');
    }

    console.log(`request error: exceeded the limit of requests to the website: ${url}`);
    return EMPTY_CONTENT;
}
