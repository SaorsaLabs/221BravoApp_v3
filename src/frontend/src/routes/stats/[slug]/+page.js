let _slugData = 0;
const load = async ({ params, url }) => {
	const token = params.slug;
	let capToken = token.toUpperCase();
	_slugData = {
		token: capToken,
	};
};

export { _slugData, load };
