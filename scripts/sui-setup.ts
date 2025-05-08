import { execSync, spawn } from "child_process";
import * as os from "os";
import * as fs from "fs";
import * as path from "path";
import { Logger } from "../packages/tools/Logger";

const CONFIG_DIR = path.join(os.homedir(), ".sui", "sui_config");
const CONFIG_PATH = path.join(CONFIG_DIR, "client.yaml");

const execCommand = (command: string, silent = false): string => {
  try {
    return execSync(command, { stdio: silent ? "pipe" : "inherit" })
      .toString()
      .trim();
  } catch (error) {
    if (!silent) console.error(`Error executing: ${command}`);
    throw error;
  }
};

const checkSuiInstalled = (): boolean => {
  try {
    execCommand("sui --version", true);
    return true;
  } catch {
    return false;
  }
};

const installSui = (): void => {
  const platform = os.platform();
  if (platform === "darwin") {
    Logger.info("Installing Sui CLI via Homebrew...");
    execCommand("brew install sui");
  } else if (platform === "win32") {
    Logger.info("Installing Sui CLI via Chocolatey...");
    execCommand("choco install sui");
  } else {
    throw new Error("Unsupported platform for automatic installation.");
  }
};

const startLocalNetwork = (): void => {
  Logger.info("Starting local Sui network with faucet...");
  const ps = spawn("sui", ["start", "--with-faucet", "--force-regenesis"], {
    stdio: "inherit",
    detached: true,
  });

  ps.stderr?.on("data", (data) => {
    const errorMessage = data.toString();
    if (errorMessage.includes("Error")) {
      console.error(`Error starting Sui network: ${errorMessage}`);
      ps.kill();
    }
  });

  ps.on("error", (error) => {
    console.error(`Failed to start Sui network: ${error.message}`);
  });
};

const checkOrCreateAlias = (): void => {
  const aliases = execCommand("sui client envs", true);
  if (!aliases.includes("local")) {
    Logger.info("Creating 'local' environment alias...");
    execCommand("sui client new-env --alias local --rpc http://127.0.0.1:9000");
  }
  Logger.info("Switching to 'local' environment...");
  execCommand("sui client switch --env local");
};

const getPreferredAddressFromConfig = (): string | null => {
  if (!fs.existsSync(CONFIG_PATH)) return null;
  const yaml = fs.readFileSync(CONFIG_PATH, "utf8");
  const match = yaml.match(/active_address:\s*(.+)/);
  return match ? match[1].trim() : null;
};

const getCurrentActiveAddress = (): string => {
  return execCommand("sui client active-address", true);
};

const setActiveAddress = (address: string): void => {
  Logger.info(`Setting active address to ${address}...`);
  execCommand(`sui client switch --address ${address}`);
};

const getActiveAddressBalance = (address: string): number => {
  const json = execCommand(`sui client gas --address ${address} --json`, true);
  const output = JSON.parse(json);
  const totalBalance = output.reduce(
    (sum: number, item: any) => sum + Number(item.balance),
    0
  );
  return totalBalance / 1e9; // Convert from MIST
};

const requestFaucet = (address: string): void => {
  Logger.info("Requesting tokens from faucet...");
  execCommand(`sui client faucet --address ${address}`);
};

const main = async (): Promise<void> => {
  if (!checkSuiInstalled()) installSui();

  startLocalNetwork();

  await new Promise((resolve) => setTimeout(resolve, 5000));

  checkOrCreateAlias();

  const preferred = getPreferredAddressFromConfig();
  const current = getCurrentActiveAddress();

  if (preferred && preferred !== current) {
    setActiveAddress(preferred);
  }

  const active = getCurrentActiveAddress();
  Logger.success(`✅ Active address: ${active}`);

  const balance = getActiveAddressBalance(active);
  Logger.success(`💰 Balance: ${balance} SUI`);

  if (balance < 1) {
    requestFaucet(active);
    Logger.success("Faucet called. Waiting before rechecking...");
    await new Promise((resolve) => setTimeout(resolve, 5000));
    const newBalance = getActiveAddressBalance(active);
    Logger.success(`💸 New Balance: ${newBalance} SUI`);
  } else {
    Logger.success("✔️ Sufficient balance available.");
  }
};

main().catch((e) => {
  console.error("❌ Error:", e.message);
  process.exit(1);
});
