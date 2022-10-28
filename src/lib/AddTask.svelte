<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let text = "";
  let message = {}

  async function AddTask(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    message = await invoke("add_task", { text })
    console.log(typeof message.id);
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a task..." bind:value={text} />
    <button on:click={AddTask}>
      Add
    </button>
  </div>
  <p>{ JSON.stringify(message) }</p>
</div>
