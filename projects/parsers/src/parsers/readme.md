

Enable or disable long prompt weighting.
Long prompt weighting enables prompts beyond the typical 77 token limit of the CLIP tokenizer, and allows for emphasizing or de-emphasizing certain parts of the prompt. The weighting syntax and values are the same as AUTOMATIC1111's implementation:
a (word) - increase attention to word by a factor of 1.1
a ((word)) - increase attention to word by a factor of 1.21 (= 1.1 * 1.1)
a [word] - decrease attention to word by a factor of 1.1
a (word:1.5) - increase attention to word by a factor of 1.5
a (word:0.25) - decrease attention to word by a factor of 4 (= 1 / 0.25)
a \(word\) - use literal () characters in prompt
With (), a weight can be specified like this: (text:1.4). If the weight is not specified, it is assumed to be 1.1. Specifying weight only works with (), but not with [].
If you want to use any of the literal ()[] characters in the prompt, use the backslash to escape them: anime_\(character\).
This method of weighting is slightly different to NovelAI's; NAI uses 1.05 as the multiplier and {} instead of ():

- NAI {word} = PD (word:1.05)
- NAI {{word}} = PD (word:1.1025)
- NAI [word] = PD (word:0.952)
- NAI [[word]] = PD (word:0.907)

