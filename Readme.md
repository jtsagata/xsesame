# xsessame

Some desktop environments install a lot of different types that have to real use for
the end user. For example cinnamon also install a cinnamon fallback. Others install a lot more.
This small utility helps you to list and disable some of them. Of course you can also re-enable them.

The propose of this little tool is to minimize the clutter in the display manager.

## Commands

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


