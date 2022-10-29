<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let text = "";

  async function AddTask() {
    await invoke("add_task", { text }).then(() => {
    });
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a task..." bind:value={text} />
    <button on:click={AddTask}> Add </button>
  </div>
</div>
