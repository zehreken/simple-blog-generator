layout = "post"
title = "Audio Terminology for Audio Programmers"
created = "2022-05-07"
updated = "2022-09-16"
tags = "#audio"
markdown = """
I decided to write this post since I'll be an audio programmer in a few months
time. It is very exciting but also a little scary since I have no professional
experience with audio. As an audio programmer, if you want your fellow audio
designer love you, you should know these terms.

**dB(decibel):** A unit to measure 'sound pressure level'. It is also a thing in
logarithms.

**RMS:** Root mean square. It is basically the average, but for continuous
signals, such as audio or electrical signals. Measured in dBs.

**Peak:** Maximum value of an audio signal. Measuered in dBs.

**Buffer underrun:** In audio context, a buffer underrun occurs when there are no
more samples to read in the audio buffer. This can happen when the processing thread
consumes the samples faster than the audio input device can produce.

**Buffer overrun:** In audio context, similar to buffer underrun, buffer overrun occurs
when the buffer is filled faster than the processing thread consumes.
"""