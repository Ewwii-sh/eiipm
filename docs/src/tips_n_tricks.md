# Tips & Tricks

Here are some tips and tricks which you can use to make your experience with eiipm better.

## Confirm all confirmation requests

As you may know, eiipm sometimes require user confirmation to perform certain tasks. One of the examples where eiipm asks for confirmation is in the update task when it cant resolve merge.

You can [pipe](https://www.baeldung.com/linux/pipe-output-to-function#introduction) the `yes` command to answer "y" to all eiipm commands.

**Example:**

```bash
yes | eiipm update
```
