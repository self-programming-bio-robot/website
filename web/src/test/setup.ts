import '@testing-library/jest-dom/vitest';
import { cleanup } from '@testing-library/react';
import { afterEach } from 'vitest';

afterEach(() => cleanup());

if (!HTMLDialogElement.prototype.showModal) {
  HTMLDialogElement.prototype.showModal = function showModal() {
    this.setAttribute('open', '');
  };
}

if (!HTMLDialogElement.prototype.close) {
  HTMLDialogElement.prototype.close = function close() {
    this.removeAttribute('open');
  };
}

Object.defineProperty(HTMLMediaElement.prototype, 'pause', {
  configurable: true,
  value: () => undefined,
});

window.requestAnimationFrame = (callback: FrameRequestCallback) => {
  callback(0);
  return 0;
};
