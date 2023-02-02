const axios = require('axios');
const fs = require('fs');
const YAML = require('yaml');
const config = YAML.parse(fs.readFileSync("./config.yaml").toString());

(async () => {
    await axios.post(config['ntfy']['server'] + "/" + config['ntfy']['topic'], "This is a test, ignore me", {
        headers: {
            Title: 'Test message',
            Priority: '2',
            Tags: 'status'
        },
        auth: {
            username: config['ntfy']['user'],
            password: config['ntfy']['password']
        }
    })
})();