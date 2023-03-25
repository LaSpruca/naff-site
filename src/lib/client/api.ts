import { PUBLIC_BACKEND } from '$env/static/public';
// const PUBLIC_BACKEND = 'http://0.0.0.0:8080';

export class ApiError extends Error {
	public code: number;

	constructor({ error: message, code }: { error: string; code: number }) {
		super(message);
		this.name = 'ApiError';
		this.code = code;
	}
}

export type User = {
	id: string;
	name: string;
	email: string;
};

export type Team = {
	id: string;
	name: string;
	film_name: string;
	film_descriptions: string;
	has_file: boolean;
};

export const getUser = async (options?: { fetch: typeof fetch; token: string }): Promise<User> => {
	let fetch_options: RequestInit = {
		credentials: 'include',
		headers: { ...(options ? { Authorization: options.token } : {}) }
	};

	const fetch_fn = options?.fetch ?? fetch;
	let request = await fetch_fn(`${PUBLIC_BACKEND}/api/user`, fetch_options);
	let requestJson = await request.json();

	if (request.status != 200) {
		throw new ApiError(requestJson);
	}

	return requestJson;
};

export const getTeam = async (options?: {
	fetch: typeof fetch;
	token: string;
}): Promise<Team | null> => {
	let fetch_options: RequestInit = {
		credentials: 'include',
		headers: { ...(options ? { Authorization: options.token } : {}) }
	};

	const fetch_fn = options?.fetch ?? fetch;
	let request = await fetch_fn(`${PUBLIC_BACKEND}/api/team/`, fetch_options);
	let requestJson = await request.json();

	if (request.status != 200) {
		throw new ApiError(requestJson);
	}

	return requestJson;
};

export const getMembers = async (
	teamId: string,
	options?: {
		fetch: typeof fetch;
		token: string;
	}
): Promise<User[]> => {
	let fetch_options: RequestInit = {
		credentials: 'include',
		headers: { ...(options ? { Authorization: options.token } : {}) }
	};

	const fetch_fn = options?.fetch ?? fetch;
	let request = await fetch_fn(`${PUBLIC_BACKEND}/api/team/${teamId}/members`, fetch_options);
	let requestJson = await request.json();

	if (request.status != 200) {
		throw new ApiError(requestJson);
	}

	return requestJson;
};

export const joinTeam = async (code: string): Promise<Team> => {
	let response = await fetch(`${PUBLIC_BACKEND}/api/team/join?id=${code}`, {
		method: 'POST',
		credentials: 'include'
	});
	// The API will always return a JSON response, unless you fuck'd up real bad
	let responseJson = await response.json();

	if (response.status != 200) {
		throw new ApiError(responseJson);
	}

	return responseJson;
};

export const createTeam = async (code: string): Promise<Team> => {
	let response = await fetch(`${PUBLIC_BACKEND}/api/team/new?name=${code}`, {
		method: 'POST',
		credentials: 'include'
	});
	// The API will always return a JSON response, unless you fuck'd up real bad
	let responseJson = await response.json();

	if (response.status != 200) {
		throw new ApiError(responseJson);
	}

	return responseJson;
};

export const leaveTeam = async (): Promise<void> => {
	let response = await fetch(`${PUBLIC_BACKEND}/api/team/leave`, {
		method: 'POST',
		credentials: 'include'
	});
	// The API will always return a JSON response, unless you fuck'd up real bad
	let responseJson = await response.json();

	if (response.status != 200) {
		console.log('Throwing error');
		throw new ApiError(responseJson);
	}
};
