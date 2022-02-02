import randomUseragent from 'random-useragent';

import puppeteer from 'puppeteer-extra';
import StealthPlugin from 'puppeteer-extra-plugin-stealth';
import {getBrowserViewport, getRandomReferer} from "./helper/browser.js";
import {validatePageContent} from "./helper/page-validator.js";
puppeteer.use(StealthPlugin());


const EMPTY_CONTENT = "";
const MAX_CONNECTION_ATTEMPTS = 5;
const PAGE_WAIT_TIMEOUT = 6000;//ms
export async function extract(url) {
    logger.info("[extractor] start initializing virtual browser")

    let browser = null;
    let page = null;
    try {
        browser = await puppeteer.launch({
            headless: true,
            ignoreHTTPSErrors: true,
            executablePath: process.env.CHROMIUM_PATH,
            args: [
                '--no-sandbox',
                '--ignore-certificate-errors',
                '--ignore-certificate-errors-spki-list',
                '--lang=en-US,en',
                '--disable-extensions',
                //`--proxy-server=${process.env.PROXY_URL}`,
            ],
        });

        page = await browser.newPage();

        const viewport = getBrowserViewport();
        await page.setViewport(viewport);
        logger.info(`  |> viewport (height: ${viewport.height}px | width: ${viewport.width}px)`);

        await page.setJavaScriptEnabled(true);
        logger.info("  |> javascript: enabled")
        await page.setDefaultNavigationTimeout(0);
        logger.info("  |> browser infinity timeout: enabled")

        await page.setExtraHTTPHeaders({
            "Accept-Language": "en,en-US;q=0,5",
            Accept: "text/html,application/xhtml+xml,application/xml;q=0.9,/;q=0.8",
        });

    } catch (err) {
        logger.error("request error: ", err);
        return EMPTY_CONTENT;
    }

    if (!browser || !page) {
        logger.error("request error: empty page or browser", page, browser);
    }

    let attempt = 0;
    while (attempt <= MAX_CONNECTION_ATTEMPTS) {
        try {
            logger.info(`  |> trying to send a request to ${url}`)
            const userAgent = randomUseragent.getRandom();
            logger.info(`  |> with user agent: ${userAgent}`);
            await page.setUserAgent(userAgent);

            const referer = await getRandomReferer();
            logger.info(`  |> with referer: ${referer}`);
            await page.setExtraHTTPHeaders({referer: referer});

            await page.goto(url);
            let isNotFirstAttempt = attempt !== 0;
            if (isNotFirstAttempt) {
                logger.info(`  |> waiting ${PAGE_WAIT_TIMEOUT} (ms)`);
                await page.waitForTimeout(PAGE_WAIT_TIMEOUT)
                logger.info(`  |> waiting finished`);
            }

            logger.info(`  |> goto page`);
            const response = await page.goto(url);
            if (!response.ok()) {
                logger.error(`request error ${url}: invalid status ${response.status()}`);
                throw new Error(`invalid status ${response.status()}`);
            }
            logger.info(`  |> page have been reached`);

            logger.info('  |> validate page content...')
            const content = await page.content();
            validatePageContent(content);

            logger.info('  |> access has been successfully granted!')
            await page.close();
            logger.info('  |> page has been closed');

            console.log(content);
            return content;

        } catch (err) {
            logger.error(`attempt #${attempt} error: ${err.message}`);
            attempt+=1;
        }
    }

    if (!page.isClosed()) {
        await page.close();
        logger.info('  |> page has been closed');
    }

    logger.error(`request error: exceeded the limit of requests to the website: ${url}`);
    return EMPTY_CONTENT;
}
