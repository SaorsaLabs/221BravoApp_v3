<!--  https://github.com/trash-and-fire/svelte-lightweight-charts -->
<!-- https://github.com/tradingview/lightweight-charts -->
<!-- //   const data = [
// 	{ time: '2018-10-19', open: 180.34, high: 180.99, low: 178.57, close: 179.85 },
// 	{ time: '2018-10-22', open: 180.82, high: 181.40, low: 177.56, close: 178.75 },
// 	{ time: '2018-10-23', open: 175.77, high: 179.49, low: 175.44, close: 178.53 },
// 	{ time: '2018-10-24', open: 178.58, high: 182.37, low: 176.31, close: 176.97 },
// 	{ time: '2018-10-25', open: 177.52, high: 180.50, low: 176.83, close: 179.07 },
// 	{ time: '2018-10-26', open: 176.88, high: 177.34, low: 170.91, close: 172.23 },
// ]; -->


<script>
  import { Chart, CandlestickSeries } from "svelte-lightweight-charts";
  import {CrosshairMode} from 'lightweight-charts';
  export let width = 700;
  export let height = 400;
  export let data;
  export let chartDecimals;
  export let darkMode = true;

  $: chartDecimals = chartDecimals;

  let textColour, backgroundColour, borderColour; 
  if (darkMode == true){
    textColour = 'rgba(200, 200, 200, 1)';
    backgroundColour = 'rgba(0, 130, 150, 0.2)';
    borderColour = 'rgba(197, 203, 206, 0.2)';
  } else {
    textColour = 'rgba(50, 50, 50, 1)';
    backgroundColour = 'rgba(0, 100, 150, 0.15)';
    borderColour = 'rgba(0, 0, 0, 0.1)';
  }

  let formatter = new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'USD',
        minimumFractionDigits: chartDecimals,
        // maximumFractionDigits: 8,
    }); 

  const options = {
        rightPriceScale: {
            borderVisible: true,
            borderColor: borderColour,

        },
        layout: {
            background: { color: backgroundColour },
            textColor: textColour,
        },
        grid: {
            vertLines: { color: borderColour },
            horzLines: { color: borderColour },
        },
        crosshair: {
            mode: CrosshairMode.Normal,
        },
		timeScale: {
                borderVisible: true,
                timeVisible: true,
                secondsVisible: false,
                borderColor: borderColour,
        },
        localization: {
            priceFormatter: price => formatter.format(price),//price.toFixed(4),
        },
    }

</script>

<Chart {...options} width={width} height={height} autoSize={true} >
	<CandlestickSeries data={data}/>
</Chart>


