'use client';

import { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import Terminal from 'react-console-emulator';

import styles from '@/styles/TerminalConsole.module.css';
import welcomeMessageText from '@/pages/main.txt';
import aboutMeText from '@/pages/about_me.txt';
import cvText from '@/pages/cv.txt';
import { parseContent, ParsedLink } from '@/utils/contentParser';
import { HoverLinkInfo, transformMarkersToLinks } from '@/utils/linkOverlayBinder';
import OldComputerFilterDefs from '@/components/OldComputerFilterDefs';

type CommandResponse = string | string[] | Promise<string | string[]>;

type TerminalCommand = {
  description?: string;
  usage?: string;
  fn: (...args: string[]) => CommandResponse;
};

const TerminalConsole = () => {
  const welcomeParsed = useMemo(() => parseContent(welcomeMessageText), []);

  const [overlayImage, setOverlayImage] = useState<{ src: string; alt: string } | null>(null);
  const [overlayVisible, setOverlayVisible] = useState(false);
  const parsedLinksRef = useRef<Record<string, ParsedLink[]>>({
    welcome: welcomeParsed.links,
  });
  const overlayCleanupTimeout = useRef<number | null>(null);
  const [filterId, setFilterId] = useState('terminal-old-filter');

  const hideOverlay = useCallback(() => {
    setOverlayVisible(false);

    if (overlayCleanupTimeout.current) {
      window.clearTimeout(overlayCleanupTimeout.current);
    }

    overlayCleanupTimeout.current = window.setTimeout(() => {
      setOverlayImage(null);
      overlayCleanupTimeout.current = null;
    }, 280);
  }, []);

  useEffect(() => {
    if (!overlayVisible) {
      return undefined;
    }

    const handleKeydown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        event.preventDefault();
        hideOverlay();
      }
    };

    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  }, [overlayVisible, hideOverlay]);

  const findLink = useCallback(
    (commandId: string, marker: string): ParsedLink | undefined =>
      parsedLinksRef.current[commandId]?.find((link) => link.marker === marker),
    [],
  );

  const openMarker = useCallback(
    (commandId: string, marker: string) => {
      const link = findLink(commandId, marker);
      if (!link) {
        return;
      }

      if (overlayCleanupTimeout.current) {
        window.clearTimeout(overlayCleanupTimeout.current);
        overlayCleanupTimeout.current = null;
      }

      setOverlayImage({ src: link.src, alt: link.alt });
      setOverlayVisible(true);
    },
    [findLink],
  );

  useEffect(() => {
    if (overlayVisible && overlayImage) {
      setFilterId(`terminal-old-filter-${Date.now()}`);
    }
  }, [overlayVisible, overlayImage]);

  useEffect(() => () => {
    if (overlayCleanupTimeout.current) {
      window.clearTimeout(overlayCleanupTimeout.current);
    }
  }, []);

  const onOutputRender = useCallback(
    (commandId: string) => {
      const selector = "[name='react-console-emulator__content']";
      const container = document.querySelector(selector);

      if (!container) {
        return;
      }

      transformMarkersToLinks(
        container,
        commandId,
        (marker) => findLink(commandId, marker),
        (marker) => openMarker(commandId, marker),
        () => {},
      );
    },
    [findLink, openMarker],
  );

  const handleContent = useCallback(
    (id: string, raw: string) => {
      const parsed = parseContent(raw);
      parsedLinksRef.current = {
        ...parsedLinksRef.current,
        [id]: parsed.links,
      };

      requestAnimationFrame(() => onOutputRender(id));
      return parsed.text;
    },
    [onOutputRender],
  );

  useEffect(() => {
    requestAnimationFrame(() => onOutputRender('welcome'));
  }, [onOutputRender]);

  const commands = useMemo<Record<string, TerminalCommand>>(
    () => ({
      about: {
        description: 'About Nikolai Zhdanov',
        fn: () => handleContent('about', aboutMeText),
      },
      cv: {
        description: 'Resume',
        fn: () => handleContent('cv', cvText),
      },
    }),
    [handleContent],
  );

  return (
    <div className={styles.terminal}>
      <div className={styles.scanline}></div>
      <div className={styles.view}>
        <Terminal
          autoFocus
          commands={commands}
          promptLabel="guest@zhdanov.dev:~$"
          className={styles.terminal} /* This will be overridden by wrapper but needed */
          inputAreaClassName={styles.inputArea}
          contentClassName={styles.content}
          promptLabelClassName={styles.promptLabel}
          promptLabelStyle={{
            color: 'inherit',
            paddingTop: '0'
          }}
          inputStyle={{
            padding: '0',
            textShadow: 'inherit',
            fontSize: 'inherit',
            height: 'inherit'
          }}
          inputTextStyle={{
            padding: '0',
            fontSize: 'inherit',
            color: 'inherit',
            textShadow: 'inherit',
          }}
          contentStyle={{ 
            padding: '0 2vw',  
            color: 'inherit',
          }}
          welcomeMessage={welcomeParsed.text}
        />
      </div>
      {overlayImage && (
        <div
          className={`${styles.overlay} ${overlayVisible ? styles.overlayActive : styles.overlayInactive}`}
          onClick={hideOverlay}
        >
          <OldComputerFilterDefs id={filterId} key={filterId} />
          <div className={styles.overlayContent}>
            <img
              key={filterId}
              src={overlayImage.src}
              alt={overlayImage.alt}
              className={styles.overlayImage}
              style={{ filter: `url(#${filterId})` }}
            />
          </div>
          <div className={styles.overlayNoise}></div>
        </div>
      )}
    </div>
  );
};

export default TerminalConsole;
