# xsesame

Some desktop environments install a lot of different types that have to real use for
the end user. For example cinnamon also install a cinnamon fallback. Others install a lot more.
This small utility helps you to list and disable some of them. Of course you can also re-enable them.

The propose of this little tool is to minimize the clutter in the display manager.

### Example 
<pre>List of active and inactive sessions:
   <b>Key           </b>  <b>Name                         </b> <b>Comment</b>
<font color="#4E9A06">💚</font>  <font color="#4E9A06">budgie-desktop</font>  <font color="#4E9A06">Budgie Desktop               </font> <font color="#4E9A06">This session logs you into the Budgie Desktop</font>
<font color="#4E9A06">💚</font>  <font color="#4E9A06">cinnamon      </font>  <font color="#4E9A06">Cinnamon                     </font> <font color="#4E9A06">This session logs you into Cinnamon</font>
<font color="#C4A000">🤍</font>  <font color="#C4A000">cinnamon2d    </font>  <font color="#C4A000">Cinnamon (Software Rendering)</font> <font color="#C4A000">This session logs you into Cinnamon (using software rendering)</font>
<font color="#4E9A06">💚</font>  <font color="#4E9A06">icewm-session </font>  <font color="#4E9A06">IceWM Session                </font> <font color="#4E9A06">This session logs you into IceWM</font>
<font color="#4E9A06">💚</font>  <font color="#4E9A06">lxde          </font>  <font color="#4E9A06">LXDE                         </font> <font color="#4E9A06">LXDE - Lightweight X11 desktop environment</font>
<font color="#4E9A06">💚</font>  <font color="#4E9A06">plasma        </font>  <font color="#4E9A06">Plasma (X11)                 </font> <font color="#4E9A06">Plasma by KDE</font>
<font color="#C4A000">🤍</font>  <font color="#C4A000">pop           </font>  <font color="#C4A000">Pop                          </font> <font color="#C4A000">This session logs you into Pop</font>
</pre>


## Commands

### help

<pre><font color="#C4A000">USAGE:</font>
    xsesame [SUBCOMMAND]

<font color="#C4A000">OPTIONS:</font>
    <font color="#4E9A06">-d</font>, <font color="#4E9A06">--session-dir</font> <font color="#4E9A06">&lt;session-dir&gt;</font>
            Session config directory [default: <font color="#4E9A06">test/samples</font>]

    <font color="#4E9A06">-h</font>, <font color="#4E9A06">--help</font>                         Prints help information
    <font color="#4E9A06">-V</font>, <font color="#4E9A06">--version</font>                      Prints version information

<font color="#C4A000">SUBCOMMANDS:</font>
    <font color="#4E9A06">list</font>          list display manager sessions
    <font color="#4E9A06">toggle</font>        Toggle session visibility
    <font color="#4E9A06">enable</font>        Enable a session
    <font color="#4E9A06">disable</font>       Disable a session
    <font color="#4E9A06">export</font>        Export session list
    <font color="#4E9A06">completion</font>    Generate completions for various shells
    <font color="#4E9A06">help</font>          Prints this message or the help of the given subcommand(s)
</pre>


### list
<pre><font color="#C4A000">USAGE:</font>
    xsesame list [FLAGS] [OPTIONS]

<font color="#C4A000">FLAGS:</font>
    <font color="#4E9A06">-l</font>, <font color="#4E9A06">--no-nls</font>     
            Show comment localized if possible

    <font color="#4E9A06">-h</font>, <font color="#4E9A06">--help</font>       
            Prints help information

    <font color="#4E9A06">-V</font>, <font color="#4E9A06">--version</font>    
            Prints version information


<font color="#C4A000">OPTIONS:</font>
    <font color="#4E9A06">-d</font>, <font color="#4E9A06">--session-dir</font> <font color="#4E9A06">&lt;session-dir&gt;</font>
            Session config directory [default: <font color="#4E9A06">test/samples</font>]

    <font color="#4E9A06">-e</font>, <font color="#4E9A06">--emoji</font>=<font color="#4E9A06">&lt;emoji&gt;</font>                
            Use emoji [possible values: <font color="#4E9A06">hearts</font>, <font color="#4E9A06">check</font>, <font color="#4E9A06">plain</font>]

    <font color="#4E9A06">-c</font>, <font color="#4E9A06">--comments</font>=<font color="#4E9A06">&lt;comments&gt;</font>
            Show comments [possible values: <font color="#4E9A06">auto</font>, <font color="#4E9A06">show</font>, <font color="#4E9A06">hide</font>]

    <font color="#4E9A06">-w</font>, <font color="#4E9A06">--what</font>=<font color="#4E9A06">&lt;what&gt;</font>
            filter results [possible values: <font color="#4E9A06">all</font>, <font color="#4E9A06">valid</font>, <font color="#4E9A06">invalid</font>]
</pre>

### toogle

<pre>Toggle session visibility

<font color="#C4A000">USAGE:</font>
    xsesame toggle [FLAGS] [OPTIONS] &lt;session_key&gt;

<font color="#C4A000">FLAGS:</font>
    <font color="#4E9A06">-T</font>, <font color="#4E9A06">--help</font>          Prints help information
    <font color="#4E9A06">-J</font>, <font color="#4E9A06">--no-journal</font>    Disable logging to journal
    <font color="#4E9A06">-V</font>, <font color="#4E9A06">--version</font>       Prints version information

<font color="#C4A000">OPTIONS:</font>
    <font color="#4E9A06">-d</font>, <font color="#4E9A06">--session-dir</font> <font color="#4E9A06">&lt;session-dir&gt;</font>
            Session config directory [default: <font color="#4E9A06">test/samples</font>]


<font color="#C4A000">ARGS:</font>
    <font color="#4E9A06">&lt;session_key&gt;</font>    Toggle session visibility
</pre>

### completion
<pre>Generate completions for various shells

<font color="#C4A000">USAGE:</font>
    xsesame completion [OPTIONS]

<font color="#C4A000">FLAGS:</font>
    <font color="#4E9A06">-h</font>, <font color="#4E9A06">--help</font>       Prints help information
    <font color="#4E9A06">-V</font>, <font color="#4E9A06">--version</font>    Prints version information

<font color="#C4A000">OPTIONS:</font>
    <font color="#4E9A06">-d</font>, <font color="#4E9A06">--session-dir</font> <font color="#4E9A06">&lt;session-dir&gt;</font>
            Session config directory [default: <font color="#4E9A06">test/samples</font>]

    <font color="#4E9A06">-s</font>, <font color="#4E9A06">--shell</font>=<font color="#4E9A06">&lt;shell&gt;</font>
            shell to generate completions [possible values: <font color="#4E9A06">bash</font>, <font color="#4E9A06">zsh</font>, <font color="#4E9A06">fish</font>, <font color="#4E9A06">elvish</font>]
</pre>