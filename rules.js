const convertDegToRad = (degrees) => degrees * Math.PI / 180;
const GROWTH_FACTOR = 0.1;
const ANGLE_SHIFT = 45;

const RULES = {
                 RECURSIONS: 10,
		 AXIOM: "X",
		 START_POSITION: {
                   x: 500,
                   y: 1000,
                   angle: 0,
                   memory: {
                        x: [],
			y: [],
			angle: []
		   }
		 },
                 STRING_CONVERT: {
					 "X": "F[-X]+X",
					 "F": "FF",
				 },
                 TURTLE: {
 
				      "F": (x, y, angle, memory, path) => {
					      const xn = x  + GROWTH_FACTOR * (Math.sin(convertDegToRad(angle)));
					      const yn = y - GROWTH_FACTOR * (Math.cos(convertDegToRad(angle)));


					      return [xn, yn, angle, memory, 
						     `${path} M ${x} ${y} L ${xn} ${yn}`];
				      },

				      "X": (x, y, angle, memory, path) => {
					      const xn = x  + GROWTH_FACTOR * (Math.sin(convertDegToRad(angle)));
					      const yn = y - GROWTH_FACTOR * (Math.cos(convertDegToRad(angle)));

					      return [xn, yn, angle, memory, path];
				      },

				      "+": (x, y, angle, memory, path) => {
					      return [x, y, angle + ANGLE_SHIFT , memory, path]; 
				      },

				      "-": (x, y, angle, memory, path) =>  {
					      return [x, y, angle - ANGLE_SHIFT, memory, path];
				      },

				      "[": (x, y, angle, memory, path) => {
					      return [
				              x, 
					      y,
					      angle, 
					      {
x: [...(memory.x.length === 0 ? [] : memory.x), x ],
y: [...(memory.y.length === 0 ? [] : memory.y), y ],
angle: [...(memory.angle.length === 0 ? [] : memory.angle), angle ],
					      },
path
					      ];
				      },

				      "]": (x, y, angle, memory, path) => {

					      x = memory.x[memory.x.length -1];
					      y = memory.y[memory.y.length -1];
					      angle = memory.angle[memory.angle.length -1];
					      return [
x, 
y, 
angle, 
{
x: memory.x.slice(0,-1),
y: memory.y.slice(0,-1),
angle: memory.angle.slice(0,-1),
},
path
];
				      }
			      }
}
