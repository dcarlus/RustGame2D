# Yet Another Platform Game (unnamed game)

This is an unnamed game for now.

The main goal of this project is to discover several things, among which the most important ones are:
* Rust language;
* ECS architecture (and so data-driven programmation).

The game will use the [Amethyst engine](https://amethyst.rs/).

## Main gameplay ideas
This is a solo game, no multiplayer planned.

There are enemies that can damage the player. The player can damage the enemies as well.
There is no character background, nor story (for now), nor charadesign or whatever, it is not important for now.

The player (and some enemies) will be able to have weapons and armors. Here are the details of the gameplay that are drawn for now, they could evolve progressively if needed.

### Movements
* Walk/run/sprint
* Jump
    * Height depends on the pressure on the button
    * Length depends on the speed of the character
    * The player loses health if falling from too high
* Climb (ladder, rocks, foliage, other)
* Crawl
* Swim and dive

### Fights
* Real-time
* Enemies live in their 'area':
    * Common bots only (re)spawn if the player is far enough from the area
        * Bosses can make spawn new bots for assist
    * No predefined pattern in their movement (based on player position)
    * Natural speed
    * AI is prone to errors (ie. jump, fall and die)
    * AI only follows the player in this area
    * AI has some kind of 'personality':
        * AI can call allies in the area if in danger
        * AI can heal itself or another bot if in danger

#### Player skills
* Single or two hand weapons
* Hand-to-hand gameplay: dodge, block (shield or weapon), feint on visual signals
* Distance gameplay: targeting

### Equipment and inventory
* Enemies leave resources
* Resources can be crafter to create/repair:
	* weapons and ammunition,
	* armors,
	* miscellaneous items (health, mana?...)

### Player evolution
* RPG style to let the character gain experience when killing enemies
* The more the character do an action, the more it is better in that task. Do not impact essential elements for evolving in the game (craft with less resources, ...)
* Enemies strength adjusted to the player level
