<script>
import icpLogo from '$lib/images/tokenLogos/icpLogo.png';
import ckBTCLogo from '$lib/images/tokenLogos/ckBTC_logo.svg';
import chatLogo from '$lib/images/tokenLogos/Openchat_logo.png';
import kinicLogo from '$lib/images/tokenLogos/kinic.png';
import hotLogo from '$lib/images/tokenLogos/hotOrNot.png';
import ghostLogo from '$lib/images/tokenLogos/ghost_logo.png';
import snsLogo from '$lib/images/tokenLogos/SNS1_Logo.png';
import modLogo from '$lib/images/tokenLogos/ModClub.png';
import catLogo from '$lib/images/tokenLogos/catalyzeLogo.png';
import boomLogo from '$lib/images/tokenLogos/BoomLogo.png'; 
import icxLogo from '$lib/images/tokenLogos/IcxLogo.png';
import nuanceLogo from '$lib/images/tokenLogos/nuanceLogo.png';
import sonicLogo from   '$lib/images/tokenLogos/sonicLogo.png';
import talLogo from   '$lib/images/tokenLogos/TalLogo.png';
import ckEthLogo from   '$lib/images/tokenLogos/ckEthLogo.png';
import ogyLogo from '$lib/images/tokenLogos/ogyLogo.png';
import ntnLogo from '$lib/images/tokenLogos/nutriniteLogo.png';
import gldLogo from  '$lib/images/tokenLogos/goldDAO.png';
import traxLogo from  '$lib/images/tokenLogos/traxLogo.png';
import sneedLogo from '$lib/images/tokenLogos/sneedLogo.png';
import exeLogo from '$lib/images/tokenLogos/Windoge98Logo.jpg';

export let token;
export let width; // eg "100%" , "50px"
export let logoStyle;
//let sonicWidth = "80%";

$: token;

</script>

<div class={logoStyle}>
    {#if token == "ICP"}
    <img src={icpLogo} alt='Token Logo' width="{width}"/>
{:else if token == "CKBTC"}
    <img src={ckBTCLogo} alt='Token Logo' width="{width}"/>
{:else if token == "CHAT"}
    <img src={chatLogo} alt='Token Logo' width="{width}"/>
{:else if token == "KINIC"}
    <img src={kinicLogo} alt='Token Logo' width="{width}"/>
{:else if token == "HOT"}
    <img src={hotLogo} alt='Token Logo' width="{width}"/>
{:else if token == "GHOST"}
    <img src={ghostLogo} alt='Token Logo' width="{width}"/>
{:else if token == "SNS1"}
    <img src={snsLogo} alt='Token Logo' width="{width}"/>
    <!-- MODCLUB need depricated! -->
{:else if token == "MODCLUB"}
    <img src={modLogo} alt='Token Logo' width="{width}"/>
{:else if token == "MOD"}
<img src={modLogo} alt='Token Logo' width="{width}"/>
{:else if token == "CAT"}
    <img src={catLogo} alt='Token Logo' width="{width}"/>
{:else if token == "BOOM"}
    <img src={boomLogo} alt='Token Logo' width="{width}"/>
{:else if token == "ICX"}
    <img src={icxLogo} alt='Token Logo' width="{width}"/>
{:else if token == "NUA"}
    <img src={nuanceLogo} alt='Token Logo' width="{width}"/>
{:else if token == "SONIC"}
    <img src={sonicLogo} alt='Token Logo' width="{width}"/>
{:else if token == "TAL"}
    <img src={talLogo} alt='Token Logo' width="{width}"/>
{:else if token == "CKETH"}
    <img src={ckEthLogo} alt='Token Logo' width="{width}"/>
{:else if token == "OGY"}
    <img src={ogyLogo} alt='Token Logo' width="{width}"/>
{:else if token == "NTN"}
    <img src={ntnLogo} alt='Token Logo' width="{width}"/>
{:else if token == "GLDGOV"}
    <img src={gldLogo} alt='Token Logo' width="{width}"/>
{:else if token == "TRAX"}
    <img src={traxLogo} alt='Token Logo' width="{width}"/>
{:else if token == "SNEED"}
    <img src={sneedLogo} alt='Token Logo' width="{width}"/>
{:else if token == "EXE"}
    <img src={exeLogo} alt='Token Logo' width="{width}"/>
{/if}
</div>
