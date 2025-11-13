## simply having fun with dioxus, trying it out a little, curios about this, more stuff will follow --maybe ;-)

#### scope of this project is to generate data about stocks in an app, show all actual data and plot it in a later step, also we like to use dioxus to compile for all targets, the usual developers wet dream, build once compile for all :-P

- get a API key from finnhub.io
- create `.env` like so
  - `FINNHUB_API_KEY=d<yourKeyHere>`

### how to:

- follow the installation process on dioxuslabs.com
- in this project run `dx serve --web` to gain a local hot-reloadable webserver

### today we learned that finnhub.io does not provide historicalStockData on the free tier, time for 🍻
