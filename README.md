# Ticket Tycoon
Ashton Sawyer

# Description
A clicker/idle game based on an IT organization and its help desk.

# Running
This was tested using `dx serve --platform desktop`

# Dev Notes

## Types
The different data types came together fairly seamlessly. The basis of a game
about completing help desk tickets is the ticket itself, so I started there. 
Then I needed something to buy upgrades with, so there's the currency. 
Finally, for the most basic form of the game I needed some sort of game state,
so I did that. After those three, there is something "playable," even if it's 
not very engaging. 

The next step for me was to work on the GUI. I know that's probably not the most
logical next step, but in order to figure out what other functionality was 
needed I had to be able to see the button that the user would be pressing. 

After I got a basic GUI off the ground, I started working on the upgrades.

## Dioxus
I thought about different GUIs for a long time. I had a handful of suggestions
and wasn't sure what to go with. I used ChatGPT to get a super rough outline
of a GUI in each of the libraries so I could get an idea of what they looked
like, how well I could read their code, and how easy they were to fix. I ended
up going with Dioxus because it was familiar. I've used React in the past and
the structure makes sense to me, so writing it was intuitive. I'm also 
fairly comfortable with CSS, so using it to style components was convenient. 
It's also very well documented and pretty easy to google things for, which is
nice. 

## Effects: Struct or Enum
Originally I had my upgrades be a struct because it seemed simple and quick to
implement. I ended up changing to and enum because of the autosolve effect. If
I had kept effects as a struct, I would have had to use some sort of Option to
deal with upgrades that aren't supposed to do any autosolving and that seemed 
like more trouble than it was worth. 

## async
I spent at least two and a half hours trying to debug a problem that I thought
was happening with my async `autosolve` code but was actually caused by values
in `upgrades.json`. 

Essentially, I was running into a strange error where my game would work as
expected up until I bought the first autosolve upgrade. After that, the
autosolve would be working, but clicking the `work` buttons on the tickets did
nothing. I thought it might be related to locks or signal updates not 
propogating correctly and spent a very long time trying different 
configurations to see what might work. Eventually, I commented out all of my 
async code and started playing the game again just because I needed something
to take my mind off of the immediate problem. When I bought the first autosolve
upgrade, again my tickets stopped working. This meant that the problem couldn't
be in the async code. I looked through my autosolve method for the game struct
and didn't see anything wrong with it. 

I decided to turn on dioxus logging so I could try and see if the struct was 
being updated incorrectly. A part of printing the struct was also printing 
all of the upgrades, and I noticed on some of them `IncMultiplier(0.0)`. 
This was leftover from when I was adding to the multiplier, and also a little
from when effects was a struct and not an enum. It multiplied my click 
multiplier by 0, rendering all clicks absolutely useless. 

I don't think I would have ever though on my own to look at `upgrades.json` and
make sure that it was as expected. So here's a reminder to thoroughly update
data files when code changes happen to save yourself and *incredibly* 
frustrating evening. 