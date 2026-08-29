import type {
  Code,
  Image,
  Link,
  Paragraph,
  Root,
  RootContent,
  Table,
  Text,
} from 'mdast';
import remarkDirective from 'remark-directive';
import remarkGfm from 'remark-gfm';
import remarkParse from 'remark-parse';
import { unified } from 'unified';

import {
  TERMINAL_COLORS,
  type InlineContent,
  type ParagraphAlignment,
  type TableCell,
  type TableColumn,
  type TerminalBlock,
  type TerminalColor,
} from '@/components/terminal/terminal.types';

type SourcePosition = {
  start: { line: number; column: number; offset?: number };
  end: { line: number; column: number; offset?: number };
};

type DirectiveAttributes = Record<string, string | null | undefined> | null | undefined;

type TextDirectiveNode = {
  type: 'textDirective';
  name: string;
  attributes?: DirectiveAttributes;
  children: InlineNode[];
  position?: SourcePosition;
};

type ContainerDirectiveNode = {
  type: 'containerDirective';
  name: string;
  attributes?: DirectiveAttributes;
  children: BlockNode[];
  position?: SourcePosition;
};

type LeafDirectiveNode = {
  type: 'leafDirective';
  name: string;
  attributes?: DirectiveAttributes;
  children: InlineNode[];
  position?: SourcePosition;
};

type InlineNode = Paragraph['children'][number] | TextDirectiveNode;
type BlockNode = RootContent | ContainerDirectiveNode | LeafDirectiveNode;

const TERMINAL_COLOR_SET = new Set<string>(TERMINAL_COLORS);
const ALIGNMENTS = new Set<ParagraphAlignment>(['left', 'center', 'right', 'justify']);
const TABLE_ALIGNMENTS = new Set<TableColumn['align']>(['left', 'center', 'right']);
const VIDEO_EXTENSIONS = new Set(['mp4', 'webm', 'ogv', 'mov', 'm4v']);

export class TerminalMarkdownError extends Error {
  constructor(filePath: string, message: string, position?: SourcePosition) {
    const location = position
      ? `${filePath}:${position.start.line}:${position.start.column}`
      : filePath;
    super(`${location} ${message}`);
    this.name = 'TerminalMarkdownError';
  }
}

export function parseTerminalMarkdown(source: string, filePath = '<markdown>'): TerminalBlock[] {
  const root = unified()
    .use(remarkParse)
    .use(remarkGfm)
    .use(remarkDirective)
    .parse(source) as Root;

  return parseBlocks(root.children as BlockNode[], filePath, source);
}

function parseBlocks(
  nodes: readonly BlockNode[],
  filePath: string,
  source: string,
  inheritedColor?: TerminalColor,
): TerminalBlock[] {
  return nodes.flatMap((node) => {
    if (node.type === 'paragraph') {
      return [withColor({
        type: 'paragraph',
        content: parseInline(node.children as InlineNode[], filePath),
      }, inheritedColor)];
    }

    if (node.type === 'code') {
      validateFencedCode(node, filePath, source);
      return [withColor({ type: 'pre', text: node.value }, inheritedColor)];
    }

    if (node.type === 'table') {
      return [withColor(parseTable(node, filePath), inheritedColor)];
    }

    if (node.type === 'containerDirective') {
      return parseContainerDirective(node, filePath, source, inheritedColor);
    }

    if (node.type === 'leafDirective') {
      return [parseLeafDirective(node, filePath, inheritedColor)];
    }

    throw markdownError(filePath, node, `Unsupported Markdown block '${node.type}'.`);
  });
}

function parseLeafDirective(
  node: LeafDirectiveNode,
  filePath: string,
  inheritedColor?: TerminalColor,
): TerminalBlock {
  if (node.name !== 'blank') {
    throw markdownError(filePath, node, `Unsupported directive '${node.name}'.`);
  }
  assertOnlyAttributes(node, filePath, []);
  if (node.children.length > 0) {
    throw markdownError(filePath, node, "The 'blank' directive cannot contain text.");
  }
  return withColor({ type: 'blank' }, inheritedColor);
}

function parseContainerDirective(
  node: ContainerDirectiveNode,
  filePath: string,
  source: string,
  inheritedColor?: TerminalColor,
): TerminalBlock[] {
  validateContainerFence(node, filePath, source);

  if (node.name === 'color') {
    assertOnlyAttributes(node, filePath, ['value']);
    const color = parseColor(attribute(node, 'value', filePath), filePath, node);
    return parseBlocks(node.children, filePath, source, color);
  }

  if (node.name === 'align') {
    assertOnlyAttributes(node, filePath, ['value']);
    const alignment = attribute(node, 'value', filePath);
    if (!ALIGNMENTS.has(alignment as ParagraphAlignment)) {
      throw markdownError(filePath, node, `Invalid alignment '${alignment}'.`);
    }

    return parseBlocks(node.children, filePath, source, inheritedColor).map((block) => {
      if (block.type !== 'paragraph') {
        throw markdownError(filePath, node, "The 'align' directive may contain paragraphs only.");
      }
      return { ...block, align: alignment as ParagraphAlignment };
    });
  }

  if (node.name === 'table') {
    assertOnlyAttributes(node, filePath, ['bordered', 'widths', 'aligns']);
    if (node.children.length !== 1 || node.children[0].type !== 'table') {
      throw markdownError(filePath, node, "The 'table' directive must contain exactly one GFM table.");
    }

    return [withColor(parseTable(node.children[0], filePath, node), inheritedColor)];
  }

  throw markdownError(filePath, node, `Unsupported directive '${node.name}'.`);
}

function validateContainerFence(
  node: ContainerDirectiveNode,
  filePath: string,
  source: string,
): void {
  const startOffset = node.position?.start.offset;
  const endOffset = node.position?.end.offset;
  const sourceAtNode = startOffset === undefined || endOffset === undefined
    ? ''
    : source.slice(startOffset, endOffset).trimEnd();
  const lastLine = sourceAtNode.split(/\r?\n/).at(-1) ?? '';
  if (!/^:{3,}$/.test(lastLine.trim())) {
    throw markdownError(filePath, node, `Directive '${node.name}' is missing its closing marker.`);
  }
}

function parseInline(nodes: readonly InlineNode[], filePath: string): InlineContent[] {
  return nodes.flatMap((node): InlineContent[] => {
    if (node.type === 'text') {
      const value = (node as Text).value;
      if (/(?:^|\n):{3,}(?:[a-z]|$)/i.test(value)) {
        throw markdownError(filePath, node, 'Malformed or unmatched container directive marker.');
      }
      return [value];
    }

    if (node.type === 'link') {
      return [parseLink(node as Link, filePath)];
    }

    if (node.type === 'image') {
      return [parseMedia(node as Image, filePath)];
    }

    if (node.type === 'textDirective') {
      if (node.name !== 'color') {
        throw markdownError(filePath, node, `Unsupported inline directive '${node.name}'.`);
      }
      assertOnlyAttributes(node, filePath, ['value']);
      const color = parseColor(attribute(node, 'value', filePath), filePath, node);
      const content = parseInline(node.children, filePath);
      if (content.length === 0) {
        throw markdownError(filePath, node, "The inline 'color' directive cannot be empty.");
      }
      return [{ type: 'color', color, content }];
    }

    throw markdownError(filePath, node, `Unsupported inline Markdown '${node.type}'.`);
  });
}

function parseLink(node: Link, filePath: string) {
  const label = plainInlineText(node.children as InlineNode[], filePath, node, 'link');
  validateUrl(node.url, filePath, node);
  return {
    type: 'link' as const,
    label,
    href: node.url,
    ...(isExternalHttpUrl(node.url) ? { newTab: true } : {}),
  };
}

function parseMedia(node: Image, filePath: string) {
  const label = node.alt?.trim();
  if (!label) {
    throw markdownError(filePath, node, 'Media alt text is required and becomes its visible label.');
  }
  validateUrl(node.url, filePath, node);

  return {
    type: 'media' as const,
    mediaType: detectMediaType(node.url),
    label,
    src: node.url,
    description: node.title?.trim() || label,
  };
}

function parseTable(
  node: Table,
  filePath: string,
  directive?: ContainerDirectiveNode,
): Extract<TerminalBlock, { type: 'table' }> {
  const bordered = directive ? parseBooleanAttribute(directive, 'bordered', true, filePath) : true;
  const rows = node.children.map((row) => row.children.map((cell) => (
    parseTableCell(cell.children as InlineNode[], filePath)
  )));
  const columnCount = rows[0]?.length ?? 0;

  if (columnCount === 0 || rows.some((row) => row.length !== columnCount)) {
    throw markdownError(filePath, node, 'Tables must contain at least one column and consistent rows.');
  }
  if (bordered && rows.some((row) => row.some(tableCellHasInteractiveContent))) {
    throw markdownError(
      filePath,
      node,
      'Bordered table cells support text and inline colors only.',
    );
  }

  const widths = directive?.attributes?.widths
    ? parseWidths(directive.attributes.widths, columnCount, filePath, directive)
    : Array.from({ length: columnCount }, (_, columnIndex) => (
      Math.max(1, ...rows.flatMap((row) => (
        tableCellText(row[columnIndex]).split('\n').map(characterLength)
      )))
    ));

  const explicitAlignments = directive?.attributes?.aligns
    ? parseTableAlignments(directive.attributes.aligns, columnCount, filePath, directive)
    : undefined;
  const columns = widths.map((width, index): TableColumn => {
    const alignment = explicitAlignments?.[index] ?? node.align?.[index] ?? undefined;
    return alignment ? { width, align: alignment } : { width };
  });

  return {
    type: 'table',
    bordered,
    columns,
    rows,
  };
}

function parseTableCell(nodes: readonly InlineNode[], filePath: string): TableCell {
  const content = nodes.flatMap((node): InlineContent[] => {
    if (
      node.type === 'link'
      && node.url.toLowerCase().startsWith('mailto:')
      && node.children.every((child) => child.type === 'text')
    ) {
      return [node.children.map((child) => (child as Text).value).join('')];
    }

    return parseInline([node], filePath);
  });

  return content.every((inline) => typeof inline === 'string')
    ? content.join('')
    : content;
}

function tableCellText(cell: TableCell): string {
  if (typeof cell === 'string') {
    return cell;
  }

  return cell.map((inline): string => {
    if (typeof inline === 'string') {
      return inline;
    }
    if (inline.type === 'color') {
      return tableCellText(inline.content);
    }
    return inline.label;
  }).join('');
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

function validateFencedCode(node: Code, filePath: string, source: string): void {
  const startOffset = node.position?.start.offset;
  const sourceAtNode = startOffset === undefined ? '' : source.slice(startOffset);
  if (!/^ {0,3}(?:`{3,}|~{3,})/.test(sourceAtNode)) {
    throw markdownError(filePath, node, 'Indented code blocks are not supported. Use a fenced text block.');
  }
  if (node.lang && node.lang.toLowerCase() !== 'text') {
    throw markdownError(filePath, node, `Unsupported fenced-code language '${node.lang}'. Use 'text'.`);
  }
  if (node.meta) {
    throw markdownError(filePath, node, 'Fenced-code metadata is not supported.');
  }
}

function plainInlineText(
  nodes: readonly InlineNode[],
  filePath: string,
  parent: { position?: SourcePosition },
  context: string,
): string {
  return nodes.map((node) => {
    if (node.type === 'text') {
      return (node as Text).value;
    }

    if (
      context === 'table cell'
      && node.type === 'link'
      && node.url.toLowerCase().startsWith('mailto:')
      && node.children.every((child) => child.type === 'text')
    ) {
      return node.children.map((child) => (child as Text).value).join('');
    }

    throw markdownError(filePath, parent, `${context[0].toUpperCase()}${context.slice(1)} must contain plain text only.`);
  }).join('');
}

function parseWidths(
  raw: string,
  columnCount: number,
  filePath: string,
  node: ContainerDirectiveNode,
): TableColumn['width'][] {
  const widths = raw.split(',').map((value): TableColumn['width'] => {
    const trimmed = value.trim();
    return trimmed === '*' ? '*' : Number(trimmed);
  });
  if (widths.length !== columnCount) {
    throw markdownError(filePath, node, `Table widths count (${widths.length}) must match column count (${columnCount}).`);
  }
  if (widths.filter((width) => width === '*').length > 1) {
    throw markdownError(filePath, node, "Table widths may contain at most one '*'.");
  }
  if (widths.some((width) => width !== '*' && (!Number.isInteger(width) || width <= 0))) {
    throw markdownError(filePath, node, "Table widths must be positive integers or '*'.");
  }
  return widths;
}

function parseTableAlignments(
  raw: string,
  columnCount: number,
  filePath: string,
  node: ContainerDirectiveNode,
): NonNullable<TableColumn['align']>[] {
  const alignments = raw.split(',').map((value) => value.trim());
  if (alignments.length !== columnCount) {
    throw markdownError(filePath, node, `Table aligns count (${alignments.length}) must match column count (${columnCount}).`);
  }
  const invalid = alignments.find((alignment) => !TABLE_ALIGNMENTS.has(alignment as TableColumn['align']));
  if (invalid) {
    throw markdownError(filePath, node, `Invalid table alignment '${invalid}'.`);
  }
  return alignments as NonNullable<TableColumn['align']>[];
}

function parseBooleanAttribute(
  node: ContainerDirectiveNode,
  name: string,
  defaultValue: boolean,
  filePath: string,
): boolean {
  const value = node.attributes?.[name];
  if (value === undefined || value === null) {
    return defaultValue;
  }
  if (value !== 'true' && value !== 'false') {
    throw markdownError(filePath, node, `Attribute '${name}' must be 'true' or 'false'.`);
  }
  return value === 'true';
}

function attribute(
  node: TextDirectiveNode | ContainerDirectiveNode,
  name: string,
  filePath: string,
): string {
  const value = node.attributes?.[name];
  if (!value) {
    throw markdownError(filePath, node, `Directive '${node.name}' requires attribute '${name}'.`);
  }
  return value;
}

function assertOnlyAttributes(
  node: TextDirectiveNode | ContainerDirectiveNode | LeafDirectiveNode,
  filePath: string,
  allowed: readonly string[],
): void {
  const unexpected = Object.keys(node.attributes ?? {}).find((name) => !allowed.includes(name));
  if (unexpected) {
    throw markdownError(filePath, node, `Unsupported attribute '${unexpected}' on '${node.name}'.`);
  }
}

function parseColor(
  value: string,
  filePath: string,
  node: { position?: SourcePosition },
): TerminalColor {
  if (!TERMINAL_COLOR_SET.has(value)) {
    throw markdownError(filePath, node, `Invalid terminal color '${value}'.`);
  }
  return value as TerminalColor;
}

function detectMediaType(url: string): 'image' | 'video' {
  const path = url.split(/[?#]/, 1)[0];
  const extension = path.match(/\.([^.\/]+)$/)?.[1].toLowerCase();
  return extension && VIDEO_EXTENSIONS.has(extension) ? 'video' : 'image';
}

function validateUrl(url: string, filePath: string, node: { position?: SourcePosition }): void {
  const trimmed = url.trim();
  if (!trimmed) {
    throw markdownError(filePath, node, 'URL cannot be empty.');
  }

  if (/^(?:https?:|mailto:)/i.test(trimmed) || /^(?:\/\/|\/|\.\.?\/|#|\?)/.test(trimmed)) {
    return;
  }

  if (/^[a-z][a-z\d+.-]*:/i.test(trimmed)) {
    throw markdownError(filePath, node, `Unsafe or unsupported URL scheme in '${url}'.`);
  }
}

function isExternalHttpUrl(url: string): boolean {
  return /^(?:https?:)?\/\//i.test(url);
}

function withColor<T extends TerminalBlock>(block: T, color?: TerminalColor): T {
  return (color ? { ...block, color } : block) as T;
}

function characterLength(value: string): number {
  return Array.from(value).length;
}

function markdownError(
  filePath: string,
  node: { position?: SourcePosition; type?: string },
  message: string,
): TerminalMarkdownError {
  return new TerminalMarkdownError(filePath, message, node.position);
}
