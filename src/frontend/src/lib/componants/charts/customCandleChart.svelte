<script>
    import { onMount } from 'svelte';
    import { createChart } from 'lightweight-charts';

    let chartContainer;

    onMount(() => {
        const chart = createChart(chartContainer, {
            width: chartContainer.offsetWidth,
            height: 400
        });

        const candleSeries = chart.addCandlestickSeries();

        // oldest to newest
        candleSeries.setData([
            {time: 1705254300, open: 0.072522273233109, high: 0.07277424396307258, low: 0.072522273233109, close: 0.07274039328981874},
            {time: 1705255200, open: 0.07274039328981874, high: 0.07246625301280997, low: 0.07274039328981874, close: 0.07246625301280997},
            {time: 1705256100, open: 0.07246625301280997, high: 0.0722836643561569, low: 0.07246625301280997, close: 0.0722836643561569},
            {time: 1705257000, open: 0.0722836643561569, high: 0.07218992040999417, low: 0.0722836643561569, close: 0.07218992040999417},
            {time: 1705257900, open: 0.07218992040999417, high: 0.07218992040999417, low: 0.07218992040999417, close: 0.07224417400626942},
            {time: 1705258800, open: 0.07224417400626942, high: 0.07259908455971677, low: 0.07224417400626942, close: 0.07298039041937052}           

            // { time: '2023-01-26', open: 100, high: 110, low: 90, close: 105 },
            // { time: '2023-01-27', open: 100, high: 110, low: 90, close: 105 },
            // Add more data points here...
        ]);
    });
</script>

<div bind:this={chartContainer} style="height: 400px; width: 100%;"></div>