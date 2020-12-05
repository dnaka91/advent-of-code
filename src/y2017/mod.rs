//! # Year 2017
//!
//! <style>
//! .calendar {
//!   cursor: default;
//!   -webkit-user-select: none;
//!   -khtml-user-select: none;
//!   -moz-user-select: -moz-none;
//!   -o-user-select: none;
//!   user-select: none;
//!   color: #666666;
//!   background-color: #0f0f23;
//! }
//! .calendar > span {
//!   color: #333333;
//! }
//! .calendar > a {
//!   text-decoration: none !important;
//!   color: #666666 !important;
//!   outline: none;
//!   cursor: default;
//! }
//! .calendar a:hover, .calendar a:focus {
//!   background-color: #1e1e46;
//!   background-color: rgba(119,119,165,.2);
//!   cursor: pointer;
//! }
//! .calendar .calendar-day { color: #666666; }
//! .calendar a .calendar-day { color: #cccccc; }
//! .calendar a .calendar-mark-complete,
//! .calendar a .calendar-mark-verycomplete { visibility: hidden; }
//! .calendar a.calendar-complete     .calendar-mark-complete,
//! .calendar a.calendar-verycomplete .calendar-mark-complete { visibility: visible; color: #ffff66; }
//! .calendar a.calendar-verycomplete .calendar-mark-verycomplete { visibility: visible; color: #ffff66; }
//!
//! .calendar i { font-style:normal; display:inline-block; width:.6em; line-height:.6em; }
//! .calendar .calendar-edge { color:#cccccc; }
//! .calendar .calendar-edge, .calendar .calendar-day, .calendar .calendar-mark-complete, .calendar .calendar-mark-verycomplete { animation-name:none; }
//! .calendar .calendar-disabled { opacity:.2; }
//! .calendar > a > .calendar-star, .calendar > span > .calendar-star { color:#666666; animation-name:none; } .calendar > a.calendar-day-new > .calendar-star { color:inherit; }
//! .calendar .calendar-ornament5 { color:#990099; }
//! .calendar .calendar-ornament1 { color:#0066ff; }
//! .calendar .calendar-ornament4 { color:#009900; }
//! .calendar > a.calendar-verycomplete > .calendar-star { color:#ffff66; text-shadow:0 0 5px #ffff66; }
//! .calendar .calendar-ornament2 { color:#aaaaaa; }
//! .calendar > a.calendar-complete > .calendar-star { color:#cccccc; animation-name:none; }
//! .calendar > a > span { animation-name:anim-reveal; animation-duration:1s; animation-fill-mode:both; } @keyframes anim-reveal { 0%   { opacity:0.2; filter:grayscale(100%) brightness(100%); } 10%  { opacity:1.0; filter:grayscale(100%) brightness(1000%); } 100% { opacity:1.0; filter:grayscale(0%) brightness(100%); } } .calendar > a.calendar-verycomplete > .calendar-star { animation-name:anim-reveal-star; animation-duration:1s; animation-fill-mode:both; } @keyframes anim-reveal-star { 0%   { opacity:0.2; filter:grayscale(100%) brightness(100%);  text-shadow:0 0 0px #ffff66, 0 0  0px #ffff66, 0 0  0px #ffff66, 0 0   0px #ffff66;  } 10%  { opacity:1.0; filter:grayscale(100%) brightness(1000%); text-shadow:0 0 5px #ffff66, 0 0 10px #ffff66, 0 0 50px #ffff66, 0 0 100px #ffff66; } 100% { opacity:1.0; filter:grayscale(0%)   brightness(100%);  text-shadow:0 0 5px #ffff66, 0 0  0px #ffff66, 0 0  0px #ffff66, 0 0   0px #ffff66;  } } .calendar .calendar-disabled { animation-play-state:paused; }
//! .calendar .calendar-ornament0 { color:#ff9900; }
//! .calendar .calendar-ornament3 { color:#ff0000; }
//! </style>
//!
//! <pre class="calendar">
//! <span class="calendar-edge"><i>.</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>.</i></span>
//! <a href="d25/index.html" class="calendar-day25"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>┌</i><i>─</i><i>─</i><i>─</i><i>o</i><i>┌</i><i>─</i><i>┬</i><i>─</i><span class="calendar-ornament4"><i>o</i><i>T</i><i>o</i></span><i>─</i><i>o</i><i>┌</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┬</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><span class="calendar-ornament5"><i>┤</i><i>[</i><i>]</i><i>├</i></span><i>─</i><i>─</i><i>─</i><i>o</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">25</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d24/index.html" class="calendar-day24"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>├</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>V</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i><i>o</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>┌</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><span class="calendar-ornament1"><i>|</i><i>(</i></span><i>─</i><i>─</i><i>─</i><i>─</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>─</i><i>┐</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">24</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d23/index.html" class="calendar-day23"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>│</i><i>o</i><i>─</i><i>┬</i><i>┴</i><i>┴</i><i>┴</i><i>┴</i><i>┴</i><i>┬</i><i>─</i><i>─</i><i>┐</i><i>┌</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┼</i><i>┴</i><i>┴</i><i>┴</i><i>┴</i><i>┬</i><i>─</i><i>┘</i><i>o</i><i>─</i><span class="calendar-ornament5"><i>┤</i><i>[</i><i>]</i><i>├</i></span><i>┐</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>o</i><i>─</i><i>┘</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">23</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d22/index.html" class="calendar-day22"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>│</i><i>┌</i><i>─</i><i>┤</i><i> </i><i> </i><i> </i><i> </i><i> </i><i>├</i><i>─</i><i>─</i><i>┤</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┤</i><i> </i><i> </i><i> </i><i>1</i><i>├</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i><i>┌</i><i>┘</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>o</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">22</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d21/index.html" class="calendar-day21"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>│</i><i>└</i><i>─</i><i>┤</i><i> </i><i>A</i><i>o</i><i>C</i><i> </i><i>├</i><i>o</i><i>─</i><i>┘</i><i>┌</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┤</i><i> </i><i> </i><i>A</i><i>0</i><i>├</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>o</i><i>└</i><i>┤</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┤</i><i>V</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">21</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d20/index.html" class="calendar-day20"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>└</i><i>─</i><i>─</i><i>┤</i><i> </i><i>2</i><i>0</i><i>1</i><i>7</i><i>├</i><i>┌</i><i>─</i><i>┐</i><i>└</i><i>─</i><i>┐</i><i>┌</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┤</i><i> </i><i> </i><i>P</i><i>7</i><i>├</i><i>─</i><i>─</i><span class="calendar-ornament2"><i>[</i><i>─</i><i>]</i></span><i>─</i><i>─</i><i>┘</i><i>└</i><i>─</i><i>─</i><i>─</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>o</i><i>─</i><i>┐</i><i>└</i><i>┤</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">20</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d19/index.html" class="calendar-day19"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>┌</i><i>─</i><i>─</i><i>┴</i><i>┬</i><i>┬</i><i>┬</i><i>┬</i><i>┬</i><i>┴</i><i>┘</i><i>o</i><i>┴</i><i>─</i><i>─</i><i>┘</i><i>└</i><i>─</i><i>┐</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>┤</i><i> </i><i> </i><i>L</i><i>1</i><i>├</i><i>o</i><i>┌</i><i>┬</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>┌</i><i>─</i><i>┴</i><i>o</i><i>│</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">19</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d18/index.html" class="calendar-day18"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>├</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>┌</i><i>┘</i><i>┌</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>└</i><i>─</i><i>┤</i><i> </i><i> </i><i> </i><i> </i><i>├</i><i>─</i><i>┘</i><i>=</i><i>┌</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>o</i><i>─</i><i>─</i><i>─</i><i>┴</i><i>─</i><i>┐</i><i>┌</i><i>┘</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">18</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d17/index.html" class="calendar-day17"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>└</i><i>─</i><i>─</i><i>┬</i><i>─</i><i>o</i><i>└</i><i>─</i><i>┤</i><i>┌</i><i>─</i><i>─</i><span class="calendar-ornament5"><i>┤</i><i>[</i><i>]</i><i>├</i></span><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┴</i><i>┬</i><i>┬</i><i>┬</i><i>┬</i><i>┴</i><i>─</i><i>─</i><i>┐</i><i>│</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>─</i><i>─</i><i>┘</i><i>┌</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>└</i><i>┐</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">17</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d16/index.html" class="calendar-day16"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>o</i><i>─</i><i>─</i><i>┘</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>─</i><i>┐</i><i>│</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>o</i><i>┌</i><i>─</i><i>─</i><i>┴</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>o</i><i>└</i><i>┘</i><i>└</i><i>─</i><i>─</i><i>┐</i><i>o</i><i>┴</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">16</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d15/index.html" class="calendar-day15"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>─</i><i>─</i><i>┘</i><i>o</i><i>┐</i><i>│</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>┌</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">15</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d14/index.html" class="calendar-day14"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>├</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i><i>└</i><i>┴</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><span class="calendar-ornament1"><i>|</i><i>(</i></span><i>─</i><i>─</i><i>┘</i><i>┌</i><i>─</i><i>─</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>o</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┴</i><i>─</i><i>─</i><span class="calendar-ornament1"><i>|</i><i>(</i></span><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┤</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">14</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d13/index.html" class="calendar-day13"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>└</i><i>─</i><i>─</i><i>─</i><i>┐</i><i>└</i><i>─</i><i>─</i><i>─</i><span class="calendar-ornament4"><i>o</i><i>T</i><i>o</i></span><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┬</i><i>┴</i><i>┴</i><i>┴</i><i>┴</i><i>┴</i><i>┬</i><i>─</i><i>─</i><span class="calendar-ornament4"><i>o</i><i>T</i><i>o</i></span><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>o</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">13</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d12/index.html" class="calendar-day12"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>┌</i><i>─</i><i>─</i><i>o</i><i>│</i><i>┌</i><i>─</i><i>─</i><i>o</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>─</i><span class="calendar-ornament4"><i>o</i><i>T</i><i>o</i></span><i>─</i><i>─</i><i>─</i><i>─</i><i>┤</i><i> </i><i> </i><i> </i><i> </i><i> </i><i>├</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">12</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d11/index.html" class="calendar-day11"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>├</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>├</i><i>─</i><i>─</i><i>┐</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><span class="calendar-ornament2"><i>[</i><i>─</i><i>]</i></span><i>─</i><i>─</i><i>┤</i><i>E</i><i>N</i><i>C</i><i>R</i><i> </i><i>├</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>o</i><i>─</i><i>┼</i><i>┴</i><i>┴</i><i>┴</i><i>┬</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">11</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d10/index.html" class="calendar-day10 calendar-complete"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>└</i><i>┐</i><i>o</i><i>─</i><i>─</i><i>┘</i><i>┌</i><i>─</i><i>┴</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>─</i><i>─</i><i>─</i><i>┤</i><i>Y</i><i>P</i><i>T</i><i>R</i><i> </i><i>├</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i><i>└</i><i>─</i><i>─</i><i>┤</i><i> </i><i>3</i><i>5</i><i>├</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day">10</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d09/index.html" class="calendar-day9 calendar-verycomplete"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>┌</i><i>┴</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i></span><span class="calendar-star" style="animation-name:none;"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i><i>=</i><i>└</i><i>─</i><i>─</i><i>┐</i><i>o</i><i>┴</i><i>┬</i><i>┬</i><i>┬</i><i>┬</i><i>┬</i><i>┴</i><i>─</i><i>o</i><i>┌</i><i>─</i><i>─</i><i>┘</i><i>┌</i><i>─</i><i>─</i><i>┤</i><i> </i><i>1</i><i>9</i><i>├</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┤</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day"> 9</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d08/index.html" class="calendar-day8 calendar-verycomplete"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>=</i></span><span class="calendar-star" style="animation-name:none;"><i>*</i></span><span class="calendar-disabled"><i>─</i><i>─</i><i>┬</i><i>─</i><i>─</i><i>┘</i><i>┌</i><i>─</i><i>─</i><i>┐</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>┌</i><i>─</i><i>┘</i><i>┌</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>o</i><i>─</i><i>─</i><i>┴</i><i>─</i><i>─</i><i>┤</i><i> </i><i>4</i><i>2</i><i>├</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day"> 8</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d07/index.html" class="calendar-day7 calendar-complete"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>┌</i><i>┘</i><i>o</i><i>─</i><i>┘</i><i>┌</i><i>─</i></span><span class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>└</i><i>─</i><i>o</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┴</i><i>─</i><i>─</i><i>┴</i><i>─</i><i>─</i><i>─</i><span class="calendar-ornament0"><i>∧</i><i>∧</i><i>∧</i></span><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┤</i><i> </i><i>1</i><i>6</i><i>├</i><i>o</i><i>┌</i><i>─</i><i>─</i><i>┐</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day"> 7</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d06/index.html" class="calendar-day6 calendar-verycomplete"><span class="calendar-edge"><i>|</i></span><i> </i><span class="calendar-disabled"><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┴</i><i>o</i></span><span style="animation-delay:1.4s"><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:1.3s"><i>─</i><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:1.3s" class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>o</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┐</i><i>┌</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┴</i><i>┬</i><i>┬</i><i>┬</i><i>┴</i><i>─</i><i>┴</i><i>o</i><i>┌</i><i>┘</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day"> 6</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d05/index.html" class="calendar-day5 calendar-verycomplete"><span class="calendar-edge"><i>|</i></span><i> </i><span style="animation-delay:1.1s" class="calendar-star"><i>*</i></span><span style="animation-delay:1.1s"><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><span class="calendar-ornament3"><i>┤</i><i>|</i><i>├</i></span></span><span style="animation-delay:1.2s"><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:1.3s"><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i></span><span class="calendar-disabled"><i>┌</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┤</i><i>V</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day"> 5</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d04/index.html" class="calendar-day4 calendar-verycomplete"><span class="calendar-edge"><i>|</i></span><i> </i><span style="animation-delay:1.1s"><i>├</i><i>─</i><i>─</i></span><span style="animation-delay:1.0s"><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:0.9s"><i>─</i><i>─</i></span><span style="animation-delay:0.9s" class="calendar-star"><i>*</i></span><span class="calendar-disabled"><i>o</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┴</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><span class="calendar-ornament0"><i>∧</i><i>∧</i><i>∧</i></span><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┴</i><i>┘</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day"> 4</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d03/index.html" class="calendar-day3 calendar-verycomplete"><span class="calendar-edge"><i>|</i></span><i> </i><span style="animation-delay:1.1s"><i>│</i></span><span style="animation-delay:1.2s"><i>┌</i><i>─</i><i>─</i><i>─</i><span class="calendar-ornament4"><i>o</i><i>T</i><i>o</i></span><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:1.3s"><i>─</i><i>─</i><i>─</i><i>o</i></span><span style="animation-delay:0.9s"><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:0.8s"><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┬</i><i>─</i><i>─</i><i>─</i><span class="calendar-ornament2"><i>[</i><i>─</i><i>]</i></span></span><span style="animation-delay:0.7s"><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:0.6s"><i>─</i></span><span style="animation-delay:0.6s" class="calendar-star"><i>*</i></span><span style="animation-delay:0.6s"><i>o</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day"> 3</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d02/index.html" class="calendar-day2 calendar-verycomplete"><span class="calendar-edge"><i>|</i></span><i> </i><span style="animation-delay:1.1s"><i>│</i><i>└</i><i>┐</i></span><span style="animation-delay:0.2s" class="calendar-star"><i>*</i></span><span style="animation-delay:0.2s"><i>─</i><i>─</i><i>─</i><i>─</i><span class="calendar-ornament4"><i>o</i><i>T</i><i>o</i></span><i>─</i></span><span style="animation-delay:0.3s"><span class="calendar-ornament0"><i>∧</i><i>∧</i><i>∧</i></span><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:0.4s"><span class="calendar-ornament0"><i>∧</i><i>∧</i><i>∧</i></span><i>─</i><i>┐</i></span><span style="animation-delay:0.8s"><i>└</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:0.9s"><i>─</i><i>─</i><i>─</i><i>─</i><i>o</i></span><span style="animation-delay:0.5s"><i>┌</i><i>─</i><i>─</i></span><span style="animation-delay:0.6s"><i>─</i><i>─</i><i>┐</i><i>└</i><i>┐</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day"> 2</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d01/index.html" class="calendar-day1 calendar-verycomplete"><span class="calendar-edge"><i>|</i></span><i> </i><span style="animation-delay:1.1s"><i>└</i><i>─</i><i>┘</i></span><span style="animation-delay:0.2s"><i>└</i></span><span style="animation-delay:0.1s"><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:0.0s"><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:0.0s" class="calendar-star"><i>*</i></span><span style="animation-delay:0.4s"><i>o</i><i>─</i><i>─</i><i>┴</i><i>─</i><i>─</i><i>─</i><i>─</i></span><span style="animation-delay:0.5s"><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>─</i><i>┘</i></span><span style="animation-delay:0.6s"><i>o</i><i>─</i><i>─</i><i>─</i><i>┴</i><i>─</i><i>┘</i></span><i> </i><span class="calendar-edge"><i>|</i></span>  <span class="calendar-day"> 1</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <span class="calendar-edge"><i>'</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>-</i><i>'</i></span>
//! </pre>

#![allow(unused_variables)]

pub mod d01;
pub mod d02;
pub mod d03;
pub mod d04;
pub mod d05;
pub mod d06;
pub mod d07;
pub mod d08;
pub mod d09;
pub mod d10;
pub mod d11;
pub mod d12;
pub mod d13;
pub mod d14;
pub mod d15;
pub mod d16;
pub mod d17;
pub mod d18;
pub mod d19;
pub mod d20;
pub mod d21;
pub mod d22;
pub mod d23;
pub mod d24;
pub mod d25;
