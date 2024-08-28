<script lang="ts">
	// Function to compute SHA-1 hash using Web Crypto API

	async function sha1Hash(message: string): Promise<string> {
	  const encoder = new TextEncoder();
	  const data = encoder.encode(message);
  
	  const hashBuffer = await crypto.subtle.digest('SHA-1', data);
	  const hashArray = Array.from(new Uint8Array(hashBuffer));
	  const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
  
	  return hashHex;
	}
  
	let username = '';
	let password = '';
  
	async function logging_service() {
	  console.log("Username:", username);
	  console.log("Password:", password);
  
	  const hashedPassword = await sha1Hash(password);
  
	  const login_response = await fetch("http://127.0.0.1:8000/logging/", {
		method: 'POST',
		headers: {
			'Content-Type': 'Authorization',
		},
		body: JSON.stringify({
			username: 'user',
			password: 'hashedPassword',
		}),
	});
	
	  console.log(login_response);
	}
  </script>

<div class="uk-section uk-section-muted uk-flex uk-flex-middle uk-animation-fade" uk-height-viewport>
	<div class="uk-width-1-1">
		<div class="uk-container">
			<div class="uk-grid-margin uk-grid uk-grid-stack" uk-grid>
				<div class="uk-width-1-1@m">
					<div class="uk-margin uk-width-large uk-margin-auto uk-card uk-card-default uk-card-body uk-box-shadow-large">
						<h3 class="uk-card-title uk-text-center">Welcome back!</h3>
						<form on:submit|preventDefault={logging_service}>
							<div class="uk-margin">
								<div class="uk-inline uk-width-1-1">
									<span class="uk-form-icon" uk-icon="icon: user"></span>
									<input
										class="uk-input uk-form-large"
										type="text"
										placeholder="Username"
										bind:value={username}
									/>
								</div>
							</div>
							<div class="uk-margin">
								<div class="uk-inline uk-width-1-1">
									<span class="uk-form-icon" uk-icon="icon: lock"></span>
									<input
										class="uk-input uk-form-large"
										type="password"
										placeholder="Password"
										bind:value={password}
									/>
								</div>
							</div>
							<div class="uk-margin">
								<button class="uk-button uk-button-primary uk-button-large uk-width-1-1">
									Login
								</button>
							</div>
							<div class="uk-text-small uk-text-center">
								Not registered? <a href="#">Create an account</a>
							</div>
						</form>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>