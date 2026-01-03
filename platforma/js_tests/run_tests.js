const { exec } = require('child_process');
const fs = require('fs');

const resultsFile = 'results_js.txt';

exec('npx jest --colors=never', (error, stdout, stderr) => {
    let output = '';
    
    if (error) {
        output += `Error: ${error.message}\n`;
    }
    if (stderr) {
        output += `Stderr: ${stderr}\n`;
    }

    output += stdout;

    fs.writeFileSync(resultsFile, output);
    console.log(`Testet u ekzekutuan. Rezultatet u ruajtën në ${resultsFile}`);
});
