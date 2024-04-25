#!/bin/bash

npx minify app.js > app.min.js
npx minify style.css > style.min.css

mv app.js app.max.js
mv style.css style.max.css
mv app.min.js app.js
mv style.min.css style.css

php index.php > index.html

npx minify index.html > index1.html
rm index.html
mv index1.html index.html

mv app.js app.min.js
mv style.css style.min.css
mv app.max.js app.js
mv style.max.css style.css
rm style.min.css
rm app.min.js

vercel --prod
