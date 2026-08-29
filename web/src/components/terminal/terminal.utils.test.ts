import { describe, expect, it } from 'vitest';

import { createCommandRegistry } from './commands';
import {
  formatBorderedTable,
  formatBorderedTableLines,
  formatSuggestionWindow,
  getCommandSuggestions,
  parseCommand,
  validateTable,
} from './terminal.utils';

describe('terminal utilities', () => {
  const registry = createCommandRegistry();

  it('parses a command name and whitespace-separated arguments', () => {
    expect(parseCommand('  ABOUT   one two  ')).toEqual({
      name: 'about',
      args: ['one', 'two'],
    });
    expect(parseCommand('   ')).toBeNull();
  });

  it('returns case-insensitive prefix suggestions for the command token only', () => {
    expect(getCommandSuggestions('H', registry)).toEqual(['help']);
    expect(getCommandSuggestions('about ', registry)).toEqual([]);
    expect(getCommandSuggestions('help', registry)).toEqual([]);
  });

  it('renders suggestions as a text-only popup window', () => {
    expect(formatSuggestionWindow(['help', 'history'])).toBe([
      '┌──────────────┐',
      '│ › help       │',
      '│   history    │',
      '└──────────────┘',
    ].join('\n'));
  });

  it('builds help output from the command registry', () => {
    const result = registry.help.execute([]);
    expect(result).not.toBeInstanceOf(Promise);
    expect(result).toMatchObject({
      type: 'output',
      blocks: [{ type: 'table', bordered: false }],
    });
  });

  it('returns a clear result from the clear command', () => {
    expect(registry.clear.execute([])).toEqual({ type: 'clear' });
  });

  it('rejects invalid table widths and row shapes', () => {
    expect(() => validateTable({
      type: 'table',
      columns: [{ width: 0 }],
      rows: [['value']],
    })).toThrow(/positive integers/);

    expect(() => validateTable({
      type: 'table',
      columns: [{ width: 10 }, { width: 10 }],
      rows: [['only one cell']],
    })).toThrow(/number of columns/);
  });

  it('renders bordered tables with DOS box-drawing characters', () => {
    expect(formatBorderedTable({
      type: 'table',
      bordered: true,
      columns: [{ width: 5 }, { width: 4, align: 'right' }],
      rows: [['Name', '7'], ['Long value', '42']],
    })).toBe([
      '┌───────┬──────┐',
      '│ Name  │    7 │',
      '├───────┼──────┤',
      '│ Long  │   42 │',
      '│ value │      │',
      '└───────┴──────┘',
    ].join('\n'));
  });

  it('preserves explicit whitespace in multiline bordered table cells', () => {
    expect(formatBorderedTable({
      type: 'table',
      bordered: true,
      columns: [{ width: 5 }, { width: 5 }],
      rows: [['A\n  B', 'text']],
    })).toContain('│   B   │       │');
  });

  it('stretches a star column to the available character width', () => {
    const output = formatBorderedTable({
      type: 'table',
      bordered: true,
      columns: [{ width: 5 }, { width: '*' }],
      rows: [['Name', 'Value']],
    }, 20);

    expect(output.split('\n').every((line) => Array.from(line).length === 20)).toBe(true);
    expect(output).toContain('┌───────┬──────────┐');
  });

  it('rejects more than one star column at runtime', () => {
    expect(() => validateTable({
      type: 'table',
      columns: [{ width: '*' }, { width: '*' }],
      rows: [['A', 'B']],
    })).toThrow(/at most one/);
  });

  it('rejects interactive content in bordered table cells at runtime', () => {
    expect(() => validateTable({
      type: 'table',
      bordered: true,
      columns: [{ width: 8 }],
      rows: [[[{ type: 'link', label: 'open', href: '/page' }]]],
    })).toThrow(/text and inline colors only/);
  });

  it('preserves inline colors while formatting bordered table cells', () => {
    const lines = formatBorderedTableLines({
      type: 'table',
      bordered: true,
      columns: [{ width: 8 }],
      rows: [[[{ type: 'color', color: 'yellow', content: ['ready'] }]]],
    });

    expect(lines.flat()).toContainEqual({ text: 'ready', color: 'yellow' });
    expect(lines.map((line) => line.map((run) => run.text).join('')).join('\n'))
      .toContain('│ ready');
  });
});
