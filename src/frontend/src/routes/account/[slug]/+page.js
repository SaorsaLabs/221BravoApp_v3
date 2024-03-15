let _slugData = 0;
const load = async ({ params, url }) => {
	const token = params.slug;
	let capToken = token.toUpperCase();
	const id = url.searchParams.get('id');
	_slugData = {
		token: capToken,
		id,
	};
};

export { _slugData, load };
