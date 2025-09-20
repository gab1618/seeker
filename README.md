## Seeker

A git repository provider that indexes through your documents and send them to a vector database.

**Disclaimer**: as you can probably notice, this is still work in progress. Of all the features I planned, none of them are implemented yet, but you can always have fun looking at my messy code.

## Running

### Pre-requisites

1. Docker

### Configuration

1. First of all, generate a ssh key pair just like how you do for Github ssh access.
2. At the root of the repository, create a file named "authorized_keys" and copy your generated public key into it. Each public key you might want to add to it must be separated by a line break.

### Building

After all the configuration, all you got to do is use the command `docker build`.

### Running the image

With the image already built, run it with `docker run`, mapping the port `22` to the port `22`:

```sh
docker run -p 22:22 seeker 
```

After that, you should be able to interact with this repository just like how you do in Github. In my personal setup, I created a ssh key with a hostname of `localhost` and user `git`, so the command to add the remote looks like this:

```sh
git remote add origin ssh://git@localhost/repo/seeker.git
```
