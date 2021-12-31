const IS_CLOUDFLARE_PAGE_RE = /Checking your browser before accessing/gm;
const isCloudflarePage = (content) => IS_CLOUDFLARE_PAGE_RE.test(content);

export const validatePageContent = (content) => {
    if (isCloudflarePage(content)) {
        throw new Error("found the Cloudflare page!");
    }
    /** any other blocker/captcha */
    return true;
}