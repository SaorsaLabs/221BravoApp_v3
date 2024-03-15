import { browser } from '$app/environment';
import { writable } from 'svelte/store';
import { MAX_TIME_COOKIE } from '../code/constants.js';

export const authTrigger = writable(0);

const initCookieLocalStorage = () => {
	if (!browser)
		return {
			init: () => {},
			set: (status) => {},
			check: () => {},
			read: () => {}
		};
	return {
		init: async () => {
			let sV = JSON.parse(localStorage.getItem('cookieStore')) ?? null;
			if (sV == null || sV == 'null') {
				let d = new Date();
				let time = d.getTime() / 1000; // current in secs.
				localStorage.setItem(
					'cookieStore',
					JSON.stringify({
						data: { allowCookies: false, authTime: time }
					})
				);
			} else {
				// check expired
				let d = new Date();
				let time = d.getTime() / 1000; // current in secs.
				let sVTime = sV.data.authTime;
				let timeSince = time - sVTime;
				if (timeSince >= MAX_TIME_COOKIE || sV.data.allowCookies == false) {
					let d = new Date();
					let time = d.getTime() / 1000; // current in secs.
					localStorage.setItem(
						'cookieStore',
						JSON.stringify({
							data: { allowCookies: false, authTime: time }
						})
					);
				}
			}
		},
		set: (status, time) => {
			if (browser) {
				let data = {
					data: {
						allowCookies: status, authTime: time
					}
				};
				localStorage.setItem('cookieStore', JSON.stringify(data));
			}
		},
		check: async () => {
			if (browser) {
				let localcookieStore = JSON.parse(localStorage.getItem('cookieStore')) ?? null;
				let d = new Date();
				let time = d.getTime() / 1000; // secs
				let since;
				if (localcookieStore != null) {
					since = time - localcookieStore.data.authTime;
					// need to re-authorise the cookie?
					if (since > MAX_TIME_COOKIE) {
						let data = {
							data: {
								allowCookies: false, authTime: time
							}
						};
						localStorage.setItem('cookieStore', JSON.stringify(data));
						return true;
					}
					return "no action needed";
				} else {
					return "cookieStore doesn't exist yet";
				}
			}
		},
		read: () => {
			if (browser) {
				return JSON.parse(localStorage.getItem('cookieStore'));
			}
		}
	};
};

export const cookieStore = initCookieLocalStorage();
