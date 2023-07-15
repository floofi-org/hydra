module.exports = () => {
    const axios = require('axios');
    const fs = require('fs');
    const YAML = require('yaml');
    const config = YAML.parse(fs.readFileSync("./config.yaml").toString());

    const last = parseInt(fs.readFileSync("./last.txt").toString().trim());
    const current = JSON.parse(fs.readFileSync("./output.json").toString()).total;

    function formatPonypush(text) {
        return "Update to Ponypush 3.1.0 or later — ($PA1$$" + Buffer.from(text).toString("base64") + "$$)";
    }

    console.log(last, current);

    if (last !== current) {
        switch (current) {
            case 0:
                (async () => {
                    await axios.post(config['ntfy']['server'], {
                        topic: config['ntfy']['topic'],
                        message: formatPonypush('All of the services are now working as expected.'),
                        title: formatPonypush("🟢 Everything is up"),
                        priority: 3,
                        tags: ['status'],
                        click: "https://status.equestria.dev",
                        actions: [
                            {
                                action: "view",
                                label: "View status page",
                                url: "https://status.equestria.dev",
                                clear: true
                            },
                            {
                                action: "view",
                                label: "Open Proxmox",
                                url: "https://admin.equestria.dev",
                                clear: true
                            }
                        ]
                    }, {
                        auth: {
                            username: config['ntfy']['user'],
                            password: config['ntfy']['password']
                        }
                    })
                })();
                break;
            case 1:
                (async () => {
                    await axios.post(config['ntfy']['server'], {
                        topic: config['ntfy']['topic'],
                        message: formatPonypush('This service is experiencing temporary performance degradation, this is most likely safe.'),
                        title: formatPonypush("🟡 [?] is slower than expected"),
                        priority: 3,
                        tags: ['status'],
                        click: "https://status.equestria.dev",
                        actions: [
                            {
                                action: "view",
                                label: "View status page",
                                url: "https://status.equestria.dev",
                                clear: true
                            },
                            {
                                action: "view",
                                label: "Open Proxmox",
                                url: "https://admin.equestria.dev",
                                clear: true
                            }
                        ]
                    }, {
                        auth: {
                            username: config['ntfy']['user'],
                            password: config['ntfy']['password']
                        }
                    })
                })();
                break;
            case 2:
                (async () => {
                    await axios.post(config['ntfy']['server'], {
                        topic: config['ntfy']['topic'],
                        message: formatPonypush('This service is experiencing a major outage and automatic remediation has failed or is impossible, investigation is needed.'),
                        title: formatPonypush("🔴 [?] is down"),
                        priority: 3,
                        tags: ['status'],
                        click: "https://status.equestria.dev",
                        actions: [
                            {
                                action: "view",
                                label: "View status page",
                                url: "https://status.equestria.dev",
                                clear: true
                            },
                            {
                                action: "view",
                                label: "Open Proxmox",
                                url: "https://admin.equestria.dev",
                                clear: true
                            }
                        ]
                    }, {
                        auth: {
                            username: config['ntfy']['user'],
                            password: config['ntfy']['password']
                        }
                    })
                })();
                break;
        }

        fs.writeFileSync("./last.txt", current.toString());
    }
}