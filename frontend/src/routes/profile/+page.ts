import type { Load } from '@sveltejs/kit';
import { getCookie, setCookie } from 'typescript-cookie';

export const load: Load = async ({ fetch }) => {
    const sessionID = getCookie('sessionID');

    try {
        if (sessionID) {
            const response = await fetch('http://127.0.0.1:8000/check/', {
                method: 'GET',
                headers: {
                    'Session': sessionID
                }
            });

            const data = await response.json();
            if (data.test_passed) {
                setCookie('sessionID', sessionID, { expires: 1 });
                return {
                    props: {
                        username: data.username
                    }
                };
            } else {
                return {
                    props: {
                        get_new_session: true
                    }
                };
            }
        } else {
            return {
                props: {
                    get_new_session: true
                }
            };
        }
    } catch (error) {
        console.error('Fetch error:', error);
        return {
            props: {
                error: 'An error occurred while fetching data.'
            }
        };
    }
};
