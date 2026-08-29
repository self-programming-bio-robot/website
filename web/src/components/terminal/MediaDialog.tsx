'use client';

import { useEffect, useId, useRef } from 'react';

import styles from './TerminalConsole.module.css';
import OldComputerFilterDefs from './OldComputerFilterDefs';
import type { MediaContent } from './terminal.types';

type MediaDialogProps = {
  media: MediaContent | null;
  onClose: () => void;
};

export default function MediaDialog({ media, onClose }: MediaDialogProps) {
  const dialogRef = useRef<HTMLDialogElement>(null);
  const videoRef = useRef<HTMLVideoElement>(null);
  const titleId = useId();
  const filterId = `${titleId.replace(/[^a-zA-Z0-9_-]/g, '')}-old-computer-filter`;

  useEffect(() => {
    const dialog = dialogRef.current;
    const video = videoRef.current;
    if (!dialog || !media) {
      return;
    }

    if (typeof dialog.showModal === 'function') {
      if (!dialog.open) {
        dialog.showModal();
      }
    } else {
      dialog.setAttribute('open', '');
    }

    return () => {
      video?.pause();
      if (dialog.open && typeof dialog.close === 'function') {
        dialog.close();
      } else {
        dialog.removeAttribute('open');
      }
    };
  }, [media]);

  if (!media) {
    return null;
  }

  const requestClose = () => {
    videoRef.current?.pause();
    onClose();
  };

  return (
    <dialog
      aria-labelledby={titleId}
      className={styles.mediaDialog}
      onCancel={(event) => {
        event.preventDefault();
        requestClose();
      }}
      onClick={(event) => {
        if (event.target === event.currentTarget) {
          requestClose();
        }
      }}
      ref={dialogRef}
    >
      <OldComputerFilterDefs id={filterId} />
      <div
        className={styles.dialogContent}
        onClick={(event) => {
          if (event.target === event.currentTarget) {
            requestClose();
          }
        }}
      >
        <h2 className={styles.visuallyHidden} id={titleId}>{media.description}</h2>
        <button
          aria-label="Close media"
          className={styles.dialogClose}
          onClick={requestClose}
          type="button"
        >
          ×
        </button>
        {media.mediaType === 'image' ? (
          // The content model does not require image dimensions, so next/image is not appropriate here.
          // eslint-disable-next-line @next/next/no-img-element
          <img
            alt={media.description}
            className={styles.dialogMedia}
            src={media.src}
            style={{ filter: `url(#${filterId})` }}
          />
        ) : (
          <video
            aria-label={media.description}
            className={styles.dialogMedia}
            controls
            preload="metadata"
            ref={videoRef}
            src={media.src}
            style={{ filter: `url(#${filterId})` }}
          />
        )}
        <div aria-hidden className={styles.overlayNoise} />
      </div>
    </dialog>
  );
}
