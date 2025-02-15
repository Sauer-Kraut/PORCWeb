interface BackendConfig {
    url: string;
    port: string;
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
}

// Default configuration using environment variables
const defaultConfig: PorcConfigOptions = {
    backend: {
        url: import.meta.env.VITE_BACKEND_URL,
        port: import.meta.env.VITE_BACKEND_PORT,
    },
};

const config = new PorcConfig(defaultConfig);
export default config;
