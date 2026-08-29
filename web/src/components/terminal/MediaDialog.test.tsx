import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';

import MediaDialog from './MediaDialog';

describe('MediaDialog', () => {
  it('renders a video with controls and requests close on cancel', () => {
    const onClose = vi.fn();
    render(
      <MediaDialog
        media={{
          type: 'media',
          mediaType: 'video',
          label: 'Demo',
          src: '/demo.mp4',
          description: 'Terminal component demo',
        }}
        onClose={onClose}
      />,
    );

    const dialog = screen.getByRole('dialog', { name: 'Terminal component demo' });
    const video = dialog.querySelector('video');
    expect(video).not.toBeNull();
    expect(video).toHaveAttribute('controls');
    expect(video).toHaveAttribute('src', '/demo.mp4');

    fireEvent(dialog, new Event('cancel', { cancelable: true }));
    expect(onClose).toHaveBeenCalledOnce();
  });
});
