<script lang="ts">
  import Router from 'svelte-spa-router';
  import '../store/notes';
  import EditNote from './EditNote.svelte';
  import NewNote from './NewNote.svelte';
  import Notes from './Notes.svelte';
  import SidebarLayout from './SidebarLayout.svelte';

  const routes = {
    '/': NewNote,
    '/notes': Notes,
    '/notes/:id': EditNote,
  };
</script>

<SidebarLayout>
  <Router {routes} />
</SidebarLayout>
