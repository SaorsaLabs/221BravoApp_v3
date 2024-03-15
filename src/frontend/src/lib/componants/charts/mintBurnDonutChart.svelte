<script>
import { browser } from '$app/environment';
import { onMount } from 'svelte';
import { Chart, registerables } from 'chart.js';

export let dataArray;
export let labelsArray;
export let datasetTitle = "";
export let token = "";

Chart.register(...registerables);
let donutChartElement;

   onMount(() => {
     if (browser) {

      let h = donutChartElement.height;
        var gradientBlue = donutChartElement.getContext('2d').createLinearGradient(0, h*0.75, 0, h*2);
          gradientBlue.addColorStop(0, 'rgb(0, 255, 226)');
          gradientBlue.addColorStop(1, 'rgb(0, 202, 216)');

        var gradientPurple = donutChartElement.getContext('2d').createLinearGradient(0, h*0.75, 0, h*2);
        gradientPurple.addColorStop(0, 'rgb(134, 72, 185)');
        gradientPurple.addColorStop(1, 'rgb(134, 52, 145)');

        var gradientBlue2 = donutChartElement.getContext('2d').createLinearGradient(0, h*0.75, 0, h*2);
        gradientBlue2.addColorStop(0, 'rgb(95, 140, 206)');
        gradientBlue2.addColorStop(1, 'rgb(95, 100, 146)');

        const chartData = {
            labels: labelsArray,
            datasets: [
            {
                label: "Total "+token,
                data: dataArray,
                backgroundColor: [
                gradientPurple, //'rgb(134, 52, 145)',
                gradientBlue2,  //'rgb(95, 100, 176)',
                gradientBlue    //'rgb(10, 203, 241)'
              ],
              hoverOffset: 6,
                // borderColor: ['rgb(240,255,255)'],
                // borderRadius: 7,
                // borderWidth: 2,
            },
            ],
        };
       let options = {
           plugins: {
                title: {
                  display: true,
                  text: datasetTitle
                },
                legend: {
                  display: true,
                  tooltip: true,
                },
            },
            cutout : '80%',
            responsive: true,
            maintainAspectRatio: false
          };

        Chart.defaults.color = 'rgb(240,255,255)'; // text
        Chart.defaults.borderColor = 'rgba(20,20,20,0.45)';
       new Chart(donutChartElement, {
         type: 'doughnut',
         data: chartData,
         options: options,
       });
     }
   });

</script>
<canvas bind:this={donutChartElement} />
<style>

</style>
