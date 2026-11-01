import config from "@/config";

const debugWithTimestamp = (...args: unknown[]) => {
    if (config.getDebug()) {
        console.debug(new Date().toISOString(), ...args);
    }
};

export default debugWithTimestamp;