# WebNowPlaying-Redux Error Proxy
Proxies error reports received by [WebNowPlaying-Redux](https://github.com/keifufu/WebNowPlaying-Redux) to a Discord Webhook.  
This depends on nginx to configure SSL, and to proxy /report to /.

For now there is no rate limiting as I'm hoping people won't abuse it.  
However, implementing a rate limiter would be very easy either way.

## Running
Configure .env  
`pnpm i`  
`pnpm run build`  
`pm2 start dist/index.js --name wnpredux-error-proxy` or `pnpm run start`