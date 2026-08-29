import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it } from 'vitest';

import type { CommandRegistry } from './terminal.types';
import TerminalConsole from './TerminalConsole';

describe('TerminalConsole', () => {
  it('completes a suggestion with Tab and executes the command with Enter', async () => {
    const user = userEvent.setup();
    render(<TerminalConsole />);
    const input = screen.getByRole('textbox', { name: 'Terminal command' });

    await user.type(input, 'he');
    expect(screen.getByRole('status')).toHaveTextContent('help');
    expect(screen.getByRole('status')).not.toHaveTextContent('suggestions:');

    await user.keyboard('{Tab}');
    expect(input).toHaveValue('help');

    await user.keyboard('{Enter}');
    expect(await screen.findByText('List available commands')).toBeInTheDocument();
    expect(input).toHaveFocus();
  });

  it('navigates command history and restores the current draft', async () => {
    const user = userEvent.setup();
    render(<TerminalConsole />);
    const input = screen.getByRole('textbox', { name: 'Terminal command' });

    await user.type(input, 'about{Enter}');
    await waitFor(() => expect(input).not.toBeDisabled());
    await user.type(input, 'cv{Enter}');
    await waitFor(() => expect(input).not.toBeDisabled());
    await user.type(input, 'draft');

    await user.keyboard('{ArrowUp}');
    expect(input).toHaveValue('cv');
    await user.keyboard('{ArrowUp}');
    expect(input).toHaveValue('about');
    await user.keyboard('{ArrowDown}');
    expect(input).toHaveValue('cv');
    await user.keyboard('{ArrowDown}');
    expect(input).toHaveValue('draft');
  });

  it('renders unknown-command errors and clears output without clearing history', async () => {
    const user = userEvent.setup();
    render(<TerminalConsole />);
    const input = screen.getByRole('textbox', { name: 'Terminal command' });

    await user.type(input, 'missing{Enter}');
    const error = await screen.findByText(/Command 'missing' not found/);
    expect(error).toHaveStyle({ color: 'var(--terminal-color-light-red)' });

    await user.type(input, 'clear{Enter}');
    await waitFor(() => expect(screen.queryByText(/Welcome to my personal website/)).not.toBeInTheDocument());

    await user.keyboard('{ArrowUp}');
    expect(input).toHaveValue('clear');
    await user.keyboard('{ArrowUp}');
    expect(input).toHaveValue('missing');
  });

  it('opens an image dialog and restores focus after Escape', async () => {
    const user = userEvent.setup();
    render(<TerminalConsole />);
    const input = screen.getByRole('textbox', { name: 'Terminal command' });

    await user.type(input, 'about{Enter}');
    const photoButton = await screen.findByRole('button', { name: 'Photo' });
    await user.click(photoButton);

    const dialog = screen.getByRole('dialog', {
      name: 'An early photo from my first years learning programming',
    });
    expect(screen.getByRole('img')).toHaveAttribute('src', '/early_photo.jpg');

    fireEvent(dialog, new Event('cancel', { cancelable: true }));
    await waitFor(() => expect(dialog).not.toBeInTheDocument());
    expect(photoButton).toHaveFocus();
  });

  it('moves from the command input to output controls with Tab', async () => {
    const user = userEvent.setup();
    render(<TerminalConsole />);
    const input = screen.getByRole('textbox', { name: 'Terminal command' });

    await user.type(input, 'about{Enter}');
    const photoButton = await screen.findByRole('button', { name: 'Photo' });
    const emailLink = screen.getByRole('link', { name: 'email me' });
    expect(input).toHaveFocus();

    await user.keyboard('{Tab}');
    expect(emailLink).toHaveFocus();
    await user.keyboard('{Tab}');
    expect(photoButton).toHaveFocus();
  });

  it('accepts a custom command registry and initial output', async () => {
    const user = userEvent.setup();
    const commands: CommandRegistry = {
      ping: {
        description: 'Custom test command',
        execute: () => ({
          type: 'output',
          blocks: [{ type: 'paragraph', content: ['pong'] }],
        }),
      },
    };

    render(
      <TerminalConsole
        commands={commands}
        initialOutput={[{ type: 'paragraph', content: ['Custom terminal'] }]}
      />,
    );
    const input = screen.getByRole('textbox', { name: 'Terminal command' });

    expect(screen.getByText('Custom terminal')).toBeInTheDocument();
    await user.type(input, 'pi{Tab}{Enter}');
    expect(await screen.findByText('pong')).toBeInTheDocument();
  });
});
