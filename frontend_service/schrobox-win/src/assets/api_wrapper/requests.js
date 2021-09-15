import APIError from './APIError';

async function _request(url = '', method = '', headers = {}, data = null){
	const response = await fetch(url, {
		method: method,
		headers: {
			'Content-Type': 'application/json',
			...headers
		},
		referrerPolicy: 'no-referrer-when-downgrade',
		body: data == null ? null : JSON.stringify(data) // body data type must match "Content-Type" header
	});
	if (response.headers.get('Content-Type') == 'application/json') {
		let result = await response.json(); // parses JSON response into native JavaScript objects
		return new Promise((resolve, reject) => {
			if (response.ok) {
				resolve(result);
			} else {
				reject(new APIError(result.message, result.reason, response.status));
			}
		});
	} else {
		let txt = await response.text();
		return new Promise((resolve, reject) => {
			if (response.ok) {
				if (txt != '') {
					console.warn('Could not parse request body as JSON, but body was not empty either');
				}
				resolve({ text: txt });
			} else {
				reject(new APIError("Request failed but we don't know why", 'UnknownError', response));
			}
		});
	}
}

export function POST(url = '', headers = {}, data = {}) {
	return _request(url, 'POST', headers, data);
}

export function GET(url = '', headers = {}) {
	return _request(url, 'GET', headers);
}

export function PATCH(url = '', headers = {}, data = {}) {
	return _request(url, 'PATCH', headers, data);
}

export function DELETE(url = '', headers = {}, data = {}) {
	return _request(url, 'DELETE', headers, data);
}
