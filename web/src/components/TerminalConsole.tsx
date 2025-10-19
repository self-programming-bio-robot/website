'use client';

import { useMemo } from 'react';
import Terminal from 'react-console-emulator';

import styles from '@/styles/TerminalConsole.module.css';
import welcomeMessageText from '@/pages/main.txt';
import aboutMeText from '@/pages/about_me.txt';
import cvText from '@/pages/cv.txt';

type CommandResponse = string | string[] | Promise<string | string[]>;

type TerminalCommand = {
  description?: string;
  usage?: string;
  fn: (...args: string[]) => CommandResponse;
};


const TerminalConsole = () => {
  const commands = useMemo<Record<string, TerminalCommand>>(
    () => ({
      about: {
        description: 'About Nikolai Zhdanov',
        fn: () => aboutMeText,
      },
      cv: {
        description: 'Resume',
        fn: () => cvText,
      },
    }),
    [],
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
          welcomeMessage={welcomeMessageText}
        />
      </div>
    </div>
  );
};

export default TerminalConsole;
