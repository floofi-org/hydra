async function decodeStatusData() {
    let data = {
        global: null,
        ping: null,
        time: null,
        breakdown: {},
        services: [],
        notice: null
    };

    let buffer = await (await fetch(URL)).arrayBuffer();
    let binary = window.binary = new Uint8Array(buffer);
    console.log(buffer, binary);

    let status = binary[0];
    console.log(status);
    data.global = status;

    let ping = new Float32Array(buffer.slice(1, 5))[0];
    console.log(ping);
    data.ping = ping;

    let breakdownLength = binary[5];
    let pos = 6;

    for (let i = 0; i < breakdownLength; i++) {
        console.log("=> " + i);
        let date = new Uint32Array(buffer.slice(pos, pos + 4))[0];
        console.log(date);
        let statusesLength = binary[pos + 4];
        console.log(statusesLength);

        pos += 5;
        break;
    }

    console.log(data);
}
