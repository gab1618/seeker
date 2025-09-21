## Seeker

A git repository provider that indexes through your documents and send them to a search-optimized database. In this case, the goal is to index it into an Elastic Search index.

**Disclaimer**: as you can probably notice, this is still work in progress. Of all the features I planned, none of them are implemented yet, but you can always have fun looking at my messy code.

## Running

### Pre-requisites

1. Docker

### Configuration

1. First of all, generate a ssh key pair just like how you do for Github ssh access.
2. In config/.ssh, create a file named `authorized_keys`, and put you public key in there. In case you want to put multiple keys, just separate them by a linebreak.
3. Run `chmod 600` for the authorized_keys file.

### Building

After all the configuration, all you got to do is use the command `docker build`.

### Running the image

With the image already built, run it with `docker run`, mapping the port `22` and binding ssh config:

```sh
docker run -p 2222:22 -it -v ./config/.ssh:/repo/.ssh seeker
```

After that, you should be able to interact with this repository just like how you do in Github. In my personal setup, I created a ssh key with a hostname of `localhost` and user `git`, so the command to add the remote looks like this:

```sh
git remote add origin ssh://git@localhost:2222/repo/seeker.git
```

Inside the container, you can access the logs with:

```bash
tail -F /repo/seeker.git/info/log
```
