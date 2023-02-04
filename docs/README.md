# Azumi Documentation


### Structure

#### Sub-Classing

> Azumi

Azumi is subclassed from `discord.ext.bridge.Bot`, as stated within a blog post written by AquaQuokka.
This allows the bot to be more customizable.

#### Cogs

> Azumi

Azumi's commands are separated into several cogs to reduce the amount of stress put on the bot while online, and to make development of the bot easier.

### Commands

#### Internal

##### Shutdown

`/shutdown` is a developer-only command that allows a user with a specific code generated when the bot comes online, to shut down the bot in case of an emergency.

> The `/shutdown` command is not inside of a cog due to circular import errors.

##### Pronouns

`/pronouns` is a command that allows users to add pronoun roles to themselves in the official Azumi Development server. It is currently in beta.

> The `/pronouns` command is not inside of a cog because it is still in development.

##### Azumi

`/azumi` is an information command for learning about the bot.

> The `/azumi` command is not inside of a cog due to circular import errors.

##### Weather

`/wx` is an information command that allows you to get the weather for any city in the world. It uses the `python-weather` API to grab weather data from [wttr.in](https://wttr.in).

> The `/wx` command is not inside of a cog due to circular import errors.

#### botinfo.py

> `botinfo.py` is one of many cogs that are attached to Azumi. It is subclassed from `commands.Cog` as stated in a blog article by AquaQuokka.
The `botinfo.py` cog is used for commands that provide information related to Azumi.

##### Guilds

`/guilds` is a command that grabs and sends the amount of guilds that the Azumi bot that you are interacting with is in.

##### Contributor

`/contributor` is an information command that shows information about people that have contributed to the development of Azumi.

#### dice.py

> `dice.py` is one of many cogs that are attached to Azumi. It is subclassed from `commands.Cog` as stated in a blog article by AquaQuokka.
The `dice.py` cog is used for commands that allow you to roll a die.

##### Roll

`/roll` is a command group that includes commands that allow you to roll dice.

###### Six

`/roll 6` (known as `six` in the code) is a command that allows you to roll a six-sided die.

###### Twenty

`/roll 20` (known as `twenty` in the code) is a command that allows you to roll a twenty-sided die.

###### Twelve

`/roll 12` (known as `twelve` in the code) is a command that allows you to roll a twelve-sided die.

###### Ten

`/roll 10` (known as `ten` in the code) is a command that allows you to roll a ten-sided die.

###### Four

`/roll 4` (known as `four` in the code) is a command that allows you to roll a four-sided die.

###### Eight

`/roll 8` (known as `eight` in the code) is a command that allows you to roll an eight-sided die.

#### funcmds.py

> `funcmds.py` is one of many cogs that are attached to Azumi. It is subclassed from `commands.Cog` as stated in a blog article by AquaQuokka.
The `funcmds.py` cog is used for commands that are not supposed to be taken seriously, but as fun features.

##### Hug

`/hug` is a command that allows you to hug another member of the server you use the command in, and sends a random GIF of a hug.

#### mathcmds.py

> `mathcmds.py` is one of many cogs that are attached to Azumi. It is subclassed from `commands.Cog` as stated in a blog article by AquaQuokka.
The `mathcmds.py` cog is used for commands that calculate equations.

##### Math

`/math` is a command group that allows you to calculate equations.

###### Add

`/math add` is a command that can do simple addition between two `float` numbers.

###### Subtract

`/math subtract` is a command that can do simple subtraction between two `float` numbers.

###### Multiply

`/math multiply` is a command that can do simple multiplication between two `float` numbers.

###### Divide

`/math divide` is a command that can do simple division between two `float` numbers.

###### Squareroot

`/math squareroot` is a command that can find the square root of a `float` number.

###### Power

`/math power` is a command that can index a `float` number by another `float` number.

###### Percent

`/math percent` is a command that can find a `float` percentage of a `float` number.

#### messagecmds.py

> `messagecmds.py` is one of many cogs that are attached to Azumi. It is subclassed from `commands.Cog` as stated in a blog article by AquaQuokka.
The `messagecmds.py` cog is used for message commands.

##### getmid

`Get Message ID` (known as `getmid` in the code) is a message command that gets the Message ID of the selected message.

##### reportmessage

`Report Message` (known as `reportmessage` in the code) is a message command that uses a webhook to send a reported message to the developers of Azumi.

#### moderation.py

> `moderation.py` is one of many cogs that are attached to Azumi. It is subclassed from `commands.Cog` as stated in a blog article by AquaQuokka.
The `moderation.py` cog is used for commands that help moderate servers.

##### Timeout

`/timeout` is a command that allows a user with the `moderate_members` permission (this is checked on usage of the command) to timeout a member that has a lower role in the hierarchy than them for up to 28 days. The bot must also have the `moderate_members` permission for this to work.

#### postboards.py

> `postboards.py` is one of many cogs that are attached to Azumi. It is subclassed from `commands.Cog` as stated in a blog article by AquaQuokka.
The `postboards.py` cog is used for commands that post messages to the message boards in servers that have message boards enabled.

##### Post

`/post` is a command group that allows users to post messages to message boards.

###### Textboard

`/post textboard` is a command that allows users to post plaintext to the textboard in the official Azumi Development server.

###### Imageboard

`/post imageboard` is a command that allows users to post text and a single image to the imageboard in the official Azumi Development server.

> Post Commands

Azumi will automatically add the set default reactions in that server to each post in the postboards of that server.

#### utilitycmds.py

> `utilitycmds.py` is one of many cogs that are attached to Azumi. It is subclassed from `commands.Cog` as stated in a blog article by AquaQuokka.
The `utilitycmds.py` cog is used for commands that are used for utility and quality of life.

##### Report

`/report` is a command that allows you to report users, bugs, or anything that concerns the developers of Azumi, to the developers of Azumi.

The following report types are:

- Bot Abuse
- User Report
- Bug
- Exploit
- Other

##### Member

`/member` is a command that grabs information about a member.

It shows the following information:

- When the account was created.
- When they last joined the current server.
- Their current discriminator.
- Their ID.
- Their dominant role. (The one highest in the role hierarchy.)
- A list of all the roles they have.
- How many roles they have.
- A clickable mention that shows their profile.

##### Find

`/find` is a command that grabs a message and its data from a message ID.

##### Timestamp

`/timestamp` is a command that generates all forms of timestamps for the current time.

##### GitHub

`/github` is a command group that grabs information about GitHub issues and pull requests. It will soon be replaced by the Pullfrog bot made by AquaQuokka.

###### Issue

`/github issue` is a command that grabs a specific issue from a selected repository.

###### Pull

`/github pull` is a command that grabs a specific pull request from a selected repository.

#### usercmds.py

> `usercmds.py` is one of many cogs that are attached to Azumi. It is subclassed from `commands.Cog` as stated in a blog article by AquaQuokka.
The `usercmds.py` cog is used for user commands.

##### memberclick

About Member (known as `memberclick` in the code) is a user command that gets information about the selected member. It is the User Command variant of `/member`.


# End of Documentation
