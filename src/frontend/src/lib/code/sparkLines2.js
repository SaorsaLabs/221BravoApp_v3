// used to create SVG sparklines

export function createSparkline(inputData, strokeWidth, width, height, reverse) {
    if (inputData == null || inputData == undefined || inputData == []) return;
    let data = inputData;
    if (reverse == true) {
        data.reverse();
    }
    let highest = Math.max(...data)
    let lowest = Math.min(...data);
    let range = highest - lowest;
    let pctData = data.map((num) => (((num - lowest)/range)*50));
    let adjData = pctData;//data.map((num) => ((highest - num)*3));
    let textChunks = [];
    let chunk;
    let len = adjData.length ?? 0;
    let colour = (data[len-1] - data[0] < 0) ? "green" : "red";
    for(let i=1; i<len; i++){
        chunk = ` L ${i} ${adjData[i]}`;
        textChunks.push(chunk);
    }
    let firstPoint = `M 0 ${adjData[0]}`;
    let linkPath = firstPoint.concat(...textChunks);
    let lenAdj = adjData.length-1;
    //  
    let svg = `
        <svg height="${height}px" width="${width}px" viewBox="0 0 ${lenAdj} ${100}" preserveAspectRatio="none">
        <path
            d="${linkPath}"
            stroke-width="${strokeWidth}"
            stroke="${colour}"
            fill="transparent"
            vector-effect="non-scaling-stroke"
        />`;
    return svg;
}


