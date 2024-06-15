export default class Log {
    static info(message: string) {
        console.log(`%c[${new Date().toISOString()}] %cINFO  %c${message}`, "color: #6f737a", "color: #3574f0", "");
    }

    static warn(message: string) {
        console.log(`%c[${new Date().toISOString()}] %cWARN  %c${message}`, "color: #6f737a", "color: #ffaf0f", "");
    }

    static error(message: string) {
        console.log(`%c[${new Date().toISOString()}] %cERROR %c${message}`, "color: #6f737a", "color: #db3b4b", "");
    }

    static debug(message: string) {
        console.log(`%c[${new Date().toISOString()}] %cDEBUG %c${message}`, "color: #6f737a", "color: #208a3c", "");
    }
}
