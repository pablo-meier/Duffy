# Duffy -- a MIDI -> `beep` compiler

**\[Note: this isn't working yet. It's a work-in-progress, doesn't work, not a
great example of exemplary Rust, &etc.  &etc. &etc.\]**

### Baby's first Rust

Wanting to write something nontrivial in Rust, I figured I'd take an old project
concept I did in college and resurrect it, taking advantage of a few years of
maturity. See the bit on "backstory, nostalgia" below. The "Options" section is
mostly for what I'm hoping to do with it as I write more of it.

---

Duffy takes a MIDI file as input, and produces a set of bash scripts for each
of the input file's tracks that, when executed, will beep your computer's PC
speaker for that audio data. This requires a PC speaker (the hardware kind that
lives in your chassis, not a proper music speaker) and installation of the
[UNIX beep utility][7].

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

      --max-length=<seconds>
          
          Many MIDI files are programmed to loop forever. We default to 25
          seconds (these beep scripts are cute, but can be disrupting to others
          on the machine or workspace) but you can use this flag to set the
          maximum length you'd like the beep scripts to go for.

      --tracks=<tracks>

          A comma-separated list of the tracks you want to encode, where each
          track is a number. If you know this MIDI file and know the melody is
          on track 4, you can isolate it this way without producing the other
          files. If you have a 12-track MIDI and you only want three of them,
          you can all this as `duffy tracks=1,11,6


### Backstory, nostalgia

Back in college, [Saurya][1] and I often pranked each other, or other students
in [the SunLab][2]. Among the typical hijinx (someone leaves a computer
unlocked, `alias lsplease="/bin/ls" && alias ls="echo \"lolno, say lsplease\""`)
the most popular of these was a little Scheme script I wrote to consume a monophanous
[LilyPond][3] file and product a bash script calling the Unix 'beep' command in
the melody of the input file.

So for example, you could write

    d8 d e4 d g fis2 d8 d e4 d a' g2

Which Lilypond would write [as this][8]. My script would then produce:

    #!/bin/bash
    #
    # "Happy Birthday"
    
    beep -f 293.66 -l time -d 250    # d8
    beep -f 293.66 -l time -d 250    # d
    beep -f 329.63 -l time -d 500    # e4
    beep -f 293.66 -l time -d 500    # d
    beep -f 392.00 -l time -d 500    # g
    beep -f 369.99 -l time -d 1000   # fis2
    beep -f 293.66 -l time -d 250    # d8
    beep -f 293.66 -l time -d 250    # d
    beep -f 329.63 -l time -d 500    # e4
    beep -f 293.66 -l time -d 500    # d
    beep -f 440 -l time -d 500       # a
    beep -f 392.00 -l time -d 1000   # g2

Which you could execute in a SunLab computer. For good or ill, [Max Salvas'][4]
computers (MAX BUILT) had PC speakers in the chassis, so you could remotely log
into someone else's machine, run the script, and from the box under their desk
they'd hear whatever you played them.

Saurya then wrote a script to scan a room in the building for all its computers
(TA lab, SunLab, etc.) and run an arbitrary script on all of them at the same
time.

We had 4-8 bars of many popular themes -- Zelda, Mario, and we'd have the
physical boxes in the room literally hum Happy Birthday on people's birthdays.
This eventually led to 'beep' getting removed because we put all these scripts
on our publicly-visible directories, wanting to share, and people abused it.

The Scheme script to convert LilyPond->Beep is lost, but I'm re-writing
it to handle a more popular format.

### _Duffy_?

Named after [Dustin Duffy][5], the Beeper King! Thanks to [Ashely Tuccero][6] of
Etsy for the name :)

   [1]: https://github.com/saurya
   [2]: http://cs.brown.edu/about/rooms/sunlab/
   [3]: http://lilypond.org/
   [4]: http://cs.brown.edu/people/staff/mls/
   [5]: http://www.quotefully.com/tvshow/30+Rock/Dennis+Duffy
   [6]: https://twitter.com/uccero/status/398165936827412480
   [7]: http://www.johnath.com/beep/
   [8]: http://paul-meier.github.io/Duffy/pages/lily-output.png
