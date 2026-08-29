export type ParagraphAlignment = 'left' | 'center' | 'right' | 'justify';

export const TERMINAL_COLORS = [
  'black',
  'blue',
  'green',
  'cyan',
  'red',
  'magenta',
  'brown',
  'light-gray',
  'dark-gray',
  'light-blue',
  'light-green',
  'light-cyan',
  'light-red',
  'light-magenta',
  'yellow',
  'white',
] as const;

export type TerminalColor = (typeof TERMINAL_COLORS)[number];

export type LinkContent = {
  type: 'link';
  label: string;
  href: string;
  newTab?: boolean;
};

export type MediaContent = {
  type: 'media';
  mediaType: 'image' | 'video';
  label: string;
  src: string;
  description: string;
};

export type ColorContent = {
  type: 'color';
  color: TerminalColor;
  content: readonly InlineContent[];
};

export type InlineContent = string | LinkContent | MediaContent | ColorContent;

export type TableCell = string | readonly InlineContent[];

export type TableColumn = {
  width: number | '*';
  align?: 'left' | 'center' | 'right';
};

type TerminalBlockColor = {
  color?: TerminalColor;
};

export type TerminalBlock = TerminalBlockColor & (
  | {
      type: 'blank';
    }
  | {
      type: 'paragraph';
      content: readonly InlineContent[];
      align?: ParagraphAlignment;
    }
  | {
      type: 'pre';
      text: string;
    }
  | {
      type: 'table';
      bordered?: boolean;
      columns: readonly TableColumn[];
      rows: readonly (readonly TableCell[])[];
    }
);

export type CommandResult =
  | { type: 'output'; blocks: readonly TerminalBlock[] }
  | { type: 'clear' };

export type CommandDefinition = {
  description: string;
  usage?: string;
  execute: (args: readonly string[]) => CommandResult | Promise<CommandResult>;
};

export type CommandRegistry = Record<string, CommandDefinition>;

export type TerminalEntry = {
  id: number;
  command: string | null;
  output: readonly TerminalBlock[];
};
