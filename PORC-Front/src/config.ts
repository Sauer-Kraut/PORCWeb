interface BackendConfig {
    url: string;
    port: string;
    discord_auth_url: string;
}

interface PorcConfigOptions {
    backend: BackendConfig;
}

class PorcConfig {
    backend: BackendConfig;

    constructor(options: PorcConfigOptions) {
        this.backend = options.backend;
    }

    getBackendUrl(): string {
        return `${this.backend.url}${this.backend.port ? `:${this.backend.port}` : ''}`;
    }

    getDiscordUrl(): string{
        return `${this.backend.discord_auth_url || ''}`;
    }
}

// Default configuration using environment variables
const defaultConfig: PorcConfigOptions = {
    backend: {
        url: import.meta.env.VITE_BACKEND_URL,
        port: import.meta.env.VITE_BACKEND_PORT,
        discord_auth_url: import.meta.env.VITE_BACKEND_DISCORD_AUTH_URL,
    },
};

const config = new PorcConfig(defaultConfig);
export default config;
