#!/usr/bin/env node
import { Command } from "commander";
const cli = new Command();
import {extract} from "./html-extractor.js";

import { fileURLToPath } from 'url';
import { dirname } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
global.__basedir = __dirname;

cli
    .command("html-extractor")
    .argument("<url>", "URL of the site content you'd like to retrieve.")
    .description(
        "Retrieve html content at the specified url."
    )
    .action(extract);

cli.parse(process.argv);
