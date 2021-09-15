import { POST, GET, PATCH, DELETE } from './requests';
import { backendURL } from './global';

export default class AuthAPI {
	constructor() {
		this.baseURL = '/auth';
	}

	login(email = '', password = '') {
		let url = backendURL + baseURL + '/users/login';
		let body = {
			email: email,
			password: password,
		};
		return POST(url, {}, body);
	}

	signup(email = '', password = '', firstName = '', lastName = '', imageFile = '') {
		let url = backendURL + baseURL + '/users/signup';
		let body = {
			email: email,
			password: password,
			firstName: firstName,
			lastName: lastName,
			avatar: imageFile,
		};
		return POST(url, {}, body);
	}

	refresh(refreshToken) {
		let url = backendURL + baseURL + '/refresh';
		return POST(url, { Authorization: refreshToken }, {});
	}

	forgotPasswordRequest(email = '') {
		let url = backendURL + baseURL + '/users/request-reset-password';
		let body = {
			email: email,
		};
		return POST(url, {}, body);
	}

	resetPassword(email = '', password = '', resetKey = '') {
		let url = backendURL + baseURL + '/users/reset-password';
		let body = {
			email: email,
			password: password,
			passwordResetRequest: resetKey,
		};
		return POST(url, {}, body);
	}
}
