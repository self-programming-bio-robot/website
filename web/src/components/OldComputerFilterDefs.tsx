import styles from '@/styles/TerminalConsole.module.css';

type OldComputerFilterDefsProps = {
  id: string;
};

const OldComputerFilterDefs = ({ id }: OldComputerFilterDefsProps) => (
  <svg className={styles.filterDefs} aria-hidden focusable="false">
    <defs>
      <filter id={id} x="0%" y="0%" width="100%" height="100%" colorInterpolationFilters="sRGB">
        <feTurbulence type="fractalNoise" baseFrequency="0.9" numOctaves="1" seed="3" result="noise">
          <animate attributeName="baseFrequency" values="0.9;0.45;0.12;0.02" keyTimes="0;0.5;0.8;1" dur="1s" fill="freeze" />
        </feTurbulence>
        <feDisplacementMap in="SourceGraphic" in2="noise" scale="60" result="distorted">
          <animate attributeName="scale" values="60;28;10;0" keyTimes="0;0.45;0.85;1" dur="1s" fill="freeze" />
        </feDisplacementMap>
        <feComponentTransfer in="distorted" result="levels">
          <feFuncR type="discrete" tableValues="0 0.35 0.7 1" />
          <feFuncG type="discrete" tableValues="0 0.35 0.7 1" />
          <feFuncB type="discrete" tableValues="0 0.35 0.7 1" />
        </feComponentTransfer>
        <feColorMatrix in="levels" type="saturate" values="0" result="desaturated">
          <animate attributeName="values" values="0;0.5;1" keyTimes="0;0.6;1" dur="1s" fill="freeze" />
        </feColorMatrix>
        <feComposite in="noise" in2="desaturated" operator="arithmetic" k1="0" k2="0.95" k3="0.25" k4="0" result="mixed">
          <animate attributeName="k2" values="0.65;0.35;0" keyTimes="0;0.6;1" dur="1s" fill="freeze" />
          <animate attributeName="k3" values="0.25;0.18;0" keyTimes="0;0.6;1" dur="1s" fill="freeze" />
        </feComposite>
        <feBlend in="desaturated" in2="mixed" mode="screen" result="screened" />
        <feComposite in="screened" in2="SourceGraphic" operator="arithmetic" k1="0" k2="0.8" k3="0.2" k4="0">
          <animate attributeName="k2" values="0.8;0.45;0" keyTimes="0;0.6;1" dur="1s" fill="freeze" />
          <animate attributeName="k3" values="0.2;0.6;1" keyTimes="0;0.6;1" dur="1s" fill="freeze" />
        </feComposite>
      </filter>
    </defs>
  </svg>
);

export default OldComputerFilterDefs;
