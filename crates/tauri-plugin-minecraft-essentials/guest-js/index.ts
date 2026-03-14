import { invoke } from '@tauri-apps/api/core'

interface OAuthParams {
	client_id: string;
	client_secret: string;
	bedrock_rel: boolean;
	refresh_token?: string;
	scope?: string;
	port?: number;
}

interface CustomAuthData {
	access_token: string;
	uuid?: number;
	expires_in: string;
	xts_token?: string;
}

export async function oauth(params: OAuthParams): Promise<CustomAuthData> {
	return await invoke<CustomAuthData>('plugin:minecraft-essentials|oauth', { ...params });
}
