# Labyrinth--

Labyrinth-- is a two-dimensional programming language that transcompiles to brainfuck.

### Commands

The language consists of five commands.

| Character           | Meaning         |
| :-----------------: | :-------------- |
| <code>#</code>      | wall            |
| <code>&nbsp;</code> | space           |
| <code>A</code>      | start, entrance |
| <code>B</code>      | end, goal       |
| <code>\n</code>     | new line        |

### Instruction pointer

The instruction pointer is trying to move from point A to point B using right-hand rule.
The transcompiled code is determined by the movement of instruction pointer.

<table>
  <tr>
    <td align="center" colspan="2">
      <b>(n-2) moves in the same direction (if n > 2)</b>
    </td>
  </tr>
  <tr>
    <td align="center">:arrow_right:</td>
    <td align="center"><code>></code></td>
  </tr>
  <tr>
    <td align="center">:arrow_left:</td>
    <td align="center"><code><</code></td>
  </tr>
  <tr>
    <td align="center">:arrow_up:</td>
    <td align="center"><code>+</code></td>
  </tr>
  <tr>
    <td align="center">:arrow_down:</td>
    <td align="center"><code>-</code></td>
  </tr>
  <tr>
    <td align="center" colspan="2">
      <b>T-junction (if instruction pointer hits the wall)</b>
    </td>
  </tr>
  <tr>
    <td align="center">:arrow_right:</td>
    <td align="center"><code>[</code></td>
  </tr>
  <tr>
    <td align="center">:arrow_left:</td>
    <td align="center"><code>]</code></td>
  </tr>
  <tr>
    <td align="center">:arrow_up:</td>
    <td align="center"><code>.</code></td>
  </tr>
  <tr>
    <td align="center">:arrow_down:</td>
    <td align="center"><code>.</code></td>
  </tr>
  <tr>
    <td align="center" colspan="2">
      <b>four-way junction</b>
    </td>
  </tr>
  <tr>
    <td align="center">:hash:</td>
    <td align="center"><code>,</code></td>
  </tr>
</table>


### First move
The first move is first legal move of: :arrow_up:, :arrow_right:, :arrow_down:, :arrow_left:.
