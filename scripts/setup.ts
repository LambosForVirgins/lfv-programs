#!/usr/bin/env ts-node

import { Command } from "commander";
import * as fs from "fs/promises";
import { exec } from "child_process";
import { promisify } from "util";

const execPromise = promisify(exec);

// Define the interface for Solana configuration objects.
interface SolanaConfig {
  name: string;
  url: string;
  wallet?: string;
}

// Function to load and parse the config JSON file.
async function loadConfigs(filePath: string): Promise<SolanaConfig[]> {
  try {
    const data = await fs.readFile(filePath, "utf8");
    const configs = JSON.parse(data);
    if (!Array.isArray(configs)) {
      throw new Error(
        "The config file must contain an array of config objects."
      );
    }
    return configs;
  } catch (error) {
    console.error("Error reading or parsing config file:", error);
    process.exit(1);
  }
}

// Command to list available configurations.
async function listConfigs(filePath: string): Promise<void> {
  const configs = await loadConfigs(filePath);
  console.log("Available configurations:");
  configs.forEach((config: SolanaConfig) => {
    console.log(`- ${config.name}`);
  });
}

// Command to apply a configuration by name.
async function applyConfig(
  filePath: string,
  configName: string
): Promise<void> {
  const configs = await loadConfigs(filePath);
  const config = configs.find((cfg) => cfg.name === configName);
  if (!config) {
    console.error(`No configuration found with name "${configName}".`);
    process.exit(1);
  }
  if (!config.url) {
    console.error('The selected configuration must include a "url" property.');
    process.exit(1);
  }

  // Build the Solana CLI command.
  let cmd = `solana config set --url ${config.url}`;
  if (config.wallet) {
    cmd += ` --keypair ${config.wallet}`;
  }
  console.log(`Executing command: ${cmd}`);

  try {
    const { stdout, stderr } = await execPromise(cmd);
    if (stdout) console.log(stdout);
    if (stderr) console.error(stderr);
  } catch (error: any) {
    console.error(`Error executing command: ${error.message}`);
    process.exit(1);
  }
}

// Set up the commander program.
const program = new Command();

program
  .version("1.0.0")
  .description("CLI tool to change Solana configuration from a JSON file")
  .option("-c, --config <path>", "Path to config file", "config.json");

program
  .command("list")
  .description("List available Solana configurations")
  .action(async () => {
    const options = program.opts();
    await listConfigs(options.config);
  });

program
  .command("apply <name>")
  .description("Apply a Solana configuration by name")
  .action(async (name: string) => {
    const options = program.opts();
    await applyConfig(options.config, name);
  });

program.parse(process.argv);
