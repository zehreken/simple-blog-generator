layout = "post"
title = "Fine-tuning SpeechT5 to Imitate Me"
created = "2025-11-04"
updated = "2025-11-04"
tags = "#artificial-intelligence #machine-learning"
markdown = """
Machine learning has changed the world in the last couple of years. I was a consumer of these models but only a spectator when it came to understanding how they worked. In this post I’ll share my experience trying to train the SpeechT5 text-to-speech model to imitate my own voice.

As I often do nowadays I started by asking questions to ChatGPT, Grok, and Claude. They are great tools for learning. I also familiarized myself with Huggingface.com. If you haven’t used it before it’s like GitHub but for machine learning models.

I tried a few different models at first, namely Coqui, SpeechT5, and others I’ll mention later. I chose SpeechT5 because the instructions were clearer and the dependencies fewer than the others.

To make this work I had to record myself first. I asked Claude to generate a table of filenames and transcriptions. I recorded my voice with Audacity, then wrote a Python script to process the recordings. It’s best to trim silence, make the audio mono, normalize it, and resample it to 16 kHz. Since I’m comfortable with signal processing, that part was easy, and Python’s audio libraries made it even smoother. It’s easy to see why Python is such a popular language.

The next step was creating the speaker embedding. A speaker embedding is basically an array of floating-point numbers representing someone’s voice characteristics—in this case mine.

Finally came training the model. My first attempt was surprisingly quick since I ran it on the GPU, and after hours of preparation I was eager to hear the result. Unfortunately, the model had collapsed and produced only a horrible noise. I tried again with a much lower learning rate, watching the loss value gradually decrease, feeling hopeful. But the model collapsed again, generating the same noise. I later learned that it’s not possible to train a model like SpeechT5 properly with only ten minutes of audio.

After some research I found out that instead of training the full model, I could fine-tune it using my speaker embedding. Since SpeechT5 already knows how to speak English, I could influence it to use my vocal characteristics. This approach worked better—the generated audio was still noisy and robotic compared to the model’s original voices, but it definitely resembled mine and captured my speech rhythm.
"""