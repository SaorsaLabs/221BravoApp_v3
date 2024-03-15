let _slugData = 0;
const load = async ({ params, url }) => {
	const id = url.searchParams.get('id');
	_slugData = {
		id,
	};
};

export { _slugData, load };
