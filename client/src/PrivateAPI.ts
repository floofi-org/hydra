import Log from "./Log.js";
import Config from "./Config.js";

enum ServiceStatus {
    Online = 0,
    Unstable = 1,
    Offline = 2,
    Maintenance = 3
}

enum ServiceCategory {
    Websites = "websites",
    Servers = "servers",
    Network = "network"
}

enum ServiceHostingProvider {
    EquestriaDev = "equestriadev",
    Scaleway = "scaleway",
    Ovh = "ovh",
    Vercel = "vercel",
    GitBook = "gitbook"
}

interface IPrivateAPI {
    global: ServiceStatus,
    ping: number,
    time: TDateTime,
    breakdown: TMiniHistory,
    services: IClientService[],
    notice?: IOutageConfig
}

interface IOutageConfig {
    enabled: true,
    title: string,
    description: string,
    link: string
}

type TDateTime = `${number}-${number}-${number}T${number}:${number}:${number}${`.${number}` | null}Z`;
type TMiniHistory = object;

interface IClientService {
    id: string,
    label: string,
    ping: number,
    status: ServiceStatus,
    type: ServiceCategory,
    hosting: ServiceHostingProvider
}

export default class PrivateAPI {
    static async fetch(url: string): Promise<IPrivateAPI> {
        Log.info("Downloading data: " + Config.url);

        try {
            return await (await fetch(url)).json() as IPrivateAPI;
        } catch (e: any) {
            Log.error("Failed to download data: " + e.name + ": " + e.message + "\n" + e.stack);
            throw new Error();
        }
    }
}
