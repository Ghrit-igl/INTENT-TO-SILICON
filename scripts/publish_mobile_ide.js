const fs = require('fs');
const path = require('path');

const srcDir = 'D:/aayu-mobile-ide/dist';
const destDir = 'D:/INTENT-TO-SILICON/website/public/mobile-ide';

fs.mkdirSync(destDir, { recursive: true });
fs.mkdirSync(path.join(destDir, 'assets'), { recursive: true });

// Copy assets
const assets = fs.readdirSync(path.join(srcDir, 'assets'));
assets.forEach(f => {
  fs.copyFileSync(path.join(srcDir, 'assets', f), path.join(destDir, 'assets', f));
  console.log('Copied asset: ' + f);
});

// Read and adjust index.html
let html = fs.readFileSync(path.join(srcDir, 'index.html'), 'utf8');
html = html.split('src="/assets/').join('src="./assets/');
html = html.split('href="/assets/').join('href="./assets/');
fs.writeFileSync(path.join(destDir, 'index.html'), html, 'utf8');

console.log('Mobile IDE successfully published to website/public/mobile-ide/index.html');
