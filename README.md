# Ticket Tycoon
Ashton Sawyer

## Description
A clicker/idle game based on an IT organization and its help desk.

## Running
All testing for this game was done with Dioxus serving to the desktop. Because
of this, Dioxus has been imported so that `cargo run` will launch the desktop 
version of the app. 

If you want to try to run this on a different platform, you can install the
Dioxus CLI (instructions found 
[here](https://dioxuslabs.com/learn/0.7/getting_started/)) and serve the app
using `dx serve --platform <platform>`

## Dev Notes

### Types
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

### Dioxus
I thought about different GUIs for a long time. I had a handful of suggestions
and wasn't sure what to go with. I used ChatGPT to get a super rough outline
of a GUI in each of the libraries so I could get an idea of what they looked
like, how well I could read their code, and how easy they were to fix. I ended
up going with Dioxus because it was familiar. I've used React in the past and
the structure makes sense to me, so writing it was intuitive. I'm also 
fairly comfortable with CSS, so using it to style components was convenient. 
It's also very well documented and pretty easy to google things for, which is
nice. 

### Effects: Struct or Enum
Originally I had my upgrades be a struct because it seemed simple and quick to
implement. I ended up changing to and enum because of the autosolve effect. If
I had kept effects as a struct, I would have had to use some sort of Option to
deal with upgrades that aren't supposed to do any autosolving and that seemed 
like more trouble than it was worth. 

### async
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

### Finishing Tickets
I was getting something where when a ticket is being worked on, it would get to
100% and need to be clicked one more time before it would be finished. There
was also something where autosolved tickets would need something in the ticket
queue to be clicked on before they would close after they got to 100%. If *any*
ticket were worked on, all autosolved tickets at 100% would close. 

The first problem is because I have a goal for each ticket, which is used to 
calculate the completion percent, but the check for `is_complete()` was looking
for strictly greater than. The second problem was because there isn't a check
for completion in the autosolve function, and the addendum to the second 
problem was because any completed tickets were removed on every call to
`click_ticket()`.  

```rust
// Original Code
pub fn click_ticket(&mut self, index: usize) {
    if let Some(ticket) = self.working.get_mut(index) {
        ...
        if ticket.is_complete() {
            ...
        }
    }
    // Remove finished tickets
    self.working.retain(|t| !t.is_complete());
}

// New Code
pub fn click_ticket(&mut self, index: usize) {
    ...
    if ticket.is_complete() {
        ...

        // Remove finished tickets
        let _ = self.working.remove(index);
    }
}
```

However, I kept the other two pieces. I could make it so that a ticket 
automatically closes after it's completed, but I found that it makes it hard to
keep clicking because all tickets below it will be shifted up in the queue, 
causing misclicks. To make it make sense to the player, I just have the text on
the button change from `Work` to `Close` when the progress bar percentage is at
100. 

### Game State Constants
When I added the `IncCashMultiplier` and `IncXPMultiplier` upgrade effects, I 
decided to change the base values to constants rather than only having them in
`click_ticket()`'s code. I did this mostly for making the unit tests more 
robust. I know that the game I've made, as I currently have it, isn't very 
balanced and that these base values should probably be changed for a better
game experience. If/When that happens, the unit tests would have to be updated.
In order to only have to change the values in one place, I changed then to 
`const`s.