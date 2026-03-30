import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { BrowserRouter } from 'react-router';
import { createTheme, MantineProvider } from '@mantine/core';
import '@mantine/core/styles.css';

const queryClient = new QueryClient();

const theme = createTheme({});

const rootElement = document.getElementById('root');
if (!rootElement) throw new Error('root element not found');

createRoot(rootElement).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        <MantineProvider theme={theme} defaultColorScheme="auto">
          <App />
        </MantineProvider>
      </BrowserRouter>
    </QueryClientProvider>
  </StrictMode>,
);
