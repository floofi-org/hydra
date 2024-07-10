class Cursor {
    position;
    inner;
    innerArray;

    constructor(inner) {
        this.inner = inner;
        this.innerArray = new Uint8Array(inner);
        this.position = 0;
    }

    #readBytes(n) {
        this.position += n;
        return this.inner.slice(this.position - n, this.position)
    }

    #read4Bytes(T) {
        return new T(this.#readBytes(4))[0];
    }

    #read8Bytes(T) {
        return new T(this.#readBytes(8))[0];
    }

    readUint8() {
        return new Uint8Array(this.#readBytes(1))[0];
    }

    readFloat32() {
        return this.#read4Bytes(Float32Array);
    }

    readUint32() {
        return this.#read4Bytes(Uint32Array);
    }

    readUint64() {
        return this.#read8Bytes(BigUint64Array);
    }

    readString() {
        let stringLength = this.readUint8();
        let stringBytes = this.#readBytes(stringLength);
        return new TextDecoder().decode(stringBytes);
    }

    readAsJson() {
        let data = this.inner.slice(this.position);
        let text = new TextDecoder().decode(data);

        this.position = this.inner.length;
        return JSON.parse(text);
    }
}

async function decodeStatusData() {
    let data = {
        global: null,
        ping: null,
        time: null,
        breakdown: {},
        services: [],
        notice: null
    };

    let cursor = new Cursor(await (await fetch(URL)).arrayBuffer());

    data.global = cursor.readUint8();
    data.ping = cursor.readFloat32();

    let breakdownLength = cursor.readUint8();

    for (let i = 0; i < breakdownLength; i++) {
        let date = cursor.readUint32();
        let dateString = new Date(date * 86400000).toISOString().split("T")[0];

        data.breakdown[dateString] = new Array(4).fill(_ => null).map(_ => cursor.readFloat32());
    }

    let serviceLength = cursor.readUint8();

    for (let i = 0; i < serviceLength; i++) {
        let id = cursor.readString();
        let label = cursor.readString();
        let ping = cursor.readUint32();
        let status = cursor.readUint8();
        let category = CATEGORIES[cursor.readUint8()] ?? "websites";
        let hostingProvider = HOSTING_PROVIDERS[cursor.readUint8()] ?? "equestriadev";

        let service = {
            id,
            label,
            ping,
            status,
            type: category,
            hosting: hostingProvider
        }

        data.services.push(service);
    }

    let timestamp = cursor.readUint64();
    data.time = new Date(parseInt(timestamp) * 1000).toISOString();

    if (cursor.readUint8() === 1) {
        data.notice = cursor.readAsJson();
    }

    return data;
}
