import type {
  CommandRegistry,
  TableCell,
  TerminalBlock,
  TerminalColor,
} from './terminal.types';

export const MAX_SUGGESTIONS = 5;

export function parseCommand(input: string): { name: string; args: string[] } | null {
  const trimmed = input.trim();

  if (!trimmed) {
    return null;
  }

  const [name, ...args] = trimmed.split(/\s+/);
  return { name: name.toLowerCase(), args };
}

export function getCommandSuggestions(input: string, registry: CommandRegistry): string[] {
  const candidate = input.trimStart();

  if (!candidate || /\s/.test(candidate)) {
    return [];
  }

  const prefix = candidate.toLowerCase();
  return Object.keys(registry)
    .filter((name) => name.startsWith(prefix) && name !== prefix)
    .slice(0, MAX_SUGGESTIONS);
}

export function formatSuggestionWindow(suggestions: readonly string[]): string {
  if (suggestions.length === 0) {
    return '';
  }

  const innerWidth = Math.max(12, ...suggestions.map((suggestion) => characterLength(suggestion) + 2));
  const border = '─'.repeat(innerWidth + 2);
  const rows = suggestions.map((suggestion, index) => {
    const marker = index === 0 ? '› ' : '  ';
    return `│ ${(marker + suggestion).padEnd(innerWidth)} │`;
  });

  return [`┌${border}┐`, ...rows, `└${border}┘`].join('\n');
}

export function validateTable(block: Extract<TerminalBlock, { type: 'table' }>): void {
  if (block.columns.length === 0) {
    throw new Error('A terminal table must define at least one column.');
  }

  if (block.columns.some(({ width }) => (
    width !== '*' && (!Number.isInteger(width) || width <= 0)
  ))) {
    throw new Error("Terminal table column widths must be positive integers or '*'.");
  }

  if (block.columns.filter(({ width }) => width === '*').length > 1) {
    throw new Error("A terminal table may define at most one '*' column.");
  }

  if (block.rows.some((row) => row.length !== block.columns.length)) {
    throw new Error('Every terminal table row must match the number of columns.');
  }

  if (block.bordered && block.rows.some((row) => row.some(tableCellHasInteractiveContent))) {
    throw new Error('Bordered terminal table cells support text and inline colors only.');
  }
}

export function formatBorderedTable(
  block: Extract<TerminalBlock, { type: 'table' }>,
  availableWidth?: number,
): string {
  return formatBorderedTableLines(block, availableWidth)
    .map((line) => line.map((run) => run.text).join(''))
    .join('\n');
}

export type FormattedTableRun = {
  text: string;
  color?: TerminalColor;
};

export function formatBorderedTableLines(
  block: Extract<TerminalBlock, { type: 'table' }>,
  availableWidth?: number,
): FormattedTableRun[][] {
  validateTable(block);
  const columns = resolveBorderedColumns(block, availableWidth);

  const horizontal = columns.map(({ width }) => '─'.repeat(width + 2));
  const lines: FormattedTableRun[][] = [[{ text: `┌${horizontal.join('┬')}┐` }]];

  block.rows.forEach((row, rowIndex) => {
    const wrappedCells = row.map((cell, columnIndex) => (
      wrapCell(cell, columns[columnIndex].width)
    ));
    const rowHeight = Math.max(...wrappedCells.map((cellLines) => cellLines.length));

    for (let lineIndex = 0; lineIndex < rowHeight; lineIndex += 1) {
      const line: TableGlyph[] = glyphs('│ ');
      wrappedCells.forEach((cellLines, columnIndex) => {
        const column = columns[columnIndex];
        line.push(...alignCell(
          cellLines[lineIndex] ?? [],
          column.width,
          column.align ?? 'left',
        ));
        line.push(...glyphs(columnIndex < columns.length - 1 ? ' │ ' : ' │'));
      });
      lines.push(toRuns(line));
    }

    if (rowIndex < block.rows.length - 1) {
      lines.push([{ text: `├${horizontal.join('┼')}┤` }]);
    }
  });

  lines.push([{ text: `└${horizontal.join('┴')}┘` }]);
  return lines;
}

function resolveBorderedColumns(
  block: Extract<TerminalBlock, { type: 'table' }>,
  availableWidth?: number,
): { width: number; align?: 'left' | 'center' | 'right' }[] {
  const flexibleIndex = block.columns.findIndex(({ width }) => width === '*');
  if (flexibleIndex === -1) {
    return block.columns as { width: number; align?: 'left' | 'center' | 'right' }[];
  }

  const contentWidth = Math.max(
    1,
    ...block.rows.flatMap((row) => (
      tableCellText(row[flexibleIndex]).split('\n').map((line) => characterLength(line))
    )),
  );
  const fixedWidth = block.columns.reduce((total, column) => (
    total + (column.width === '*' ? 0 : column.width)
  ), 0);
  const frameWidth = (block.columns.length * 3) + 1;
  const flexibleWidth = availableWidth && Number.isFinite(availableWidth)
    ? Math.max(1, Math.floor(availableWidth) - fixedWidth - frameWidth)
    : contentWidth;

  return block.columns.map((column) => ({
    ...column,
    width: column.width === '*' ? flexibleWidth : column.width,
  }));
}

type TableGlyph = {
  value: string;
  color?: TerminalColor;
};

function wrapCell(cell: TableCell, width: number): TableGlyph[][] {
  const value = cellGlyphs(cell);
  if (value.some((item) => item.value === '\n')) {
    const sourceLines: TableGlyph[][] = [[]];
    value.forEach((item) => {
      if (item.value === '\n') {
        sourceLines.push([]);
      } else {
        sourceLines[sourceLines.length - 1].push(item);
      }
    });
    return sourceLines.flatMap((sourceLine) => splitByCharacterWidth(sourceLine, width));
  }

  const words: TableGlyph[][] = [];
  let currentWord: TableGlyph[] = [];
  value.forEach((item) => {
    if (/\s/u.test(item.value)) {
      if (currentWord.length > 0) {
        words.push(currentWord);
        currentWord = [];
      }
    } else {
      currentWord.push(item);
    }
  });
  if (currentWord.length > 0) {
    words.push(currentWord);
  }

  if (words.length === 0) {
    return [[]];
  }

  const wrapped: TableGlyph[][] = [];
  let currentLine: TableGlyph[] = [];
  words.forEach((word) => {
    splitByCharacterWidth(word, width).forEach((chunk) => {
      if (currentLine.length === 0) {
        currentLine = chunk;
      } else if (currentLine.length + 1 + chunk.length <= width) {
        currentLine = [...currentLine, ...glyphs(' '), ...chunk];
      } else {
        wrapped.push(currentLine);
        currentLine = chunk;
      }

      if (currentLine.length === width) {
        wrapped.push(currentLine);
        currentLine = [];
      }
    });
  });

  if (currentLine.length > 0) {
    wrapped.push(currentLine);
  }

  return wrapped.length > 0 ? wrapped : [[]];
}

function splitByCharacterWidth(value: TableGlyph[], width: number): TableGlyph[][] {
  const chunks: TableGlyph[][] = [];

  for (let index = 0; index < value.length; index += width) {
    chunks.push(value.slice(index, index + width));
  }

  return chunks.length > 0 ? chunks : [[]];
}

function alignCell(
  value: TableGlyph[],
  width: number,
  alignment: 'left' | 'center' | 'right',
): TableGlyph[] {
  const padding = Math.max(0, width - value.length);

  if (alignment === 'right') {
    return [...glyphs(' '.repeat(padding)), ...value];
  }

  if (alignment === 'center') {
    const leftPadding = Math.floor(padding / 2);
    return [
      ...glyphs(' '.repeat(leftPadding)),
      ...value,
      ...glyphs(' '.repeat(padding - leftPadding)),
    ];
  }

  return [...value, ...glyphs(' '.repeat(padding))];
}

function cellGlyphs(cell: TableCell, inheritedColor?: TerminalColor): TableGlyph[] {
  const content = typeof cell === 'string' ? [cell] : cell;
  return content.flatMap((inline): TableGlyph[] => {
    if (typeof inline === 'string') {
      return glyphs(inline, inheritedColor);
    }
    if (inline.type === 'color') {
      return cellGlyphs(inline.content, inline.color);
    }
    return glyphs(inline.label, inheritedColor);
  });
}

function tableCellText(cell: TableCell): string {
  return cellGlyphs(cell).map((item) => item.value).join('');
}

function glyphs(
  value: string,
  color?: TerminalColor,
): TableGlyph[] {
  return Array.from(value).map((character) => ({
    value: character,
    ...(color ? { color } : {}),
  }));
}

function toRuns(value: TableGlyph[]): FormattedTableRun[] {
  const runs: FormattedTableRun[] = [];
  value.forEach((item) => {
    const previous = runs[runs.length - 1];
    if (previous && previous.color === item.color) {
      previous.text += item.value;
    } else {
      runs.push({
        text: item.value,
        ...(item.color ? { color: item.color } : {}),
      });
    }
  });
  return runs;
}

function tableCellHasInteractiveContent(cell: TableCell): boolean {
  if (typeof cell === 'string') {
    return false;
  }

  return cell.some((inline) => (
    typeof inline !== 'string'
    && (inline.type !== 'color' || tableCellHasInteractiveContent(inline.content))
  ));
}

function characterLength(value: string): number {
  return Array.from(value).length;
}
