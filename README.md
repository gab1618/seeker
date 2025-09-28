## Seeker

A git repository provider that indexes through your documents and send them to a search-optimized database. In this case, the goal is to index it into an Elastic Search index.

**Disclaimer**: as you can probably notice, this is still work in progress. Of all the features I planned, none of them are implemented yet, but you can always have fun looking at my messy code.

## Running

### Pre-requisites

1. Podman (refer to #26)

### Configuration

1. Use the setup script to create all the necessary files: `make setup`
2. Generate a ssh key pair just like how you do for Github ssh access.
3. Put your generated pub key in `.seeker/.ssh/authorized_keys` (this file was created by the setup script)
4. Run `chmod 600` for the authorized_keys file.

These steps are only necessary when running the container. Building doesn't require any prior setup

### Building

After all the configuration, all you got to do is use the command `make build`.

### Running the image

With the image already built, run it with `make run`. It will create a container named "seeker" and bind its port 22 to port 2222.

After that, you should be able to interact with this repository just like how you do in Github. In my personal setup, I created a ssh key with a hostname of `localhost` and user `git`, so the command to add the remote looks like this:

```sh
git remote add origin ssh://git@localhost:2222/repo/seeker.git
```

After the container started, you can watch the daemon logs with:
```bash
make watch-daemon-logs
```
