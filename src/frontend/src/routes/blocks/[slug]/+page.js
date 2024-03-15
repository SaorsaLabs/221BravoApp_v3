let _slugData = 0;
const load = async ({ params, url }) => {
	const token = params.slug;
	let capToken = token.toUpperCase();
	const startBlock = url.searchParams.get('start');
	const endBlock = url.searchParams.get('end');
	_slugData = {
		token: capToken,
		startBlock,
		endBlock
	};
};

export { _slugData, load };
