layout = "post"
title = "How I make modul"
created = "2022-05-07"
updated = "2022-09-21"
tags = "#audio"
markdown = """
https://tesselode.github.io/articles/audio-libraries-considered-challenging/

I've been working on an audio/visual program called **modul** for the last 2 years. It is the only personal project that I have invested this amount of time. Fortunately it paid off. I have a lot of fun and learn new things when I work on **modul**. I wish I started to write about my work on **modul** much earlier but it is better late than sorry. 

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

### Fixing the Metronome
After a while I wanted to add a metronome to the program because I needed to use an external metronome, my iPad, to start a song. I implemented one like below
<pre class="prettyprint linenums">
pub fn update(&mut self, sample_count: u32) {
    self.sample_sum += sample_count;

    let remainder = self.sample_sum % self.tick_period as u32;
    self.show_beat = remainder > 0 && remainder < 10_000;
    self.beat_index = self.sample_sum / self.tick_period as u32;
}
</pre>
I was very happy that I now didn't need an external device to start a song. I used this for a while but I was having difficulty keeping time when I play the drums while listening to the metronome. For months I thought that it was because I was bad as a musician that I couldn't even keep a proper time. I practiced a lot with my synth and my drum. But all that practice didn't improve my time keeping and at that point I finally started to think that my metronome implementation could be buggy. I tried syncing modul's metronome with the microKorg and voila! There was a discrepancy. Either my metronome's time or microKorg's time was wrong. Fortunately I'm an experienced programmer and many many years ago I learnt to blame my code first than other systems which are tested thousands times more. I then changed the implementation to this
<pre class="prettyprint linenums">
pub fn update(&mut self) {
    let remainder = self.instant.elapsed().as_nanos() % self.beat_period;
    self.show_beat = remainder > 0 && remainder < 50_000_000;
    self.beat_index = (self.instant.elapsed().as_nanos() / self.beat_period) as u32;
}
</pre>
This implementation can keep the time good enough for my ears at least. Conclusion is bugs and glitches in audio systems are very difficult to catch and fix also. Or maybe I'm so used to working on visual things on computers that I don't know what to sense when it comes to sound. It was so much fun though.

Next day update: I decided to get the opinions of the rust audio community on discord and they immediately told me that it is a bad idea to keep time using `std::time` since kernel time will always drift and the audio hardware and driver is working really hard to keep the right time. So I discarded my implementation that used std::time and decided to fix the one that counts the samples. Instead of updating metronome by calling `metronome.update(sample_count)`, I changed it to `metronome.update()` and called it every time I pushed a sample to the output buffer. It works fine as far as my ears are concerned.

# Document the development process of module
"""