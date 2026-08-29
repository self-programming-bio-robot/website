import { describe, expect, it } from 'vitest';

import { parseTerminalMarkdown } from './terminalMarkdown';

describe('parseTerminalMarkdown', () => {
  it('parses paragraphs, links, media, fenced text, alignment, and colors', () => {
    const blocks = parseTerminalMarkdown(`A [link](https://example.com) and ![photo](/photo.jpg "A photo").

\`\`\`text
fixed width
\`\`\`

::::color{value="cyan"}
:::align{value="center"}
Block :color[override]{value="yellow"}.
:::
::::
`, 'sample.md');

    expect(blocks).toEqual([
      {
        type: 'paragraph',
        content: [
          'A ',
          { type: 'link', label: 'link', href: 'https://example.com', newTab: true },
          ' and ',
          {
            type: 'media',
            mediaType: 'image',
            label: 'photo',
            src: '/photo.jpg',
            description: 'A photo',
          },
          '.',
        ],
      },
      { type: 'pre', text: 'fixed width' },
      {
        type: 'paragraph',
        align: 'center',
        color: 'cyan',
        content: [
          'Block ',
          { type: 'color', color: 'yellow', content: ['override'] },
          '.',
        ],
      },
    ]);
  });

  it('supports recursive inline colors containing links and media', () => {
    const [block] = parseTerminalMarkdown(
      ':color[outer [site](/inside) :color[video ![watch](/MOVIE.MP4?x=1#part)]{value="white"}]{value="red"}',
      'inline.md',
    );

    expect(block).toMatchObject({
      type: 'paragraph',
      content: [{
        type: 'color',
        color: 'red',
        content: [
          'outer ',
          { type: 'link', label: 'site', href: '/inside' },
          ' ',
          {
            type: 'color',
            color: 'white',
            content: [
              'video ',
              expect.objectContaining({ type: 'media', mediaType: 'video' }),
            ],
          },
        ],
      }],
    });
  });

  it('infers bare table widths and respects GFM alignment', () => {
    const [table] = parseTerminalMarkdown(`| Name | Status |
| :--- | ---: |
| Longer | ok |
`, 'table.md');

    expect(table).toEqual({
      type: 'table',
      bordered: true,
      columns: [
        { width: 6, align: 'left' },
        { width: 6, align: 'right' },
      ],
      rows: [
        ['Name', 'Status'],
        ['Longer', 'ok'],
      ],
    });
  });

  it('applies table directive metadata', () => {
    const [table] = parseTerminalMarkdown(`:::table{bordered="false" widths="8,12" aligns="center,right"}
| One | Two |
| --- | --- |
| A | B |
:::
`, 'table.md');

    expect(table).toMatchObject({
      type: 'table',
      bordered: false,
      columns: [
        { width: 8, align: 'center' },
        { width: 12, align: 'right' },
      ],
    });
  });

  it('supports one flexible table column', () => {
    const [table] = parseTerminalMarkdown(`:::table{widths="8,*"}
| One | Two |
| --- | --- |
:::
`, 'table.md');

    expect(table).toMatchObject({
      columns: [{ width: 8 }, { width: '*' }],
    });
  });

  it('parses inline colors inside table cells', () => {
    const [table] = parseTerminalMarkdown(`| Name | Status |
| --- | --- |
| API | :color[ready]{value="light-green"} |
`, 'table.md');

    expect(table).toMatchObject({
      type: 'table',
      rows: [
        ['Name', 'Status'],
        ['API', [{ type: 'color', color: 'light-green', content: ['ready'] }]],
      ],
    });
  });

  it('allows interactive inline content in borderless table cells', () => {
    const [table] = parseTerminalMarkdown(`:::table{bordered="false" widths="8,*"}
| Name | Action |
| --- | --- |
| Site | [open](https://example.com) |
:::
`, 'table.md');

    expect(table).toMatchObject({
      bordered: false,
      rows: [
        ['Name', 'Action'],
        ['Site', [{ type: 'link', label: 'open', href: 'https://example.com' }]],
      ],
    });
  });

  it('keeps an auto-linked email as plain text inside a table cell', () => {
    const [table] = parseTerminalMarkdown(`| Contact |
| --- |
| nikolai@example.com |
`, 'table.md');

    expect(table).toMatchObject({
      type: 'table',
      rows: [
        ['Contact'],
        ['nikolai@example.com'],
      ],
    });
  });

  it('parses an explicit blank terminal row', () => {
    expect(parseTerminalMarkdown('before\n\n::blank\n\nafter', 'spacing.md')).toEqual([
      { type: 'paragraph', content: ['before'] },
      { type: 'blank' },
      { type: 'paragraph', content: ['after'] },
    ]);
  });

  it.each([
    ['/movie.MP4', 'video'],
    ['/movie.webm?download=1', 'video'],
    ['/movie.OgV#preview', 'video'],
    ['/movie.mov', 'video'],
    ['/movie.m4v', 'video'],
    ['/photo.PNG?size=2', 'image'],
    ['/asset.unknown', 'image'],
    ['/asset', 'image'],
  ] as const)('detects %s as %s', (url, mediaType) => {
    const [block] = parseTerminalMarkdown(`![open](${url})`, 'media.md');
    expect(block).toMatchObject({
      content: [expect.objectContaining({ mediaType })],
    });
  });

  it.each([
    ['# Heading', "Unsupported Markdown block 'heading'"],
    ['- list', "Unsupported Markdown block 'list'"],
    ['Text with **strong**.', "Unsupported inline Markdown 'strong'"],
    ['Text with `code`.', "Unsupported inline Markdown 'inlineCode'"],
    ['    indented code', 'Indented code blocks are not supported'],
    ['<span>unsafe</span>', "Unsupported inline Markdown 'html'"],
    ['![](/photo.jpg)', 'Media alt text is required'],
    ['[bad](javascript:alert(1))', 'Unsafe or unsupported URL scheme'],
    [':::color{value="orange"}\ntext\n:::', "Invalid terminal color 'orange'"],
    [':::align{value="middle"}\ntext\n:::', "Invalid alignment 'middle'"],
    ['::blank[content]', "The 'blank' directive cannot contain text"],
    ['::blank{size="2"}', "Unsupported attribute 'size' on 'blank'"],
    [':::color{value="red"}\ntext', "Directive 'color' is missing its closing marker"],
    [
      ':::table{widths="10"}\n| A | B |\n| - | - |\n:::',
      'Table widths count (1) must match column count (2)',
    ],
    [
      ':::table{aligns="left"}\n| A | B |\n| - | - |\n:::',
      'Table aligns count (1) must match column count (2)',
    ],
    [
      ':::table{widths="*,*"}\n| A | B |\n| - | - |\n:::',
      "Table widths may contain at most one '*'",
    ],
    [
      ':::table{widths="10,auto"}\n| A | B |\n| - | - |\n:::',
      "Table widths must be positive integers or '*'",
    ],
    [
      '| A |\n| - |\n| [link](https://example.com) |',
      'Bordered table cells support text and inline colors only',
    ],
    ['| A |\n| - |\n| **bold** |', "Unsupported inline Markdown 'strong'"],
  ])('reports source-positioned validation errors for %s', (markdown, message) => {
    expect(() => parseTerminalMarkdown(markdown, 'invalid.md')).toThrow(
      expect.objectContaining({ message: expect.stringMatching(/^invalid\.md:\d+:\d+ /) }),
    );
    expect(() => parseTerminalMarkdown(markdown, 'invalid.md')).toThrow(message);
  });
});
