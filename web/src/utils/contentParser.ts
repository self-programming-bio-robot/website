export type ParsedLink = {
  marker: string;
  alt: string;
  src: string;
};

export type ParsedContent = {
  text: string;
  links: ParsedLink[];
};

const linkRegex = /!?\[([^\]]*)\]\(([^)]+)\)/g;

export const parseContent = (input: string): ParsedContent => {
  const links: ParsedLink[] = [];
  let rewritten = input;
  let match: RegExpExecArray | null;
  let cursor = 0;

  while ((match = linkRegex.exec(input)) !== null) {
    const [full, alt = '', src = ''] = match;
    const normalizedAlt = alt.trim();
    const display = normalizedAlt || 'View image';
    const marker = `[[image:${cursor}]]`;

    links.push({
      marker,
      alt: display,
      src: src.trim(),
    });

    rewritten = rewritten.replace(full, `[${display}](${marker})`);
    cursor += 1;
  }

  return {
    text: rewritten.replace(/\n{3,}/g, '\n\n'),
    links,
  };
};
