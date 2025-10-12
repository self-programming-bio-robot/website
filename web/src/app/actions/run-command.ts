'use server';

type CommandIntent = {
  command: string;
  input: string;
};

const asciiHeader = [
  '+----------------------------------+',
  '|  zhdanov.dev :: proto console v0 |',
  '+----------------------------------+',
].join('\n');

export async function executeCommand({ command, input }: CommandIntent) {
  const normalized = command.trim().toLowerCase();
  const message = input.trim();

  switch (normalized) {
    case 'say': {
      if (!message) {
        return 'Команда `say` ожидает текст. Пример: say привет, мир!';
      }

      // Симулируем небольшую задержку от "сервера" для ощущения диалога.
      await new Promise((resolve) => setTimeout(resolve, 150));

      return `${asciiHeader}\n[server] Получено: ${message}`;
    }
    default: {
      return `Сервер пока не знает команду "${command}". Попробуйте 'say'.`;
    }
  }
}
