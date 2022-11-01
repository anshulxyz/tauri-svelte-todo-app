<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  let all_tasks = [];

  async function GetAllTasks() {
    all_tasks = await invoke("get_all_tasks");
  }

  onMount(async () => {
    GetAllTasks();
  });

  let text = "";
  async function AddTask() {
    // validation
    if (text.length == 0) {
      return;
    }
    all_tasks = await invoke("add_task", { text });
    text = "";
  }
  // TODO: Use forms, so that tab works

  async function handleCheckboxInput(task_id, is_done) {
    console.log(task_id, is_done);
  }

  async function deleteTask(task_id) {
    all_tasks = await invoke("delete_task", { taskId: task_id });
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
          <input
            type="checkbox"
            name=""
            bind:checked={task.is_done}
            on:click={() => handleCheckboxInput(task.id, !task.is_done)}
          />
          {#if task.is_done}
            <s class="muted">{task.text}</s>
          {:else}
            {task.text}
          {/if}
          <button class="delete" on:click={() => deleteTask(task.id)} > Delete </button>
        </li>
      {/each}
    </ul>
  </div>
</main>
