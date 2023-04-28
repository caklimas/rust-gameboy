import { createStore } from 'redux'
import { rootReducer } from './reducers';

export function getStore() {
    const store = createStore(rootReducer);
    return store;
}