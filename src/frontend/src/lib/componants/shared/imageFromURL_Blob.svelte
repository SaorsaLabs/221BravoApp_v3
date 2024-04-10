<script>
  import { onMount } from "svelte";
    export let imageUrl;
  
    let imageDataUrl;
    let imageLoaded = false; 

    async function fetchImg(imageUrl){
		let settings = { method: "Get" };
		let url = imageUrl;
		const response = await fetch(url, settings);
		const data2 = await response.blob();
		imageDataUrl = await getTextFromBlob(data2);
		imageLoaded = true;
	}

	async function getTextFromBlob(blob) {
    return new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.onload = function() {
            resolve(reader.result);
        };
        reader.onerror = function() {
            reject(reader.error);
        };
        reader.readAsText(blob);
    });
  }

  onMount(()=>{
    if(imageUrl){
      fetchImg(imageUrl)
    }
  });

  </script>
  
  {#if imageLoaded == true}
    <img src={imageDataUrl} alt="">
  {/if}