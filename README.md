# n2yo-satellite-position-scraper
This application scrapes the N2YO Rest API for Tracking satellite positions.

Examples of json from different N2YO API endpoints

# Example 1: Summary of Information about satellite

You can get a two line summary information about satellites using the following endpoint https://api.n2yo.com/rest/v1/satellite/tle/25544&apiKey=

```json
{
    "info": {
        "satid": 25544,
        "satname": "SPACE STATION",
        "transactionscount": 0
    },
    "tle": "1 25544U 98067A   24346.21840358  .00019894  00000-0  34957-3 0  9992\r\n2 25544  51.6394 158.3036 0007055 326.7949 203.7258 15.50472908486009"
}

```
# Example 2: Get satellite positions
You can get satellite position using the following n2yo api endpoint:

https://api.n2yo.com/rest/v1/satellite/positions/25544/{observer_lat}/{observer_long}/0/1/&apiKey=

```json
{
    "info": {
        "satname": "SPACE STATION",
        "satid": 25544,
        "transactionscount": 9
    },
    "positions": [{
        "satlatitude": 3.2100203,
        "satlongitude": -105.23413431,
        "sataltitude": 411.83,
        "azimuth": 288.79,
        "elevation": -49.03,
        "ra": 134.13263841,
        "dec": -25.91597621,
        "timestamp": 1733919339,
        "eclipsed": false
    }, {
        "satlatitude": 3.26103633,
        "satlongitude": -105.19804899,
        "sataltitude": 411.83,
        "azimuth": 288.79,
        "elevation": -49,
        "ra": 134.1601113,
        "dec": -25.89075817,
        "timestamp": 1733919340,
        "eclipsed": false
    }]
}
```

# References
- https://www.n2yo.com/api/

