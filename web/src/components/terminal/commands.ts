import { aboutContent } from '@/content/about';
import { cvContent } from '@/content/cv';

import type { CommandRegistry, TerminalBlock } from './terminal.types';

export function createCommandRegistry(): CommandRegistry {
  const commands: CommandRegistry = {
    help: {
      description: 'List available commands',
      execute: () => ({
        type: 'output',
        blocks: [createHelpTable(commands)],
      }),
    },
    about: {
      description: 'About Nikolai Zhdanov',
      execute: () => ({ type: 'output', blocks: aboutContent }),
    },
    cv: {
      description: 'Resume',
      execute: () => ({ type: 'output', blocks: cvContent }),
    },
    clear: {
      description: 'Clear terminal output',
      execute: () => ({ type: 'clear' }),
    },
  };

  return commands;
}

function createHelpTable(registry: CommandRegistry): TerminalBlock {
  return {
    type: 'table',
    bordered: false,
    columns: [{ width: 12 }, { width: 42 }],
    rows: Object.entries(registry).map(([name, command]) => [
      command.usage ?? name,
      command.description,
    ]),
  };
}

export const commandRegistry = createCommandRegistry();
