#!/bin/bash

npx minify --js app.js app.min.js
npx minify --css style.css style.min.css

mv app.js app.max.js
mv style.css app.max.css
mv app.min.js app.js
mv style.min.css style.css

php index.php > index.html

mv app.js app.min.js
mv style.css app.min.css
mv app.max.js app.js
mv style.max.css style.css
rm --interactive=never style.max.css
rm --interactive=never app.max.js

#vercel --prod
