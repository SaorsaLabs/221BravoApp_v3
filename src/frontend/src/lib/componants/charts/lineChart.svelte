<script>
import { browser } from '$app/environment';
import { onMount } from 'svelte';
import { Chart, registerables,  } from 'chart.js';

export let dataArray1;
export let dataArray2;
export let dataArray3;
export let dataArray4;
export let labelsArray;
export let dataset1Title = "";
export let dataset2Title = "";
export let colours = ['rgb(75, 192, 192)', 'rgb(134, 52, 145)', 'rgb(95, 100, 176)', 'rgb(10, 203, 241)'];

Chart.register(...registerables);
let lineChartElement;

   onMount(() => {
         if (browser) {
    
      let DS = [];
        if (dataArray1) {
          DS.push({
            label: dataset1Title,
            data: dataArray1,
            fill: false,
            borderColor: colours[0],
            tension: 0.1
          });
        }
        if (dataArray2) {
          DS.push({
            label: dataset2Title,
            data: dataArray2,
            fill: false,
            borderColor: colours[1],
            tension: 0.1
          });
        }
        if (dataArray3) {
          DS.push({
            label: dataset2Title,
            data: dataArray3,
            fill: false,
            borderColor: colours[2],
            tension: 0.1
          });
        }
        if (dataArray4) {
          DS.push({
            label: dataset2Title,
            data: dataArray4,
            fill: false,
            borderColor: colours[3],
            tension: 0.1
          });
        }
        
        const labels = labelsArray;
        const data = {
          labels: labels,
          datasets: DS,
        };
               new Chart(lineChartElement, {
         type: 'line',
         data: data,
         options: {
            scales: {
                x: {
                    ticks: {
                        color: 'white'  // x-axis labels color
                    }
                },
                y: {
                    ticks: {
                        color: 'white'  // y-axis labels color
                    },
                },
            },
            plugins: {
                legend: {
                    labels: {
                        color: 'white' // Legend labels color
                    }
                },
            },
          }
       });
     }
   });

</script>
<canvas bind:this={lineChartElement} />
<style>

</style>

<!-- COLOUR OPTIONS
options: {
  scales: {
      x: {
          ticks: {
              color: 'white'  // x-axis labels color
          },
          grid: {
              color: 'white'  // x-axis grid line color
          }
      },
      y: {
          ticks: {
              color: 'white'  // y-axis labels color
          },
          grid: {
              color: 'white'  // y-axis grid line color
          }
      }
  },
  plugins: {
      legend: {
          labels: {
              color: 'white' // Legend labels color
          }
      }
  }
} -->