<script lang="ts">

console.log("control log");
export let load;

</script>

    <!-- Kontener z danymi użytkownika -->
    <div class="uk-container uk-margin-top">
        <h1 class="uk-heading-medium">Jan Kowalski</h1>
        <p class="uk-text-lead">Programista Webowy</p>
        <div class="uk-grid-match uk-child-width-expand@s" uk-grid>
            <div>
                <div class="uk-card uk-card-default uk-card-body">
                    <h3 class="uk-card-title">Informacje Kontaktowe</h3>
                    <ul class="uk-list uk-list-divider">
                        <li><strong>Email:</strong> jan.kowalski@example.com</li>
                        <li><strong>Telefon:</strong> +48 123 456 789</li>
                        <li><strong>Strona WWW:</strong> <a href="https://jan-kowalski.pl" target="_blank">jan-kowalski.pl</a></li>
                    </ul>
                </div>
            </div>

            <div>
                <div class="uk-card uk-card-default uk-card-body">
                    <h3 class="uk-card-title">O mnie</h3>
                    <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur vehicula, nulla ac convallis cursus, felis mi suscipit turpis, sit amet tristique lorem libero at eros.</p>
                </div>
            </div>
        </div>

        <!-- Aktywności użytkownika -->
        <div class="uk-card uk-card-default uk-card-body uk-margin-top">
            <h3 class="uk-card-title">Ostatnie Aktywności</h3>
            <ul class="uk-list uk-list-divider">
                <li>Opublikowano nowy artykuł: <a href="#">Jak zoptymalizować kod JavaScript</a></li>
                <li>Dodano komentarz do projektu: <a href="#">React vs Angular</a></li>
                <li>Dołączył do grupy: <a href="#">Front-End Developers</a></li>
            </ul>
        </div>
    </div>