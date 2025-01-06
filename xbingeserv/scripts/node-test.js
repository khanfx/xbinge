const http = require('http');

// ----------------------------------------------------------------------------

async function sendPostRequestAsync(path, data) {
    const options = {
        hostname: 'localhost',
        port: 3000,
        path: path,
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Content-Length': data.length
        }
    };

    return new Promise((resolve, reject) => {
        const req = http.request(options, (res) => {
            let responseBody = '';

            res.on('data', (chunk) => {
                responseBody += chunk;
            });

            res.on('end', () => {
                resolve({
                    statusCode: res.statusCode,
                    body: responseBody
                });
            });
        });

        req.on('error', (error) => {
            reject(error);
        });

        req.write(data);
        req.end();
    });
}

function doPOST(path, data) {
    (async () => {
        try {
            const response = await sendPostRequestAsync(path, data);

            console.log('Response status:', response.statusCode);
            console.log('Response body:', response.body);
        } catch (error) {
            console.error('Error making POST request:', error);
        }
    })();
}

// ----------------------------------------------------------------------------
// TEST SCRIPT
// ----------------------------------------------------------------------------

const json = JSON.stringify({
    name: 'Hello World'
});

doPOST('/items', json);
