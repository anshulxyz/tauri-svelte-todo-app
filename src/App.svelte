<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let all_tasks = [];

  async function GetAllTasks() {
    all_tasks = await invoke("get_all_tasks");
    // set the values in the store
    console.log(typeof all_tasks);
  }

  onMount(async () => {
    GetAllTasks();
  });

  let text = "";
  async function AddTask() {
    all_tasks = await invoke("add_task", { text });
  }
</script>

<main class="container">
  <div class="row">
    <input id="greet-input" placeholder="Enter a task..." bind:value={text} />
    <button on:click={AddTask}> Add </button>
  </div>

  <div class="row" style="text-align:left;">
    <ul>
      {#each all_tasks as task}
        <li class="row">
          <input type="checkbox" name="" id="{task.id}" bind:checked={task.is_done}>
          {task.id} - {task.text}
        </li>
      {/each}
    </ul>
  </div>
</main>
