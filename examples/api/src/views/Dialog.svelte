<script>
  import { open, save } from '@tauri-apps/api/dialog'
  import { readBinaryFile } from '@tauri-apps/api/fs'

  export let onMessage
  export let insecureRenderHtml
  let defaultPath = null
  let filter = null
  let multiple = false
  let directory = false

  function arrayBufferToBase64(buffer, callback) {
    const blob = new Blob([buffer], {
      type: 'application/octet-binary'
    })
    const reader = new FileReader()
    reader.onload = function (evt) {
      const dataurl = evt.target.result
      callback(dataurl.substr(dataurl.indexOf(',') + 1))
    }
    reader.readAsDataURL(blob)
  }

  function openDialog() {
    open({
      title: 'My wonderful open dialog',
      defaultPath,
      filters: filter
        ? [
            {
              name: 'Tauri Example',
              extensions: filter.split(',').map((f) => f.trim())
            }
          ]
        : [],
      multiple,
      directory
    })
      .then(function (res) {
        if (Array.isArray(res)) {
          onMessage(res)
        } else {
          const pathToRead = res
          const isFile = pathToRead.match(/\S+\.\S+$/g)
          readBinaryFile(pathToRead)
            .then(function (response) {
              if (isFile) {
                if (
                  pathToRead.includes('.png') ||
                  pathToRead.includes('.jpg')
                ) {
                  arrayBufferToBase64(
                    new Uint8Array(response),
                    function (base64) {
                      const src = 'data:image/png;base64,' + base64
                      insecureRenderHtml('<img src="' + src + '"></img>')
                    }
                  )
                } else {
                  onMessage(res)
                }
              } else {
                onMessage(res)
              }
            })
            .catch(onMessage(res))
        }
      })
      .catch(onMessage)
  }

  function saveDialog() {
    save({
      title: 'My wonderful save dialog',
      defaultPath,
      filters: filter
        ? [
            {
              name: 'Tauri Example',
              extensions: filter.split(',').map((f) => f.trim())
            }
          ]
        : []
    })
      .then(onMessage)
      .catch(onMessage)
  }
</script>

<div class="flex gap-2 children:grow">
  <input
    class="input"
    id="dialog-default-path"
    placeholder="Default path"
    bind:value={defaultPath}
  />
  <input
    class="input"
    id="dialog-filter"
    placeholder="Extensions filter, comma-separated"
    bind:value={filter}
  />
</div>

<br />
<div>
  <input type="checkbox" id="dialog-multiple" bind:checked={multiple} />
  <label for="dialog-multiple">Multiple</label>
</div>
<div>
  <input type="checkbox" id="dialog-directory" bind:checked={directory} />
  <label for="dialog-directory">Directory</label>
</div>
<br />
<button class="btn" id="open-dialog" on:click={openDialog}>Open dialog</button>
<button class="btn" id="save-dialog" on:click={saveDialog}
  >Open save dialog</button
>
