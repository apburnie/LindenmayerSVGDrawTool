
function backgroundSetup (backgroundColour) {

	const background = document.getElementById("background");

	background.setAttribute("fill", backgroundColour);

}

function applyProductionRules(axiom) {

	let n = 0;

        let result = axiom;

	while (n <= (RULES.RECURSIONS ))  {
		result = result.split("").map((v, index) => {

	
				if (Object.keys(RULES.STRING_CONVERT).includes(v)) {
				return RULES.STRING_CONVERT[v];
				} else {
				return v;
				}

				}).join("");
		n = n + 1;
	}

	return result;
}

function applyTurtle(instructionString) {
	let x = RULES.START_POSITION.x;
	let y = RULES.START_POSITION.y;
	let angle = RULES.START_POSITION.angle;
	let memory = RULES.START_POSITION.memory;
        let path = `M ${RULES.START_POSITION.x} ${RULES.START_POSITION.y} `;

instructionString.split("").map( v => {
		if (Object.keys(RULES.TURTLE).includes(v)) {
		[x, y, angle, memory, path] = RULES.TURTLE[v](x, y, angle, memory, path);
		}

}).filter(x => x).join(" ");

console.log(path);

	const background = document.getElementById("drawing");

	background.setAttribute("d", path);

}



backgroundSetup("white");

const expandedString = applyProductionRules(RULES.AXIOM);

applyTurtle(expandedString);
