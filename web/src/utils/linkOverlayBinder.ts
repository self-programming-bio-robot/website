import { ParsedLink } from '@/utils/contentParser';

export type HoverLinkInfo = {
  x: number;
  y: number;
  alt: string;
};

const markerRegex = /\[([^\]]+)\]\(\[\[image:(\d+)\]\]\)/g;

export const transformMarkersToLinks = (
  container: Element,
  commandId: string,
  findLink: (marker: string) => ParsedLink | undefined,
  openMarker: (marker: string) => void,
  setHover: (info: HoverLinkInfo | null) => void,
) => {
  const walker = document.createTreeWalker(container, NodeFilter.SHOW_TEXT);
  const textNodes: Text[] = [];

  while (walker.nextNode()) {
    const node = walker.currentNode as Text;
    if (!node.parentElement || node.parentElement.closest('a[data-overlay-marker]')) {
      continue;
    }

    if (node.textContent && node.textContent.includes('[[image:')) {
      textNodes.push(node);
    }
  }

  textNodes.forEach((textNode) => {
    const text = textNode.textContent ?? '';
    const matches = [...text.matchAll(markerRegex)];

    if (!matches.length) {
      return;
    }

    const fragment = document.createDocumentFragment();
    let lastIndex = 0;

    matches.forEach((match) => {
      const [full, label, index] = match;
      const start = match.index ?? 0;
      const end = start + full.length;

      if (start > lastIndex) {
        fragment.appendChild(document.createTextNode(text.slice(lastIndex, start)));
      }

      const marker = `[[image:${index}]]`;
      const display = label || 'View image';
      const linkMeta = findLink(marker);
      const anchor = document.createElement('a');

      anchor.textContent = display;
      anchor.href = '#';
      anchor.dataset.overlayMarker = marker;
      anchor.dataset.overlayCommand = commandId;
      anchor.style.color = 'inherit';
      anchor.style.textDecoration = 'underline';
      anchor.style.cursor = 'pointer';
      anchor.style.fontWeight = 'bold';

      if (linkMeta) {
        anchor.addEventListener('click', (event) => {
          event.preventDefault();
          openMarker(marker);
        });

        anchor.addEventListener('mouseenter', (event) => {
          const rect = (event.target as HTMLAnchorElement).getBoundingClientRect();
          setHover({ x: rect.left, y: rect.top, alt: linkMeta.alt });
        });

        anchor.addEventListener('mouseleave', () => {
          setHover(null);
        });
      }

      fragment.appendChild(anchor);
      lastIndex = end;
    });

    if (lastIndex < text.length) {
      fragment.appendChild(document.createTextNode(text.slice(lastIndex)));
    }

    textNode.replaceWith(fragment);
  });
};
