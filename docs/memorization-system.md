# Ideas

From my best understanding, SRS doesn't seem to be very scientific (and even the concept of spaced repetition may even be no better than regular repetition). Therefore, I decided to not try and implement some standard way (such as Anki) of SRS. However, while I cannot vouch for the scientific validity of SRS, I *personally* like it for learning, so a somewhat similar system will be implemented.

## Core goals

### Correct or incorrect

Many SRS systems have multiple levels of "remembered", such as Anki's "Again", "Good", "Easy" or "Very Easy", or Pleco's 6 levels (which don't quite make sense when first learning). I find myself spending a non-negligible amount of time trying to judge what level of "correct" my answer was. cedr offers a binary choice between correct and incorrect. It is up to the user to decide what threshold "correct" should be based on their individual learning goals.

> Maybe interesting thought? Optionally when marking something incorrect, we could ask for what was incorrect (e.g., tones, meaning, pinyin - because an incorrect answer is rarely **completely** wrong). In fact, in Pleco, I am doing this mentally - I mark something as "barely remembered" if I got everything but one part of the vocab right. So why not offer that in the application itself? This information can be useful as lessons thereafter can focus on what was incorrect last time (e.g. making an emphasis on tones, if tones were incorrect last time). Although an extra step adds an additional interaction (and code complexity) it may be worth looking into.

### Simple

The memorization system should not be overly complicated (as again, to the best of my knowledge, it is dubious how scientific SRS truely is, so there is little point trying to replicate a system that may be overly complex with no benefit).

A (customizable?) curve can be described to which flashcards will be designated. solving for Y which is number of hours until next repetition.

```
d = n number hours (default 12)
x = consecutive correct answers
y = (2^x * d)
```

A goal score can be set, either by time to completion, or by number of consecutive correct answers. For example, 8 repetitions would take 255 days to complete and be considered "burned".

### Character focus

Cards have four states: inactive, active, locked, and burned. Locked cards can still be learned, but are called as such because they have unmet character dependencies. This application encourages learning characters to build up compound character words, as learning individual meanings is often very helpful later on. cedr will optionally automatically hide locked cards until their dependencies are met. Once all dependencies are added to the learning queue (in an "active" state) compound words using those characters will be unlocked. States do not affect learning curve weights - the curve is solely dictated by consecutive correct answers.

> Could be cool to have a dependency tree visualization of characters in HSK.

### 好看

Interactions should not leave users with a sense of dread and boredom. Gamification is a stupid buzzword but nice interaction and polish can help make an application more engaging and fun to use. This is a weird one to include here in the memorization system, but I do all my learning at night and sometimes staring at a deep blue unfeeling uncaring UI can make me sleepy.
Any memorization system is ineffective if the implementing program does not hold attention. For example, consider breaking up a review with hundreds of items into checkpoints with 50 items each.

### Deck independent

Just a sidenote, card scores should be owned by the entry itself, and not a collection. If two collections have the same cards, working in either collection should continue progress, as there's no real reason not to do this if our goal is memorization.

### Audio focus

Audio is really important for learning languages. TBD on where to get said audio, but cards should always play it on reveal (and audio quizzes should also be available).

### Self moderated

Wanikani is cool but it's impossible to be accurate 100% of the time unless you're trusting the user to self-moderate. Slipping your finger and typing one letter wrong and getting the card progress reset really sucks to the point where I often would just quit the session without saving.

