import type { Load } from '@sveltejs/kit';
import { getCookie, setCookie } from 'typescript-cookie';

const crypto = require('crypto');

export const profilePageLoad: Load = async ({ fetch }) => {
    const sessionID = getCookie('sessionID');

    try {
        if (sessionID != undefined) {
            const response = await fetch('http://127.0.0.1:8000/check/', {
                method: 'GET',
                headers: {
                    'Session': sessionID
                }
            });

            const data = await response.json();
            if (data.test_passed === true) {
                setCookie(sessionID, 'value', { expires: 1 });

                return {
                    username: data.username
                }

            } else {
                return {
                    get_new_session: true
                }
            }

        } else {
            // IDK I WILL DO IT IN FUTURE
        }
    } catch (error) {
        console.error('Fetch error:', error); // Log or handle error appropriately
        return {
            props: {
                error: 'An error occurred while fetching data.' // Optional: Return error information
            }
        };
    }
};