<!-- source: https://codepen.io/shubhamjainco/pen/oPEzrV -->

<script lang="ts">
    import { getCookie } from 'typescript-cookie';
    import { onMount } from 'svelte';

    let user = "Login/Register";

    async function user_collect() {
        const sessionID = getCookie('sessionID');

        if (!sessionID) {
            return;
        }

        try {
            const response = await fetch('http://127.0.0.1:8000/check/', {
                method: 'GET',
                headers: {
                    'Session': sessionID
                }
            });

            if (!response.ok) {
                return;
            }

            const apiResponse = await response.json();
            user = apiResponse.username;

        } catch (error) {
            console.error('Error fetching user data:', error);
            user = "Login/Register";
        }
    }

    onMount(async () => {
        await user_collect();
    });
</script>

<nav class="uk-container uk-navbar">
    <div class="uk-navbar-left">
        <ul class="uk-navbar-nav">
            <li class="uk-active">
                <a href="/" class="uk-text-large" style="font-size: 160%;">RUST<strong>CASINO</strong></a>
            </li> 
        </ul>
    </div>
    <div class="uk-navbar-right">
        <ul class="uk-navbar-nav uk-visible@s">
            <li><a class="uk-text-large" style="font-size: 110%;" href="/">blank</a></li>
            {#if (user != "Login/Register")}
            <li><a class="uk-text-large" style="font-size: 110%;" href="/profile">{ user }</a></li>
            {:else}
            <li><a class="uk-text-large" style="font-size: 110%;" href="/logging_page">{ user }</a></li>
            {/if}
        </ul>
    </div>
</nav>