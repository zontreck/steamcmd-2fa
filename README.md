# steamcmd-2fa

A simple tool that automatically generates a 2FA Steam Guard code and then runs `steamcmd` with that login and the rest of your arguments.

## Installation

You can either clone this and build it yourself or use the supplied binaries for both Windows and Linux.

```bash
git clone https://github.com/zontreck/steamcmd-2fa
cd steamcmd-2fa
cargo build
```

## Usage
```
steamcmd-2fa 0.2.0

USAGE:
    steamcmd-2fa [OPTIONS] --secret <SECRET>

OPTIONS:
    -a <ARGS>                [default: +quit]
    -h, --help               Print help information
    -p <PASSWORD>            [default: ]
        --path <PATH>        [default: /home/steam/steamcmd]
        --raw                
    -s, --secret <SECRET>    
    -u <USERNAME>            [default: ]
    -V, --version            Print version information
```

For example, instead of running `steamcmd +login exampleuser examplepass +quit`, you would run `steamcmd-2fa --path /home/steam/steamcmd --username exampleuser --password examplepass --secret YOURSECRET --args "+quit"`. 

You can get your 2FA seed by [various methods](https://github.com/SteamTimeIdler/stidler/wiki/Getting-your-%27shared_secret%27-code-for-use-with-Auto-Restarter-on-Mobile-Authentication). Your seed here is the `shared_secret`.

**Please keep your seed/secret VERY safe as people with your seed can easily bypass your 2FA by simply generating the Steam Guard codes themselves!**
*I am not responsible if you somehow manage to lose/get yourself locked out of your account.*

## Contributing
Pull requests are welcome. 
## License
[MIT](https://choosealicense.com/licenses/mit/)
