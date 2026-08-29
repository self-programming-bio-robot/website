import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import styles from './TerminalConsole.module.css';
import TerminalOutput from './TerminalOutput';
import type { TerminalBlock } from './terminal.types';

describe('TerminalOutput', () => {
  it('renders paragraph alignment, keyboard-native links, and media buttons', () => {
    const openMedia = vi.fn();
    const blocks: readonly TerminalBlock[] = [{
      type: 'paragraph',
      align: 'center',
      content: [
        { type: 'link', label: 'Project', href: 'https://example.com', newTab: true },
        ' and ',
        {
          type: 'media',
          mediaType: 'video',
          label: 'Demo',
          src: '/demo.mp4',
          description: 'Project demo',
        },
      ],
    }];

    render(<TerminalOutput blocks={blocks} onOpenMedia={openMedia} />);

    const link = screen.getByRole('link', { name: 'Project' });
    expect(link).toHaveAttribute('href', 'https://example.com');
    expect(link).toHaveAttribute('target', '_blank');
    expect(link.closest('p')).toHaveStyle({ textAlign: 'center' });

    const mediaButton = screen.getByRole('button', { name: 'Demo' });
    fireEvent.click(mediaButton);
    expect(openMedia).toHaveBeenCalledWith(
      expect.objectContaining({ mediaType: 'video', src: '/demo.mp4' }),
      mediaButton,
    );
  });

  it('renders bordered and borderless tables with ch column widths', () => {
    const blocks: readonly TerminalBlock[] = [
      {
        type: 'table',
        bordered: true,
        columns: [{ width: 12 }, { width: 24, align: 'right' }],
        rows: [['one', 'two']],
      },
      {
        type: 'table',
        bordered: false,
        columns: [{ width: 8 }, { width: '*' }],
        rows: [['plain', 'flexible']],
      },
    ];

    const { container } = render(<TerminalOutput blocks={blocks} onOpenMedia={() => {}} />);
    const tables = container.querySelectorAll('table');

    expect(tables[0]).toHaveClass(styles.borderedTable);
    expect(tables[1]).toHaveClass(styles.borderlessTable);
    expect(tables[0].querySelectorAll('col')[0]).toHaveStyle({ width: '12ch' });
    expect(screen.getByText('two')).toHaveStyle({ textAlign: 'right', width: '24ch' });
    expect(tables[1]).toHaveClass(styles.flexibleTable);
    expect(tables[1].querySelectorAll('col')[1]).not.toHaveAttribute('style');
    expect(container.querySelector(`.${styles.characterTable}`)).toHaveTextContent('┌');
    expect(container.querySelector(`.${styles.characterTable}`)).toHaveTextContent('│ one');
  });

  it('applies block colors and lets recursive inline colors override them', () => {
    const openMedia = vi.fn();
    const blocks: readonly TerminalBlock[] = [
      { type: 'pre', color: 'cyan', text: 'cyan pre' },
      { type: 'blank' },
      {
        type: 'paragraph',
        color: 'red',
        content: [
          'red ',
          {
            type: 'color',
            color: 'yellow',
            content: [
              { type: 'link', label: 'colored link', href: '/link' },
              ' and ',
              {
                type: 'media',
                mediaType: 'image',
                label: 'colored media',
                src: '/photo.jpg',
                description: 'Photo',
              },
            ],
          },
        ],
      },
      {
        type: 'table',
        color: 'white',
        bordered: true,
        columns: [{ width: 4 }],
        rows: [['cell']],
      },
    ];

    const { container } = render(<TerminalOutput blocks={blocks} onOpenMedia={openMedia} />);

    expect(screen.getByText('cyan pre')).toHaveStyle({ color: 'var(--terminal-color-cyan)' });
    expect(container.querySelector(`.${styles.blankLine}`)).toHaveAttribute('aria-hidden', 'true');
    expect(container.querySelector('p')).toHaveStyle({
      color: 'var(--terminal-color-red)',
    });
    expect(screen.getByRole('link', { name: 'colored link' }).parentElement).toHaveStyle({
      color: 'var(--terminal-color-yellow)',
    });
    const media = screen.getByRole('button', { name: 'colored media' });
    expect(media.parentElement).toHaveStyle({ color: 'var(--terminal-color-yellow)' });
    fireEvent.click(media);
    expect(openMedia).toHaveBeenCalledOnce();
    expect(container.querySelector(`.${styles.tableViewport}`)).toHaveStyle({
      color: 'var(--terminal-color-white)',
    });
  });

  it('renders inline colors inside bordered and borderless table cells', () => {
    const coloredCell = [{
      type: 'color' as const,
      color: 'yellow' as const,
      content: ['highlighted'],
    }];
    const blocks: readonly TerminalBlock[] = [
      {
        type: 'table',
        bordered: true,
        columns: [{ width: 12 }],
        rows: [[coloredCell]],
      },
      {
        type: 'table',
        bordered: false,
        columns: [{ width: 12 }],
        rows: [[coloredCell]],
      },
    ];

    const { container } = render(<TerminalOutput blocks={blocks} onOpenMedia={() => {}} />);
    const visualColor = container.querySelector(`.${styles.characterTable} span`);
    expect(visualColor).toHaveStyle({ color: 'var(--terminal-color-yellow)' });
    expect(screen.getAllByText('highlighted')).toHaveLength(3);
  });

  it('renders links inside borderless table cells', () => {
    const blocks: readonly TerminalBlock[] = [{
      type: 'table',
      bordered: false,
      columns: [{ width: '*' }],
      rows: [[[{ type: 'link', label: 'open table link', href: '/inside' }]]],
    }];

    render(<TerminalOutput blocks={blocks} onOpenMedia={() => {}} />);
    expect(screen.getByRole('link', { name: 'open table link' })).toHaveAttribute(
      'href',
      '/inside',
    );
  });
});
