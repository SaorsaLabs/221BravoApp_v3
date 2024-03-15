<script>

    export let style = 'loaderBlue'; // loaderBlue loaderBlueSmall
    export let align = 'centre';  // centre or standard

</script>

<div class="{style} {align}"></div>

<style>
    .loader {
        border: 6px solid #f3f3f3;
        border-radius: 50%;
        border-top: 6px solid #444444;
        width: 45px;
        height: 45px;
        animation: spin 1s linear infinite;
        z-index: 1000;
    }

    .loaderBlue {
        border: 6px solid #00cad8;
        border-radius: 50%;
        border-top: 6px solid #444444;
        width: 45px;
        height: 45px;
        animation: spin 1s linear infinite;
        z-index: 1000;
    }

    .loaderBlueSmall {
        border: 6px solid #00cad8;
        border-radius: 50%;
        border-top: 6px solid #444444;
        width: 20px;
        height: 20px;
        animation: spin 1s linear infinite;
        z-index: 1000;
    }

    .centre {
        position: relative;
        margin: auto;
    }
    .standard{
        position: relative;
    }
    
    @keyframes spin {
        100% {
            transform: rotate(360deg);
        }
    }
</style>