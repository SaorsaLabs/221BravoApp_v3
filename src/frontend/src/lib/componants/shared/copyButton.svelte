<script>
  import { onMount } from "svelte";
  import copy from "$lib/images/icons/copyIcon_sml.png";
  import copy2 from "$lib/images/icons/copyIcon_sml_white.png";
  
  export let text = '';

  let bgChange = false;
  let isDarkMode;
  onMount(() => {
    // [][] for switching icons light/ dark [][]
		const updateMode = () => {
			isDarkMode = document.documentElement.classList.contains('dark');
		};
		// Initial check
		updateMode();
		// Listen for class changes on the html element
		const observer = new MutationObserver(mutations => {
			mutations.forEach(mutation => {
			if (mutation.attributeName === "class") {
				updateMode();
			}
			});
		});
		observer.observe(document.documentElement, { attributes: true });
		return () => {
			observer.disconnect();
		}    
    });

    function setClipboard(text) {
    const type = "text/plain";
    let writeText = text;
    const blob = new Blob([writeText], { type });
    const data = [new ClipboardItem({ [type]: blob })];

      navigator.clipboard.write(data).then(
        () => {
          bgChange = true;
          setTimeout(resetColour, 500)
        },
        () => {
          bgChange = false;
        }
      );
    }
    function resetColour() {
      bgChange = false;
    }
  </script>
  
  <button on:click={() => setClipboard(text)}>
    <img class="copy" src={isDarkMode ? copy2 : copy} class:BG={bgChange} alt="copyText" width="20px" style="margin-left:5px"/>
  </button>

  
  <style>
    button{
      cursor: pointer;
      background: none;
      padding: 0px;
      margin: 0px;
      border: 0px;
    }
    .BG{
      background-color: rgb(40, 202, 40);
      border-radius: 4px;
    }
  </style>