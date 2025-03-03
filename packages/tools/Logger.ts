import color from "colorts";

export class Logger {
  static info(message: string, description?: string): void {
    console.log(
      color("INFO").cyan.toString(),
      color(message).white.toString(),
      description && color(description).grey.toString()
    );
  }

  static success(message: string, description?: string): void {
    console.log(
      color("OKAY").green.toString(),
      color(message).white.toString(),
      description && color(description).grey.toString()
    );
  }

  static warn(message: string, description?: string): void {
    console.log(
      color("WARN").yellow.toString(),
      color(message).white.toString(),
      description && color(description).grey.toString()
    );
  }
}
