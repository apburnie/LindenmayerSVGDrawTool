import init, { generate_fractal } from '$lib/wasm/fractal_draw'

async function drawString(
    axiom: string,
    stringConvertStep: { m?: string; l?: string; f?: string },
    recursions: number,
    startX: number,
    startY: number,
    startAngle: number,
    angleShift: number
): Promise<string> {
    await init()

    const result = await generate_fractal(
        axiom,
        stringConvertStep.m || '',
        stringConvertStep.l || '',
        stringConvertStep.f || '',
        recursions,
        startX,
        startY,
        startAngle,
        angleShift
    )

    return result
}

export { drawString }
