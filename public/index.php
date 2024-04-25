<!doctype html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <title>Equestria.dev Status</title>
    <link rel="icon" href="./logo.svg" type="image/svg+xml">
    <style>
        <?= file_get_contents("./style.css") ?>
    </style>
</head>
<body>
    <noscript>This website requires JavaScript/ECMAScript to function. Please enable JavaScript (also known as ECMAScript) and refresh this page.</noscript>
    <div id="eqs-dark-theme-indicator"></div>
    <div id="eqs-loader">
        <svg id="eqs-loader-svg" viewBox="0 0 100 100" preserveAspectRatio="xMidYMid" style="shape-rendering: auto; display: block; background: transparent;">
            <g transform="rotate(0 50 50)">
                <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                    <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.875s"
                             repeatCount="indefinite"/>
                </rect>
            </g>
            <g transform="rotate(45 50 50)">
                <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                    <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.75s"
                             repeatCount="indefinite"/>
                </rect>
            </g>
            <g transform="rotate(90 50 50)">
                <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                    <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.625s"
                             repeatCount="indefinite"/>
                </rect>
            </g>
            <g transform="rotate(135 50 50)">
                <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                    <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.5s"
                             repeatCount="indefinite"/>
                </rect>
            </g>
            <g transform="rotate(180 50 50)">
                <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                    <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.375s"
                             repeatCount="indefinite"/>
                </rect>
            </g>
            <g transform="rotate(225 50 50)">
                <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                    <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.25s"
                             repeatCount="indefinite"/>
                </rect>
            </g>
            <g transform="rotate(270 50 50)">
                <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                    <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="-0.125s"
                             repeatCount="indefinite"/>
                </rect>
            </g>
            <g transform="rotate(315 50 50)">
                <rect x="47" y="28" rx="3" ry="3.84" width="6" height="12" fill="var(--theme-loader-stroke)">
                    <animate attributeName="opacity" values="1;0" keyTimes="0;1" dur="1s" begin="0s"
                             repeatCount="indefinite"/>
                </rect>
            </g>
        </svg>
    </div>
    <div id="eqs-app"></div>

    <script>
        <?= file_get_contents("./app.js") ?>
    </script>

    <script>
        window.va = window.va || function () { (window.vaq = window.vaq || []).push(arguments); };
    </script>
    <!--suppress HtmlUnknownTarget -->
    <script defer src="/_vercel/insights/script.js"></script>

    <script>
        window.si = window.si || function () { (window.siq = window.siq || []).push(arguments); };
    </script>
    <!--suppress HtmlUnknownTarget -->
    <script defer src="/_vercel/speed-insights/script.js"></script>
</body>
</html>
