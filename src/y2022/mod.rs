//! # Year 2022
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
//! .calendar .calendar-color-a { color:#aaaaaa; }
//! .calendar .calendar-color-c { color:#eeeeee; }
//! .calendar .calendar-color-d { color:#685c46; }
//! .calendar .calendar-color-g0 { color:#488813; }
//! .calendar .calendar-color-g1 { color:#4d8b03; }
//! .calendar .calendar-color-g2 { color:#7fbd39; }
//! .calendar .calendar-color-g3 { color:#427322; }
//! .calendar .calendar-color-g4 { color:#01461f; }
//! .calendar .calendar-color-s { color:#d0b376; }
//! .calendar .calendar-color-u { color:#5eabb4; }
//! </style>
//!
//! <pre class="calendar">
//! <a href="d25/index.html" class="calendar-day25">#@@@@@@#@@@@##@#@@@#@@#@#@###@@##@#@@@#@@#@@@@@@@  <span class="calendar-day">25</span></a>
//! <a href="d24/index.html" class="calendar-day24">@#@@@@@@@@@#@##@@@@@#@@@@@@@@@@#@@##@@@@@@@#@@#@#  <span class="calendar-day">24</span></a>
//! <a href="d23/index.html" class="calendar-day23">@@@@@##@@@#@@##@@@@@##@@#@@#@######@@@#@@@#####@#  <span class="calendar-day">23</span></a>
//! <a href="d22/index.html" class="calendar-day22">@@#@@@@@@@#@@@@@@#@@##@@#@@#@@@#@##@@@@#@@@#@@@@#  <span class="calendar-day">22</span></a>
//! <a href="d21/index.html" class="calendar-day21">@@@#@@@#@@@###@@@@@##@@@#@@#@#@@@@@#@##@##@###@@@  <span class="calendar-day">21</span></a>
//! <a href="d20/index.html" class="calendar-day20">@@@@@@@##@@@####@#@@#@##@##@@@@@@@#@#@#@@@@@@#@@@  <span class="calendar-day">20</span></a>
//! <a href="d19/index.html" class="calendar-day19">@@#@@@@@@@#@@@#@@@##@@@#@#|#@@#@@##@@@@@@#@@@@###  <span class="calendar-day">19</span></a>
//! <a href="d18/index.html" class="calendar-day18">@@@@@@@#@@@@@@@@@@@@##@@#@##@@@#@@@@@@@@@#@#@@@@@  <span class="calendar-day">18</span></a>
//! <a href="d17/index.html" class="calendar-day17">@@@@@@@@@@@@@@@##@@##@@#@@#@@#@@@@@@##@@#@@@@###@  <span class="calendar-day">17</span></a>
//! <a href="d16/index.html" class="calendar-day16">@@@@@@#@#@#@@#@#@####@@@@@@@@#@@@@#@#@@#@@@##@@@@  <span class="calendar-day">16</span></a>
//! <a href="d15/index.html" class="calendar-day15">@@###@#@#@@@@@##@@@#@@@#@#@@@@#@#@#@@#@@@@@@##@@@  <span class="calendar-day">15</span></a>
//! <a href="d14/index.html" class="calendar-day14">#####@@#@@@#@#@@@@@#@@##@#@@@#@@#@@@@@#@##@#@@#@#  <span class="calendar-day">14</span></a>
//! <a href="d13/index.html" class="calendar-day13">@@@#@#@@@@@@##@@@@@###@##@##@@@@@@@@#@@@@#@@@#@#@  <span class="calendar-day">13</span></a>
//! <a href="d12/index.html" class="calendar-day12">@@@@#@@@@@##@@@#@@@@@@@@@##@@#@@@@##@@##@@@#@@@#@  <span class="calendar-day">12</span></a>
//! <a href="d11/index.html" class="calendar-day11">@@#@##@@@@@@@@@#@@@@@@@@@#@##@##@#@@@#@@@@@@@@@@@  <span class="calendar-day">11</span></a>
//! <a href="d10/index.html" class="calendar-day10">@@@@@@@@#@#@#@@#@@@@@@###@@@@@@@#@@@#@@@@@@@##@#@  <span class="calendar-day">10</span></a>
//! <a href="d09/index.html" class="calendar-day9">@@@#@@@@#@#<span class="calendar-color-s">%%%%%|</span><span class="calendar-color-a">_</span> <span class="calendar-color-g0">@</span><span class="calendar-color-g3">@</span><span class="calendar-color-g2">@</span><span class="calendar-color-g4">#@</span><span class="calendar-color-g0">@</span>@##@@@@@@@@#@@@#@@@@@@#@  <span class="calendar-day"> 9</span></a>
//! <a href="d08/index.html" class="calendar-day8 calendar-verycomplete">#@#<span class="calendar-color-g0">#</span><span class="calendar-color-g1">@</span><span class="calendar-color-s">..</span><span class="calendar-color-a">/  \</span><span class="calendar-color-u">.~~.</span><span class="calendar-color-a">/  \</span><span class="calendar-color-s">.....</span><span class="calendar-color-g2">#</span><span class="calendar-color-g1">@</span>@@@@@@@@@@#@@@#|#@@@@@@  <span class="calendar-day"> 8</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d07/index.html" class="calendar-day7 calendar-verycomplete">@#@<span class="calendar-color-g2">@</span><span class="calendar-color-g0">@</span><span class="calendar-color-g2">#@#</span><span class="calendar-color-g3">#</span><span class="calendar-color-g1">@</span><span class="calendar-color-g3">@</span><span class="calendar-color-u">.~~.</span><span class="calendar-color-g2">#</span><span class="calendar-color-g3">@</span><span class="calendar-color-g0">#</span><span class="calendar-color-g2">#@</span><span class="calendar-color-g3">@</span><span class="calendar-color-g1">#@@</span><span class="calendar-color-s">.</span><span class="calendar-color-g3">@</span><span class="calendar-color-g4">@</span>##@@@@#@@#@#@@#@@@#@##  <span class="calendar-day"> 7</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d06/index.html" class="calendar-day6 calendar-verycomplete">@@@<span class="calendar-color-g3">@@</span><span class="calendar-color-g2">@</span><span class="calendar-color-g3">@</span><span class="calendar-color-g2">@</span><span class="calendar-color-g3">@</span><span class="calendar-color-g0">@</span><span class="calendar-color-g4">@</span><span class="calendar-color-g1">@</span><span class="calendar-color-u">.~~.</span><span class="calendar-color-g2">#</span><span class="calendar-color-g3">#@@</span><span class="calendar-color-g0">#</span><span class="calendar-color-g1">@#</span><span class="calendar-color-s">..</span><span class="calendar-color-g3">@</span><span class="calendar-color-g2">@@</span><span class="calendar-color-g1">@</span>@#@@@@@@@@@####@@@##  <span class="calendar-day"> 6</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d05/index.html" class="calendar-day5 calendar-verycomplete">#@<span class="calendar-color-g1">#</span><span class="calendar-color-g0">@</span><span class="calendar-color-g1">@</span><span class="calendar-color-g2">@</span><span class="calendar-color-g3">#@#</span><span class="calendar-color-g0">#</span><span class="calendar-color-g3">@</span><span class="calendar-color-g0">@</span><span class="calendar-color-g1">#</span><span class="calendar-color-u">.~~.</span><span class="calendar-color-g3">@</span><span class="calendar-color-g0">#</span><span class="calendar-color-s">.</span><span class="calendar-color-c">/\</span><span class="calendar-color-s">.'</span><span class="calendar-color-g2">@@</span><span class="calendar-color-g3">@</span><span class="calendar-color-d">|</span><span class="calendar-color-g0">@</span><span class="calendar-color-g4">@</span>@@@@@@@#@@@@@#@@@@@  <span class="calendar-day"> 5</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d04/index.html" class="calendar-day4 calendar-verycomplete">#<span class="calendar-color-g3">#</span><span class="calendar-color-g0">@</span><span class="calendar-color-g4">#</span><span class="calendar-color-g1">@</span><span class="calendar-color-g4">@</span><span class="calendar-color-g1">@</span><span class="calendar-color-g3">@</span><span class="calendar-color-g0">#@@</span><span class="calendar-color-s">.'</span><span class="calendar-color-u"> ~  </span><span class="calendar-color-s">'.</span><span class="calendar-color-c">/\</span><span class="calendar-color-s">'.</span><span class="calendar-color-c">/\</span><span class="calendar-color-s">' .</span><span class="calendar-color-g1">#</span><span class="calendar-color-g0">@@</span>#@@@@@#@#@#@@@@@@@  <span class="calendar-day"> 4</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d03/index.html" class="calendar-day3 calendar-verycomplete"><span class="calendar-color-g1">#</span><span class="calendar-color-g3">#</span><span class="calendar-color-g2">@</span><span class="calendar-color-g1">@</span><span class="calendar-color-g0">@</span><span class="calendar-color-g2">@</span><span class="calendar-color-g4">#</span><span class="calendar-color-g2">#</span><span class="calendar-color-g0">@</span><span class="calendar-color-s">_/</span><span class="calendar-color-u"> ~   ~  </span><span class="calendar-color-s">\ ' '. '.'.</span><span class="calendar-color-g1">@</span><span class="calendar-color-g3">@</span>@@@@@@@@##@@@@#@@  <span class="calendar-day"> 3</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d02/index.html" class="calendar-day2 calendar-verycomplete"><span class="calendar-color-s">-~------'</span><span class="calendar-color-u">    ~    ~ </span><span class="calendar-color-s">'--~-----~-~----___________--</span>  <span class="calendar-day"> 2</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d01/index.html" class="calendar-day1 calendar-verycomplete"><span class="calendar-color-u">  ~    ~  ~      ~     ~ ~   ~     ~  ~  ~   ~   </span>  <span class="calendar-day"> 1</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
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
