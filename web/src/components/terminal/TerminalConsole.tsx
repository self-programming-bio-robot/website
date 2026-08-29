'use client';

import { FormEvent, KeyboardEvent, useEffect, useMemo, useRef, useState } from 'react';

import { mainContent } from '@/content/main';

import { commandRegistry } from './commands';
import MediaDialog from './MediaDialog';
import styles from './TerminalConsole.module.css';
import TerminalOutput from './TerminalOutput';
import type {
  CommandRegistry,
  MediaContent,
  TerminalBlock,
  TerminalEntry,
} from './terminal.types';
import {
  formatSuggestionWindow,
  getCommandSuggestions,
  parseCommand,
} from './terminal.utils';

const PROMPT = 'guest@zhdanov.dev:~$';
type TerminalConsoleProps = {
  commands?: CommandRegistry;
  initialOutput?: readonly TerminalBlock[];
};

export default function TerminalConsole({
  commands = commandRegistry,
  initialOutput = mainContent,
}: TerminalConsoleProps = {}) {
  const [input, setInput] = useState('');
  const [entries, setEntries] = useState<readonly TerminalEntry[]>(() => [
    { id: 0, command: null, output: initialOutput },
  ]);
  const [commandHistory, setCommandHistory] = useState<readonly string[]>([]);
  const [historyIndex, setHistoryIndex] = useState<number | null>(null);
  const [suggestionsDismissed, setSuggestionsDismissed] = useState(false);
  const [activeMedia, setActiveMedia] = useState<MediaContent | null>(null);
  const [isExecuting, setIsExecuting] = useState(false);
  const entryIdRef = useRef(1);
  const historyDraftRef = useRef('');
  const inputRef = useRef<HTMLInputElement>(null);
  const viewportRef = useRef<HTMLDivElement>(null);
  const mediaTriggerRef = useRef<HTMLButtonElement | null>(null);
  const executionLockRef = useRef(false);

  const suggestions = useMemo(
    () => suggestionsDismissed ? [] : getCommandSuggestions(input, commands),
    [commands, input, suggestionsDismissed],
  );

  useEffect(() => {
    const viewport = viewportRef.current;
    if (viewport) {
      viewport.scrollTop = viewport.scrollHeight;
    }
  }, [entries, isExecuting]);

  const appendEntry = (command: string, output: readonly TerminalBlock[]) => {
    const entry: TerminalEntry = {
      id: entryIdRef.current,
      command,
      output,
    };
    entryIdRef.current += 1;
    setEntries((current) => [...current, entry]);
  };

  const executeInput = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (executionLockRef.current) {
      return;
    }

    const rawCommand = input.trim();
    const parsed = parseCommand(rawCommand);
    if (!parsed) {
      return;
    }

    executionLockRef.current = true;
    setIsExecuting(true);
    setInput('');
    setHistoryIndex(null);
    setSuggestionsDismissed(false);
    setCommandHistory((current) => [...current, rawCommand]);

    try {
      const definition = commands[parsed.name];

      if (!definition) {
        appendEntry(rawCommand, [errorBlock(
          `Command '${parsed.name}' not found. Type 'help' to list commands.`,
        )]);
        return;
      }

      const result = await definition.execute(parsed.args);
      if (result.type === 'clear') {
        setEntries([]);
      } else {
        appendEntry(rawCommand, result.blocks);
      }
    } catch (error) {
      console.error('Terminal command failed:', error);
      appendEntry(rawCommand, [errorBlock('The command could not be completed.')]);
    } finally {
      executionLockRef.current = false;
      setIsExecuting(false);
      requestAnimationFrame(() => inputRef.current?.focus());
    }
  };

  const handleInputKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
    if (event.key === 'Tab' && !event.shiftKey && suggestions.length > 0) {
      event.preventDefault();
      setInput(suggestions[0]);
      setSuggestionsDismissed(true);
      return;
    }

    if (event.key === 'Tab') {
      const outputControls = viewportRef.current?.querySelectorAll<HTMLElement>(
        '[role="log"] a[href], [role="log"] button:not([disabled])',
      );

      if (outputControls && outputControls.length > 0) {
        event.preventDefault();
        const targetIndex = event.shiftKey ? outputControls.length - 1 : 0;
        outputControls[targetIndex].focus();
        return;
      }
    }

    if (event.key === 'Escape' && suggestions.length > 0) {
      event.preventDefault();
      setSuggestionsDismissed(true);
      return;
    }

    if (event.key === 'ArrowUp') {
      if (commandHistory.length === 0) {
        return;
      }

      event.preventDefault();
      const nextIndex = historyIndex === null
        ? commandHistory.length - 1
        : Math.max(0, historyIndex - 1);

      if (historyIndex === null) {
        historyDraftRef.current = input;
      }

      setHistoryIndex(nextIndex);
      setInput(commandHistory[nextIndex]);
      setSuggestionsDismissed(true);
      return;
    }

    if (event.key === 'ArrowDown' && historyIndex !== null) {
      event.preventDefault();

      if (historyIndex < commandHistory.length - 1) {
        const nextIndex = historyIndex + 1;
        setHistoryIndex(nextIndex);
        setInput(commandHistory[nextIndex]);
      } else {
        setHistoryIndex(null);
        setInput(historyDraftRef.current);
      }

      setSuggestionsDismissed(true);
    }
  };

  const closeMedia = () => {
    setActiveMedia(null);
    requestAnimationFrame(() => {
      mediaTriggerRef.current?.focus();
      mediaTriggerRef.current = null;
    });
  };

  return (
    <div
      className={styles.terminal}
      onClick={(event) => {
        const target = event.target as HTMLElement;
        if (!target.closest('a, button, input, dialog')) {
          inputRef.current?.focus();
        }
      }}
    >
      <div aria-hidden className={styles.scanline} />
      <div className={styles.viewport} ref={viewportRef}>
        <div
          aria-live="polite"
          aria-relevant="additions"
          className={styles.outputLog}
          role="log"
        >
          {entries.map((entry) => (
            <div className={styles.entry} key={entry.id}>
              {entry.command !== null && (
                <div className={styles.commandEcho}>
                  <span className={styles.prompt}>{PROMPT}</span>
                  <span>{entry.command}</span>
                </div>
              )}
              <TerminalOutput
                blocks={entry.output}
                onOpenMedia={(media, trigger) => {
                  mediaTriggerRef.current = trigger;
                  setActiveMedia(media);
                }}
              />
            </div>
          ))}
        </div>

        <form className={styles.inputForm} onSubmit={executeInput}>
          <label className={styles.visuallyHidden} htmlFor="terminal-command">
            Terminal command
          </label>
          <span aria-hidden className={styles.prompt}>{PROMPT}</span>
          <div className={styles.inputField}>
            <div
              aria-live="polite"
              className={styles.suggestions}
              id="terminal-suggestions"
              role="status"
            >
              {suggestions.length > 0 && (
                <>
                  <span className={styles.visuallyHidden}>
                    Available completions: {suggestions.join(', ')}
                  </span>
                  <pre aria-hidden className={styles.suggestionWindow}>
                    {formatSuggestionWindow(suggestions)}
                  </pre>
                </>
              )}
            </div>
            <input
              aria-describedby="terminal-suggestions"
              autoCapitalize="none"
              autoComplete="off"
              autoCorrect="off"
              autoFocus
              className={styles.commandInput}
              disabled={isExecuting}
              id="terminal-command"
              onChange={(event) => {
                setInput(event.target.value);
                setHistoryIndex(null);
                setSuggestionsDismissed(false);
              }}
              onKeyDown={handleInputKeyDown}
              ref={inputRef}
              spellCheck={false}
              value={input}
            />
          </div>
        </form>

      </div>

      <MediaDialog media={activeMedia} onClose={closeMedia} />
    </div>
  );
}

function errorBlock(message: string): TerminalBlock {
  return {
    type: 'paragraph',
    content: [message],
    color: 'light-red',
  };
}
