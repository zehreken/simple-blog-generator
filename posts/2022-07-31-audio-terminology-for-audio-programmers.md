layout = "post"
title = "Audio Terminology for Audio Programmers"
created = "2022-05-07"
updated = "2022-09-16"
tags = "#audio"
markdown = """
**It** is very important to speak the same language as the sound designer in game development. As a seasoned programmer in games without much audio programming experience, learning their jargon helps me a lot. Here are some useful terms for game audio programmers.

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

**Attenuation:** The combined effect of scattering and absorption is called attenuation.

**Propagation:** This is basically the movement of sound. In games this can be more about how the sounds through portals(e.g. doors, windows).

**Diffraction:** The phenomenon in sound propagation when the sound wave moves around an object whose dimensions are smaller than or about equal to the wavelength of the sound.

**Obstruction:** Obstruction occurs when a game object partially blocks the space between the source and the listener.

**Occlusion:** Occlusion occurs when a game object completely blocks the space between the source and the listener.

**Wavelength:** Wavelength is the distance between identical points in the cycles of a waveform signal. It is the horizontal distance.

**HRTF:** Short for head *related transfer function*. It describes how an ear receives a sound from a point in space based on its physical properties, like the shape of the external ear, ear canal and also head size etc. This is also know as anatomical transfer function(ATF).

**Dry&Wet:** In audio dry refers to unprocessed audio and wet refers to processed audio.

**Early Reflections:** Early reflections are the echoes of a signal that arrive at the listener within a short amount of time, around 30ms, hence the name early reflections. Also called first reflections.

**Comb Filtering:** Happens when the same signal arrives at the listener at different times with a very small delay between signals.

**Stinger/Bumper:** Generally a short clip of music or sound that can be used to introduce, end or link various sections of an audio or audiovisual production. Think about the times when you solve a puzzle in a game or you see an enemy type for the first time.
"""