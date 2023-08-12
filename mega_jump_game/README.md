# MEGA JUMP
In this game, the player has to jump between platforms of varying sizes to reach the top. The platforms have side to side movement and the distance between each platform and size of the platform changes as the player goes up. There are clouds that impede the player's vision to add some sort of difficulty.
## The Making of Mega Jump
When I first started creating this game, I had a wildly different idea of what the game would look like than what it has become now. My inspiration for MEGA JUMP was Doodle Jump, where there would be and infinite number of platforms and the game would keep score for how far the player went vertically. In my version of MEGA JUMP, although there isn't a score system, there is a end the level where the difficulty is the highest.

At first, I was using just Bevy to create my game, and I was able to spawn a character that could move around. I limited the screen space and made a camera system where the player would be at the bottom of the screen and be able to see what was above. Using clouds textures, I placed them infront of the player and the platforms so that it would impede the vision of the player. I also was able to spawn platforms in the game world. However, these platforms and the player character would not interact with each other (collision) and there was no gravity in the game so the the player would just move up vertically and not stop.

I then implemented Bevy-Rapier, a physics engine in Bevy, to accomplish collisions and gravity in my game. Since the game felt too easy to just jump between static platforms, I implemented a random movement system for the platforms where they would translate in the X direction with varying ranges. 

One struggle I had with Bevy-Rapier was implementing a **jump** action. Even though I was using a CharacterController, with a flag that would check if the Player was grounded, my game never changed the flag to True for grounded unless tha player would move vertically down. Obviously, platformer game like this would not feel right with a down movement action so I had to resort to limiting the jump impulse. Right now, the jump action is not working as intended.

I also attempted to add music to the game however, when the music plays, the game physics become a mess (can be seen by UNCOMMENTING the *debug_Bevy-Rapier* and *music_startup*). It also does not play the music well as it is just stuttering. 

Some improvements that I want to make to MEGA JUMP are cleaning up the code and separating each major system to their own file. Since I had to manually create each platform, there is a lot of copy pasted code in the *spawn_map* system which looks messy and hard to navigate through the code. 

Regardless of all the trouble and struggles in this project, I am proud of what I was able to accomplish, making a game using an ECS game engine where all the systems are created in the code. Having game aspects gradually start working really was exciting as there would always be progress. Unfortunately, there were a lot of unfinished systems in the game that I wish would have worked but I spent a lot of time on them and had given up, like Main Menu screen, jump impulse, grounded flag checking, music, and animation.

## Demo Video
https://youtu.be/U3aid97G5uI

