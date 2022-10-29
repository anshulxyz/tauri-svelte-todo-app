<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let all_tasks = [];

  async function GetAllTasks() {
    all_tasks = await invoke("get_all_tasks");
    console.log(typeof all_tasks);
  }

  onMount(async () => {
    GetAllTasks();
  });
</script>

<div>
  <div class="row">
    <ul>
      {#each all_tasks as task}
        <li class="row">{task.id} - {task.text} - {task.is_done}</li>
      {/each}
    </ul>
  </div>
</div>
