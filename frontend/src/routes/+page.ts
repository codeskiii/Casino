import type { Load } from '@sveltejs/kit';

export const load: Load = async ({ fetch }) => {
  try {
    // Perform the API request
    const response = await fetch('http://127.0.0.1:8000');
    
    // Check if the response is okay
    if (!response.ok) {
      throw new Error('Failed to fetch data');
    }

    // Parse the response data
    const data = await response.json();
    
    // Return the data to be used in the component
    return {
      props: {
        data
      }
    };
  } catch (error) {
    console.error('Error fetching data:', error);
    return {
      props: {
        data: null,
      }
    };
  }
};