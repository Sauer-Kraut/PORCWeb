interface BackendConfig {
    url: string;
    port: string;
    discord_auth_url: string;
}

interface PorcConfigOptions {
    backend: BackendConfig;
    debug: boolean;
}

class PorcConfig {
    backend: BackendConfig;
    debug: boolean;

    constructor(options: PorcConfigOptions) {
        this.backend = options.backend;
        this.debug = options.debug;
    }

    getBackendUrl(): string {
        return `${this.backend.url}${this.backend.port ? `:${this.backend.port}` : ''}`;
    }

    getDiscordUrl(): string{
        return `${this.backend.discord_auth_url || ''}`;
    }

    getDebug(): boolean {
        return this.debug;
    }
}

// Default configuration using environment variables
const defaultConfig: PorcConfigOptions = {
    backend: {
        url: import.meta.env.VITE_BACKEND_URL,
        port: import.meta.env.VITE_BACKEND_PORT,
        discord_auth_url: import.meta.env.VITE_BACKEND_DISCORD_AUTH_URL,
    },
    debug: import.meta.env.VITE_DEBUG === 'true',
};

const config = new PorcConfig(defaultConfig);
export default config;
