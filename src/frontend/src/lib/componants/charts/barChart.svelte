<script>
import { browser } from '$app/environment';
import { onMount } from 'svelte';
import { Chart, registerables } from 'chart.js';

export let dataArray;
export let dataArray2;
export let dataArray3;
export let dataArray4;
export let labelsArray;
export let datasetTitle = "";
export let dataset2Title = "";
export let dataset3Title = "";
export let dataset4Title = "";
export let showTitle = false;
export let darkMode = true;

Chart.register(...registerables);
let barChartElement;

   onMount(() => {
     if (browser) {

      let h = barChartElement.height;
      var gradientBlue = barChartElement.getContext('2d').createLinearGradient(0, h*0.75, 0, h*2);
      gradientBlue.addColorStop(0, 'rgb(0, 255, 226)');
      gradientBlue.addColorStop(1, 'rgb(0, 202, 216)');

      var gradientPurple = barChartElement.getContext('2d').createLinearGradient(0, h*0.75, 0, h*2);
      gradientPurple.addColorStop(0, 'rgb(255, 0, 226)');
      gradientPurple.addColorStop(1, 'rgb(100, 0, 216)');

      var gradientYellow = barChartElement.getContext('2d').createLinearGradient(0, h*0.75, 0, h*2);
      gradientYellow.addColorStop(0, 'rgb(150, 226, 0)');
      gradientYellow.addColorStop(1, 'rgb(202, 216, 0)');

      var gradientOrange = barChartElement.getContext('2d').createLinearGradient(0, h*0.75, 0, h*2);
      gradientOrange.addColorStop(0, 'rgb(255, 150, 100)');
      gradientOrange.addColorStop(1, 'rgb(255, 100, 100)');

      let DS = [];
      if (dataArray) {
        DS.push({
          label: datasetTitle,
          data: dataArray,
          backgroundColor: gradientBlue,
          // borderColor: ['rgb(240,255,255)'],
            borderRadius: 7,
          // borderWidth: 2,
        });
      }
      if (dataArray2) {
        DS.push({
          label: dataset2Title,
          data: dataArray2,
          backgroundColor: gradientPurple,
          // borderColor: ['rgb(240,255,255)'],
            borderRadius: 7,
          // borderWidth: 2,
        });
      }
      if (dataArray3) {
        DS.push({
          label: dataset3Title,
          data: dataArray3,
          backgroundColor: gradientYellow,
          // borderColor: ['rgb(240,255,255)'],
            borderRadius: 7,
          // borderWidth: 2,
        });
      }
      if (dataArray4) {
        DS.push({
          label: dataset4Title,
          data: dataArray4,
          backgroundColor: gradientOrange,
          // borderColor: ['rgb(240,255,255)'],
            borderRadius: 7,
          // borderWidth: 2,
        });
      }

        const chartData = {
            labels: labelsArray,
            datasets: DS,
        };
       let options = {
           plugins: {
                legend: {
                  display: showTitle,
                  tooltip: true,
                },
            },
          };
        if (darkMode == true) {
          Chart.defaults.color = 'rgb(240,255,255)'; // text
        } else {
          Chart.defaults.color = 'rgb(0,0,0)'; // text
        }
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