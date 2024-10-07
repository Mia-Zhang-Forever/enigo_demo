## Run

```
pnpm tauri dev
```

## How to validate?

When you successfully build it, a board will open, and two global shortcuts will be registered.
* `ctrl+cmd+c`: open the board window and make it always on-top (you can close the board window first and use shortcut to open it again)
* `ctrl+cmd+n`: copy current selection and then delete it

## To reproduce the bug, follow these steps:

* Type a random word on the board.
* Select the text you just typed.
* use shortcut `ctrl+cmd+n` to try to copy it and delete it
* You can see that the copied content is not the text you just typed, and the deletion occurs after the sleep loop (check the log in the terminal).

## What is the correct behavior?
* open apple notes or any editor (not in the tauri board), do the same 
* You can see that the copied content is the text you just typed.
