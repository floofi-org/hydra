module.exports = () => {
    const axios = require('axios');
    const fs = require('fs');
    const YAML = require('yaml');
    const config = YAML.parse(fs.readFileSync("./config.yaml").toString());

    const last = parseInt(fs.readFileSync("./last.txt").toString().trim());
    const secondLast = fs.existsSync("./second-last.txt") ? parseInt(fs.readFileSync("./second-last.txt").toString().trim()) : -1;
    const current = JSON.parse(fs.readFileSync("./output.json").toString()).total;
    const outages = JSON.parse(fs.readFileSync("./git/status.json").toString()).outages;

    function formatPonypush(text) {
        return "Update to Ponypush 3.1.0 or later — ($PA1$$" + Buffer.from(text).toString("base64") + "$$)";
    }

    function generateOutageList() {
        if (outages.length === 0) return "Something is";
        if (outages.length === 1) return outages[0][1] + " is";

        let names = outages.map(i => i[1]);
        let namesButOne = names.slice(0, -1);
        let lastName = names[names.length - 1];

        return namesButOne.join(", ") + " and " + lastName + " are";
    }

    console.log(secondLast, last, current);

    if (last === current && last !== secondLast) {
        console.log("Dispatching notification");

        switch (current) {
            case 0:
                if (secondLast === 0) break;

                (async () => {
                    await axios.post(config['ntfy']['server'], {
                        topic: config['ntfy']['topic'],
                        message: formatPonypush('All of the services are now working as expected.'),
                        title: formatPonypush("🟢 Everything is up"),
                        priority: 2,
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
                                label: "Open Console",
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
                    await axios.post(config['ntfy']['server'], {
                        topic: config['ntfy']['public'],
                        message: formatPonypush('All of the services are now working as expected.'),
                        title: formatPonypush("🟢 Everything is up"),
                        priority: 2,
                        tags: ['status'],
                        click: "https://status.equestria.dev",
                        actions: [
                            {
                                action: "view",
                                label: "View status page",
                                url: "https://status.equestria.dev",
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
                        message: formatPonypush(generateOutageList() + ' experiencing temporary performance degradation, this is most likely safe.'),
                        title: formatPonypush("🟡 Something is slower than expected"),
                        priority: 2,
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
                                label: "Open Console",
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
                    await axios.post(config['ntfy']['server'], {
                        topic: config['ntfy']['public'],
                        message: formatPonypush(generateOutageList() + ' experiencing temporary performance degradation, this is most likely safe.'),
                        title: formatPonypush("🟡 Something is slower than expected"),
                        priority: 2,
                        tags: ['status'],
                        click: "https://status.equestria.dev",
                        actions: [
                            {
                                action: "view",
                                label: "View status page",
                                url: "https://status.equestria.dev",
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
                        message: formatPonypush(generateOutageList() + ' experiencing a major outage, investigation is needed.'),
                        title: formatPonypush("🔴 Something is down"),
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
                                label: "Open Console",
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
                    await axios.post(config['ntfy']['server'], {
                        topic: config['ntfy']['public'],
                        message: formatPonypush(generateOutageList() + ' experiencing a major outage, investigation is needed.'),
                        title: formatPonypush("🔴 Something is down"),
                        priority: 3,
                        tags: ['status'],
                        click: "https://status.equestria.dev",
                        actions: [
                            {
                                action: "view",
                                label: "View status page",
                                url: "https://status.equestria.dev",
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
    }

    fs.writeFileSync("./second-last.txt", last.toString());
    fs.writeFileSync("./last.txt", current.toString());
}
