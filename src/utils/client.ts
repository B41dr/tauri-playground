import { invoke } from '@tauri-apps/api/core';

export const useHello = () => {
    return invoke('hello', { name: 'World' });
};

export const useGetList = () => {
    return invoke('get_list', { name: 'World' });
};
