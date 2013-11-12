# Duffy -- a MIDI -> `beep` compiler

Duffy takes a MIDI file as input, and produces a set of bash scripts for each
of its tracks that, when executed, will beep your computer's PC speaker for that
audio data.

### Options

    duffy <options> input

    options:
      
      --start=<time>

          How far to read into the MIDI file before beginning to encode the beep
          files. Good for when you have a MIDI track with an indulgent
          introduction, and you just want the meat.
          <time> can be formatted like a YouTube video tag, so "1m32s," "45s,"
          or "0m33s" are acceptable values. Defaults to "0m0s"

      --end=<time>
          
          Like `--start`, determines when we stop encoding. If you input a value
          greater than the MIDI file has data for, the script will just go to
          the end. Time format same as above.


      --tracks=<tracks>

          A comma-separated list of the tracks you want to encode, where each
          track is a number. If you know this MIDI file and know the melody is
          on track 4, you can isolate it this way without producing the other
          files. If you have a 12-track MIDI and you only want three of them,
          you can all this as `duffy tracks=1,11,6


### Backstory, nostalgia

Back in college, [Saurya][1] and I often pranked or otherwise played with the
computers in [the SunLab][2]. The most popular of these was a script I wrote to
produces script that calling the Unix 'beep' command in the melody of whatever
you input it, expressed as a monophanous [LilyPond][3] file.

So for example, you could write

    d8 d e4 d g fis2 d8 d e4 d a' g2

Which Lilypond would write to this:

**img**

and my script would produce:

    #!/bin/bash
    #
    # "Happy Birthday"
    
    beep -f freq -l time -d 150    # d8
    beep -f freq -l time -d 150    # d
    beep -f freq -l time -d 150    # e4
    beep -f freq -l time -d 150    # d
    beep -f freq -l time -d 150    # g
    beep -f freq -l time -d 150    # fis2
    beep -f freq -l time -d 150    # d8
    beep -f freq -l time -d 150    # d
    beep -f freq -l time -d 150    # e4
    beep -f freq -l time -d 150    # d
    beep -f freq -l time -d 150    # a
    beep -f freq -l time -d 150    # g2

Which you could execute in a SunLab computer. For good or ill, [Max Salvas][4]' 
computers (MAX BUILT) had PC speakers in the chassis, so you could remotely log
into someone else's machine, run the script, and from the box under their desk
they'd hear whatever you played them.

Saurya then wrote a script to scan a room in the building for all it's computers
(TA lab, SunLab, etc.) and run a script on all of them at the same time.

We had 4-8 bars of many popular themes -- Zelda, Mario, and we'd have the room
literally sing Happy Birthday on people's birthdays. This eventually led to
'beep' getting removed because we put all these scripts on our publicly-visible
directories, wanting to share, and people abused it.

The Scheme script to convert LilyPond subset->Beep is lost, but I'm re-writing
it to handle a more popular format.

### _Duffy_?

Named after [Dustin Duffy][5], the Beeper King! Thanks to [Ashely Tuccero][6]
for the name :)

   [1]: https://github.com/saurya
   [2]: http://cs.brown.edu/about/rooms/sunlab/
   [3]: http://lilypond.org/
   [4]: http://cs.brown.edu/people/staff/mls/
   [5]: http://www.quotefully.com/tvshow/30+Rock/Dennis+Duffy
   [6]: https://twitter.com/uccero/status/398165936827412480
