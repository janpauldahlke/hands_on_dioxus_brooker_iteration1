## simply having fun with dioxus, trying it out a little, curios about this, more stuff will follow --maybe ;-)

#### scope of this project is to generate data about stocks in an app, show all actual data and plot it in a later step, also we like to use dioxus to compile for all targets, the usual developers wet dream, build once compile for all :-P

- get a API key from finnhub.io
- create `.env` like so
  - `FINNHUB_API_KEY=d<yourKeyHere>`

### how to:

- follow the installation process on dioxuslabs.com
- in this project run `dx serve --web` to gain a local hot-reloadable webserver

### today we learned that finnhub.io does not provide historicalStockData on the free tier, time for 🍻

##### TODO; explore

Top free-tier options

1. Alpha Vantage
   Free tier: 5 API calls/minute, 500 calls/day
   Historical data: Yes (TIME_SERIES_DAILY endpoint)
   Data format: OHLC (Open, High, Low, Close) + Volume
   Notes: Popular, well-documented, good for development
2. StockData.org
   Free tier: 100 daily requests (no credit card)
   Historical data: Yes (intraday and historical)
   Notes: Simple setup, no payment required
3. Marketstack
   Free tier: 100 monthly requests
   Historical data: End-of-day data from 30,000+ tickers
   Notes: Global coverage, JSON format
4. EOD Historical Data (EODHD)
   Free tier: 20 API calls/day
   Historical data: Up to 1 year of historical data
   Notes: 30+ years available on paid plans
5. Tiingo
   Free tier: Limited historical data
   Historical data: End-of-day prices
   Notes: Good documentation
6. StockAPI
   Free tier: 1,000 API calls/month
   Historical data: Basic market data
   Notes: Community support
