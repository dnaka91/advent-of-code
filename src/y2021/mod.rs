//! # Year 2021
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
//! .calendar .calendar-color-g { color:#a47a4d; }
//! .calendar .calendar-color-o { color:#c74c30; }
//! .calendar .calendar-color-r { color:#ff0000; }
//! .calendar .calendar-color-s { color:#ffffff; }
//! .calendar .calendar-color-w1 { color:#00c8ff; }
//! .calendar .calendar-color-w2 { color:#00b5ed; }
//! .calendar .calendar-color-w3 { color:#00a2db; }
//! .calendar .calendar-color-w4 { color:#0091cc; }
//! .calendar .calendar-color-w5 { color:#0085c0; }
//! .calendar .calendar-color-w6 { color:#0079b5; }
//! .calendar .calendar-color-w7 { color:#006daa; }
//! .calendar .calendar-color-w8 { color:#00619f; }
//! .calendar .calendar-color-w9 { color:#005a98; }
//! .calendar .calendar-color-w10 { color:#005291; }
//! </style>
//!
//! <pre class="calendar">
//! <a href="d01/index.html" class="calendar-day1 calendar-verycomplete"><span class="calendar-color-w1">~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~</span>  <span class="calendar-day"> 1</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d02/index.html" class="calendar-day2 calendar-verycomplete"><span class="calendar-color-w2">'  .     .  .          .       .</span> <span class="calendar-color-s">.</span> <span class="calendar-color-w2">  ''.  </span> <span class="calendar-color-g">..''''</span>  <span class="calendar-day"> 2</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d03/index.html" class="calendar-day3 calendar-verycomplete"><span class="calendar-color-w3">      ..  .    '          .  . </span> <span class="calendar-color-s">.</span> <span class="calendar-color-w3">. .    </span> <span class="calendar-color-g">:</span>        <span class="calendar-day"> 3</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d04/index.html" class="calendar-day4 calendar-verycomplete"><span class="calendar-color-w4">   .       .~       . ..     '.</span> <span class="calendar-color-s">.'</span> <span class="calendar-color-w4">' </span> <span class="calendar-color-g">....'</span>        <span class="calendar-day"> 4</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d05/index.html" class="calendar-day5 calendar-verycomplete"><span class="calendar-color-w5">          .   '            '.</span> <span class="calendar-color-o">.</span><span class="calendar-color-r">.</span><span class="calendar-color-s">|\</span><span class="calendar-color-r">.</span><span class="calendar-color-o">.</span><span class="calendar-color-g">''</span>             <span class="calendar-day"> 5</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d06/index.html" class="calendar-day6 calendar-verycomplete"><span class="calendar-color-w6"> '               . ..     . </span> <span class="calendar-color-g">:</span>                     <span class="calendar-day"> 6</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d07/index.html" class="calendar-day7 calendar-verycomplete"><span class="calendar-color-w7">    .  '  ..     .    .   </span> <span class="calendar-color-g">:'</span>                      <span class="calendar-day"> 7</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d08/index.html" class="calendar-day8 calendar-verycomplete"><span class="calendar-color-w8">     .      .   .      .  </span>  <span class="calendar-color-g">'''''.....</span>  ....       <span class="calendar-day"> 8</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d09/index.html" class="calendar-day9 calendar-verycomplete"><span class="calendar-color-w9">'    .'    . .    .      </span> <span class="calendar-color-g">:'..</span>  <span class="calendar-color-g">..</span>    <span class="calendar-color-g">'</span>'    ':     <span class="calendar-day"> 9</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <span class="calendar-day10">  .'      .     .    <span class="calendar-color-w10">  ..</span> <span class="calendar-color-g">:</span>   <span class="calendar-color-g">''</span>  <span class="calendar-color-g">''''</span>..           <span class="calendar-day">10</span></span>
//! <span class="calendar-day11">                                        '.         <span class="calendar-day">11</span></span>
//! <span class="calendar-day12">                                                   <span class="calendar-day">12</span></span>
//! <span class="calendar-day13">                                                   <span class="calendar-day">13</span></span>
//! <span class="calendar-day14">                                                   <span class="calendar-day">14</span></span>
//! <span class="calendar-day15">                                                   <span class="calendar-day">15</span></span>
//! <span class="calendar-day16">                                                   <span class="calendar-day">16</span></span>
//! <span class="calendar-day17">                                                   <span class="calendar-day">17</span></span>
//! <span class="calendar-day18">                                                   <span class="calendar-day">18</span></span>
//! <span class="calendar-day19">                                                   <span class="calendar-day">19</span></span>
//! <span class="calendar-day20">                                                   <span class="calendar-day">20</span></span>
//! <span class="calendar-day21">                                                   <span class="calendar-day">21</span></span>
//! <span class="calendar-day22">                                                   <span class="calendar-day">22</span></span>
//! <span class="calendar-day23">                                                   <span class="calendar-day">23</span></span>
//! <span class="calendar-day24">                                                   <span class="calendar-day">24</span></span>
//! <span class="calendar-day25">                                                   <span class="calendar-day">25</span></span>
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
