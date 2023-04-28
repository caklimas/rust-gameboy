import React from 'react';
import { createRoot } from 'react-dom/client';
import { Provider } from 'react-redux';
import { getStore } from './redux/store';
import App from './components/App/App';
import './index.scss';
import 'bootstrap/dist/css/bootstrap.min.css';

const root = createRoot(document.getElementById('root'));
root.render(
  <React.StrictMode>
    <Provider store={getStore()}>
      <App />
    </Provider>
  </React.StrictMode>
);