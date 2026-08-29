import { describe, expect, it } from 'vitest';

import {
  compileTerminalContent,
  CONTENT_SOURCES,
  renderGeneratedModule,
} from './compile-terminal-content';
import {
  aboutContent,
  cvContent,
  mainContent,
} from '../src/content/generated/terminalContent';

describe('terminal content generation', () => {
  it('serializes typed output deterministically', () => {
    const content = [['sample', [{ type: 'paragraph', content: ['hello'] }]]] as const;
    const first = renderGeneratedModule(content);
    const second = renderGeneratedModule(content);

    expect(first).toBe(second);
    expect(first).toContain('as const satisfies readonly TerminalBlock[]');
  });

  it('does not rewrite an already-current generated module', async () => {
    await expect(compileTerminalContent()).resolves.toBe(false);
  });

  it('migrates every declared website content source', () => {
    expect(CONTENT_SOURCES.map(([name]) => name)).toEqual([
      'mainContent',
      'aboutContent',
      'cvContent',
    ]);
    expect(mainContent.some((block) => block.type === 'table')).toBe(true);
    expect(mainContent.filter((block) => block.type === 'blank')).toHaveLength(5);
    expect(mainContent.some((block) => (
      block.type === 'paragraph'
      && block.content.some((inline) => typeof inline !== 'string' && inline.type === 'link')
    ))).toBe(true);
    expect(aboutContent.some((block) => (
      block.type === 'paragraph'
      && block.content.some((inline) => typeof inline !== 'string' && inline.type === 'media')
    ))).toBe(true);
    const cvTables = cvContent.filter((block) => block.type === 'table');
    expect(cvTables).toHaveLength(2);
    expect(cvTables[0]).toMatchObject({
      bordered: true,
      columns: [
        { width: 29, align: 'left' },
        { width: '*', align: 'left' },
      ],
    });
    expect(cvTables[1]).toMatchObject({
      bordered: false,
      columns: [
        { width: 29, align: 'left' },
        { width: '*', align: 'left' },
      ],
    });
    expect(cvTables.some((table) => table.rows.some((row) => row.some((cell) => (
      Array.isArray(cell)
      && cell.some((inline) => typeof inline !== 'string' && inline.type === 'color')
    ))))).toBe(true);
    expect(cvContent.every((block) => !('color' in block) || block.color === 'light-cyan')).toBe(true);
  });
});
