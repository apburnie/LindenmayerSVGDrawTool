<script lang="ts">
    import { drawString } from '$lib/svgGenerate/sketch'

    // User Inputs
    let axiom: string = ''
    let stringConvertStep = {
        m: '',
        l: '',
        f: '',
    }
    let recursions: string = '1'
    let angleShift: string = '45'

    let strokeWidth: number = 40

    // Set for User
    let startX: number = 0
    let startY: number = 10000
    let startAngle: number = 0
    let pathString: string = ''

    // Stage Variables
    let isFirstDraw = true
    let isLoading = true
    let isError = false

    let svgDrawSpace: SVGGraphicsElement

    async function triggerDraw() {
        if (isFirstDraw) {
            isFirstDraw = false
        }

        isLoading = true
        isError = false

        try {
            if (Number(recursions) > 0 && axiom !== '') {
                pathString = await drawString(
                    axiom,
                    stringConvertStep,
                    Number(recursions),
                    startX,
                    startY,
                    startAngle,
                    Number(angleShift)
                )
            } else {
                throw Error('Drawing failed - check parameters')
            }
        } catch (error) {
            console.error(error)
            isError = true
        } finally {
            isLoading = false
        }
    }

    function setValuesToDefault() {
        axiom = 'm'
        stringConvertStep = {
            m: 'l[-m]+m',
            l: 'll',
            f: '',
        }
        recursions = '10'
    }

    $: if (svgDrawSpace && !isLoading) {
        const bbox = svgDrawSpace.getBBox()
        svgDrawSpace.setAttribute(
            'viewBox',
            `${bbox.x} ${bbox.y} ${bbox.width} ${bbox.height}`
        )
        svgDrawSpace.setAttribute('stroke-width', `${strokeWidth / 10}rem`)
    }
</script>

<div class="container">
    <div class="result">
        {#if isFirstDraw}
            <div class="textresult">
                Please specify parameters and click Draw to draw diagram
            </div>
        {:else if isLoading}
            <div class="textresult">IS LOADING</div>
        {:else if !isError}
            <svg
                bind:this={svgDrawSpace}
                viewBox="0 0 100000 100000"
                xmlns="http://www.w3.org/2000/svg"
            >
                <path d={pathString} fill="none" stroke="black" id="drawing" />
            </svg>
        {:else}
            <div class="textresult">Draw Failed - Check Parameters</div>
        {/if}
    </div>

    <div>
        <h1>L-System SVG Drawer</h1>
        <div class="parameter">
            <h2>Algorithm</h2>
            <div>
                <label for="axiom">Axiom</label>
                <input bind:value={axiom} placeholder="" id="axiom" />
            </div>

            <div>
                <label for="mstep">Replace m with</label>
                <input
                    bind:value={stringConvertStep.m}
                    placeholder=""
                    id="mstep"
                />
            </div>

            <div>
                <label for="lstep">Replace l with</label>
                <input
                    bind:value={stringConvertStep.l}
                    placeholder=""
                    id="lstep"
                />
            </div>

            <div>
                <label for="fstep">Replace f with</label>
                <input
                    bind:value={stringConvertStep.f}
                    placeholder=""
                    id="fstep"
                />
            </div>

            <div>
                <label for="angleShift">Angle Shift</label>
                <div class="slidercontainer">
                    <input
                        bind:value={angleShift}
                        type="range"
                        id="recursions"
                        name="recursions"
                        min="0"
                        max="360"
                    />
                    <div>{angleShift}</div>
                </div>
            </div>
            <div>
                <label for="recursions">Number of Recursions</label>
                <div class="slidercontainer">
                    <input
                        bind:value={recursions}
                        type="range"
                        id="recursions"
                        name="recursions"
                        min="1"
                        max="15"
                    />
                    <div>{recursions}</div>
                </div>
            </div>

            <button on:click={triggerDraw}>Draw</button>
            <button on:click={setValuesToDefault}>Use Example</button>

            <div class="drawStyle">
                <h2>Display</h2>
                <label for="strokewidth">Stroke Width</label>

                <div class="slidercontainer">
                    <input
                        bind:value={strokeWidth}
                        type="range"
                        id="strokewidth"
                        name="strokewidth"
                        min="1"
                        max="1000"
                    />
                    <div>{strokeWidth / 10}rem</div>
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .container {
        height: 98vh;
        margin: 0;
        background: #f4f2f0;
        padding: 0;
        margin: 0;
        display: flex;
        flex-direction: row;
        justify-content: space-between;
    }

    .result {
        margin: auto;
    }

    .parameter,
    h1 {
        gap: 2rem;
        padding: 1rem;
        border: 1px solid black;
        border-radius: 10px;
        margin: 1rem;
    }

    .parameter {
        height: 80vh;
        overflow: scroll;
    }

    .parameter > div {
        gap: 1rem;
    }

    .parameter,
    .parameter > div {
        display: flex;
        flex-direction: column;
        justify-content: center;
        min-width: 20vw;
    }

    .parameter button {
        background: white;
        border-radius: 20px;
        border: 1px solid #1d1a16;
        cursor: pointer;
        font-size: 1rem;
        padding: 0.5rem;
        color: #594d40;
    }

    .parameter button:hover {
        background: #ebfee6;
        color: #0b3102;
    }

    .slidercontainer {
        display: flex;
        justify-content: space-between;
    }

    .slidercontainer input {
        flex-grow: 1;
    }

    .slidercontainer div {
        width: 4rem;
        text-align: right;
    }

    .textresult {
        flex-grow: 1;
    }

    svg {
        height: 95vh;
        stroke: black;
        background: white;
    }

    .drawStyle {
        border-top: 1px solid;
        padding-top: 2rem;
    }
</style>
