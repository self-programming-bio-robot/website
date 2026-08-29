import { Fragment, useEffect, useRef, useState, type CSSProperties } from 'react';

import styles from './TerminalConsole.module.css';
import type {
  InlineContent,
  MediaContent,
  TableCell,
  TerminalBlock,
  TerminalColor,
} from './terminal.types';
import {
  formatBorderedTableLines,
  type FormattedTableRun,
  validateTable,
} from './terminal.utils';

type TerminalOutputProps = {
  blocks: readonly TerminalBlock[];
  onOpenMedia: (media: MediaContent, trigger: HTMLButtonElement) => void;
};

export default function TerminalOutput({ blocks, onOpenMedia }: TerminalOutputProps) {
  return (
    <div className={styles.blocks}>
      {blocks.map((block, index) => {
        if (block.type === 'blank') {
          return <div aria-hidden className={styles.blankLine} key={index} />;
        }

        if (block.type === 'pre') {
          return (
            <pre className={styles.preformatted} key={index} style={colorStyle(block.color)}>
              {block.text}
            </pre>
          );
        }

        if (block.type === 'table') {
          return <TerminalTable block={block} key={index} onOpenMedia={onOpenMedia} />;
        }

        const paragraphStyle = {
          ...colorStyle(block.color),
          textAlign: block.align ?? 'left',
        } satisfies CSSProperties;

        return (
          <p className={styles.paragraph} key={index} style={paragraphStyle}>
            {block.content.map((inline, inlineIndex) => (
              <InlineNode
                inline={inline}
                key={inlineIndex}
                onOpenMedia={onOpenMedia}
              />
            ))}
          </p>
        );
      })}
    </div>
  );
}

type TableBlock = Extract<TerminalBlock, { type: 'table' }>;

function TerminalTable({
  block,
  onOpenMedia,
}: {
  block: TableBlock;
  onOpenMedia: TerminalOutputProps['onOpenMedia'];
}) {
  validateTable(block);
  const viewportRef = useRef<HTMLDivElement>(null);
  const measureRef = useRef<HTMLSpanElement>(null);
  const isFlexible = block.columns.some(({ width }) => width === '*');
  const [availableWidth, setAvailableWidth] = useState<number>();

  useEffect(() => {
    if (!isFlexible) {
      return undefined;
    }

    const updateWidth = () => {
      const viewport = viewportRef.current;
      const measure = measureRef.current;
      if (!viewport || !measure) {
        return;
      }

      const characterWidth = measure.getBoundingClientRect().width / 100;
      if (characterWidth > 0) {
        setAvailableWidth(Math.floor(viewport.clientWidth / characterWidth));
      }
    };

    updateWidth();
    if (typeof ResizeObserver === 'undefined') {
      window.addEventListener('resize', updateWidth);
      return () => window.removeEventListener('resize', updateWidth);
    }

    const observer = new ResizeObserver(updateWidth);
    if (viewportRef.current) {
      observer.observe(viewportRef.current);
    }
    return () => observer.disconnect();
  }, [isFlexible]);

  const semanticClassName = block.bordered
    ? `${styles.borderedTable} ${styles.visuallyHidden}`
    : `${styles.borderlessTable}${isFlexible ? ` ${styles.flexibleTable}` : ''}`;
  const borderedLines = block.bordered
    ? formatBorderedTableLines(block, availableWidth)
    : [];

  return (
    <div
      className={styles.tableViewport}
      ref={viewportRef}
      style={colorStyle(block.color)}
    >
      {isFlexible && (
        <span aria-hidden className={styles.characterMeasure} ref={measureRef}>
          {'0'.repeat(100)}
        </span>
      )}
      {block.bordered && (
        <pre aria-hidden className={styles.characterTable}>
          {borderedLines.map((line, lineIndex) => (
            <Fragment key={lineIndex}>
              {line.map((run, runIndex) => (
                <TableRun key={runIndex} run={run} />
              ))}
              {lineIndex < borderedLines.length - 1 ? '\n' : null}
            </Fragment>
          ))}
        </pre>
      )}
      <SemanticTable
        block={block}
        className={semanticClassName}
        onOpenMedia={onOpenMedia}
      />
    </div>
  );
}

function TableRun({
  run,
}: {
  run: FormattedTableRun;
}) {
  return run.color ? <span style={colorStyle(run.color)}>{run.text}</span> : run.text;
}

function SemanticTable({
  block,
  className,
  onOpenMedia,
}: {
  block: TableBlock;
  className: string;
  onOpenMedia: TerminalOutputProps['onOpenMedia'];
}) {
  return (
    <table className={className}>
      <colgroup>
        {block.columns.map((column, columnIndex) => (
          <col
            key={columnIndex}
            style={{ width: column.width === '*' ? undefined : `${column.width}ch` }}
          />
        ))}
      </colgroup>
      <tbody>
        {block.rows.map((row, rowIndex) => (
          <tr key={rowIndex}>
            {row.map((cell, cellIndex) => (
              <td
                key={cellIndex}
                style={{
                  textAlign: block.columns[cellIndex].align ?? 'left',
                  width: block.columns[cellIndex].width === '*'
                    ? undefined
                    : `${block.columns[cellIndex].width}ch`,
                }}
              >
                <TableCellContent cell={cell} onOpenMedia={onOpenMedia} />
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  );
}

function TableCellContent({
  cell,
  onOpenMedia,
}: {
  cell: TableCell;
  onOpenMedia: TerminalOutputProps['onOpenMedia'];
}) {
  if (typeof cell === 'string') {
    return cell;
  }

  return cell.map((inline, index) => (
    <InlineNode inline={inline} key={index} onOpenMedia={onOpenMedia} />
  ));
}

type InlineNodeProps = {
  inline: InlineContent;
  onOpenMedia: TerminalOutputProps['onOpenMedia'];
};

function InlineNode({ inline, onOpenMedia }: InlineNodeProps) {
  if (typeof inline === 'string') {
    return inline;
  }

  if (inline.type === 'link') {
    return (
      <a
        className={styles.inlineLink}
        href={inline.href}
        rel={inline.newTab ? 'noopener noreferrer' : undefined}
        target={inline.newTab ? '_blank' : undefined}
      >
        {inline.label}
      </a>
    );
  }

  if (inline.type === 'color') {
    return (
      <span style={colorStyle(inline.color)}>
        {inline.content.map((child, index) => (
          <InlineNode inline={child} key={index} onOpenMedia={onOpenMedia} />
        ))}
      </span>
    );
  }

  return (
    <button
      className={styles.mediaTrigger}
      onClick={(event) => onOpenMedia(inline, event.currentTarget)}
      type="button"
    >
      {inline.label}
    </button>
  );
}

function colorStyle(color: TerminalColor | undefined): CSSProperties | undefined {
  return color ? { color: `var(--terminal-color-${color})` } : undefined;
}
