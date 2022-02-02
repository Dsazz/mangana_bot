#!/usr/bin/env node
import { Command } from "commander";
const cli = new Command();
import {extract} from "./html-extractor.js";
import * as winston from 'winston';

import { fileURLToPath } from 'url';
import { dirname } from 'path';

const logMaxSize = 10485760; //10mb
const logger = winston.createLogger({
    transports: [
        new winston.transports.File({
            level: 'error',
            filename: './logs/error.log',
            json:false,
            maxFiles: 10,
            maxsize: logMaxSize,
            timestamp: true,
        }),
        new winston.transports.File({
            filename: './logs/all.log',
            json:false,
            maxFiles: 10,
            maxsize: logMaxSize,
            timestamp: true,
        }),
    ]
});
global.logger = logger;

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
