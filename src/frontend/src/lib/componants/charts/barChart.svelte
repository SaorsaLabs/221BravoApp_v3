<script>
import { browser } from '$app/environment';
import { onMount } from 'svelte';
import { Chart, registerables } from 'chart.js';

export let dataArray;
export let labelsArray;
export let datasetTitle = "";
export let showTitle = false;

Chart.register(...registerables);
let barChartElement;

   onMount(() => {
     if (browser) {

        let h = barChartElement.height;
        var gradientBlue = barChartElement.getContext('2d').createLinearGradient(0, h*0.75, 0, h*2);
        gradientBlue.addColorStop(0, 'rgb(0, 255, 226)');
        gradientBlue.addColorStop(1, 'rgb(0, 202, 216)');

        const chartData = {
            labels: labelsArray,
            datasets: [
            {
                label: datasetTitle,
                data: dataArray,
                backgroundColor: gradientBlue,
                // borderColor: ['rgb(240,255,255)'],
                 borderRadius: 7,
                // borderWidth: 2,
            },
            ],
        };
       let options = {
           plugins: {
                legend: {
                  display: showTitle,
                  tooltip: true,
                },
            },
          };
        Chart.defaults.color = 'rgb(240,255,255)'; // text
        Chart.defaults.borderColor = 'rgba(20,20,20,0.45)';
       new Chart(barChartElement, {
         type: 'bar',
         data: chartData,
         options: options,
       });
     }
   });

</script>
<canvas bind:this={barChartElement} />
<style>

</style>