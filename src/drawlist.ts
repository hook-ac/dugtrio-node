import { Vector2 } from "./math";

export interface Payload {
  commands: Command[];
}

export type Command =
  | CircleCommand
  | RectCommand
  | ThicknessCommand
  | RoundingCommand
  | ColorCommand
  | LineCommand
  | TextCommand
  | FontSizeCommand;

export interface GenericCommand {}

export interface CircleCommand extends GenericCommand {
  type: "circle";
  position: Vector2;
  radius: number;
  fill: boolean;
}

export interface FontSizeCommand extends GenericCommand {
  type: "fontSize";
  value: number;
}

export interface TextCommand extends GenericCommand {
  type: "text";
  position: Vector2;
  text: string;
}

export interface RectCommand extends GenericCommand {
  type: "rect";
  position: Vector2;
  size: Vector2;
  fill: boolean;
}

export interface ThicknessCommand extends GenericCommand {
  type: "thickness";
  value: number;
}

export interface RoundingCommand extends GenericCommand {
  type: "rounding";
  value: number;
}

export interface ColorCommand extends GenericCommand {
  type: "color";
  red: number;
  green: number;
  blue: number;
  alpha: number;
}

export interface LineCommand extends GenericCommand {
  type: "line";
  start: Vector2;
  end: Vector2;
}
