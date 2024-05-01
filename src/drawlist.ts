import type { Vector2 } from "./math";

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
  | FontAlignCommand
  | LoadTextureCommand
  | TextureCommand
  | TriangleCommand
  | FontSizeCommand;

export interface GenericCommand {}

export interface CircleCommand extends GenericCommand {
  type: "circle";
  position: Vector2;
  radius: number;
  fill: boolean;
}

export interface TriangleCommand extends GenericCommand {
  type: "triangle";
  p1: Vector2;
  p2: Vector2;
  p3: Vector2;
  fill: boolean;
}

export interface LoadTextureCommand extends GenericCommand {
  keyword: "$textureLoad$";
  type: "loadTexture";
  textureName: string;
  data: string;
}

export interface TextureCommand extends GenericCommand {
  type: "texture";
  position: Vector2;
  size: Vector2;
  textureId: string;
}

export interface FontSizeCommand extends GenericCommand {
  type: "fontSize";
  value: number;
}

export interface FontAlignCommand extends GenericCommand {
  type: "fontAlign";
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
