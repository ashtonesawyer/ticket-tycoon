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