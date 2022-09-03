layout = "post"
title = "How modul is made?"
created = "2022-05-07"
updated = "2022-05-07"
tags = "#audio"
markdown = """
# Find a better title
https://tesselode.github.io/articles/audio-libraries-considered-challenging/

###  Common problems that lead to latency
It is probably about the buffer size. In modul, I didn't know how to set buffer sizes for the
hardware. For months I had the default buffer size for my audio interface, 512 per channel.
In this case it is easy to calculate latency, 512 / 44.1 kHz = 11.7 ms which is considered high
for real-time audio and it was high. When I tried to keep a beat over the sampled audio, I
always missed the beat no matter how hard I tried. Then I found out that I can set the buffer size
in cpal and I set it to 128 and it fixed the latency issue. But stupidly enough I was using the
same configuration for creating input and output streams and this effectively set output buffersize
to 128 as well. This created a bunch of issues, first audio processing thread designed to process
more samples than 128 every update and this caused buffer overruns and glitches. I set output
buffersize to 2048 and there was significant improvement with latency.

### Using Arc<Mutex> vs Channel vs RingBuf
"even if the mutex is not locked by another thread: locking/unlocking a mutex in a realtime context
can lead to the OS rescheduling your thread and stealing your timeslice." **WeirdConstructor**

"get_sample_averages() is called multiple times in the GUI routine, and each call is a Mutex lock.
the mutex lock might cause problems, even though the critical zone is relatively short as you are
copying the TAPE_COUNT samples immediately" **WeirdConstructor**

```self.writing_tape.push(sample);``` regarding this line "it's safer to write recorded samples to a
ring buffer and have a dedicated thread do the storage and allocation" **WeirdConstructor**

Regarding the key_receiver "that Receiver is an mpsc::Receiver, which is internally using Mutexes
again. even though you call try_iter(), it will call unlock on that inner Mutex at some point,
which can mean that the OS kernel takes away your timeslice from the audio processing thread again
and gives it to the waiting GUI thread" **WeirdConstructor**

While adding a perspective camera, I realised that given a = mat4 and b = vec4, a * b is not equal
to b * a. :D It took me almost 2 days to debug the weird shapes and this shows how rusty I am with
basic math. I need to get better and better and better.

### Visualising Tape as Waveform
I basically tried averaging every n samples so that I could map the tape which is typically 1.5
million samples points to 100 sample points. It didn't work, the graph was mostly flat. Then I
bumped into this. https://stackoverflow.com/questions/26663494/algorithm-to-draw-waveform-from-audio
It suggests negating the sample if it is less than 0.

# Document the development process of module
"""