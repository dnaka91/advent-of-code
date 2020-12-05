//! # Year 2015
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
//! .calendar a.calendar-complete, .calendar a.calendar-verycomplete {
//!   color: #009900 !important;
//! }
//! .calendar a.calendar-day25.calendar-complete,
//! .calendar a.calendar-day25.calendar-verycomplete{ color: #ffff66; }
//! .calendar a.calendar-day25.calendar-verycomplete{ text-shadow: 0 0 5px #ffff66; }
//!
//! .calendar .calendar-trunk { color: #cccccc; }
//!
//! .calendar a                       .calendar-ornament0 { color: inherit; }
//! .calendar a                       .calendar-ornament1 { color: inherit; }
//! .calendar a                       .calendar-ornament2 { color: inherit; }
//! .calendar a                       .calendar-ornament3 { color: inherit; }
//! .calendar a.calendar-verycomplete .calendar-ornament0 { color: #0066ff; text-shadow: 0 0 5px #0066ff; }
//! .calendar a.calendar-verycomplete .calendar-ornament1 { color: #ff9900; text-shadow: 0 0 5px #ff9900; }
//! .calendar a.calendar-verycomplete .calendar-ornament2 { color: #ff0000; text-shadow: 0 0 5px #ff0000; }
//! .calendar a.calendar-verycomplete .calendar-ornament3 { color: #ffff66; text-shadow: 0 0 5px #ffff66; }
//!
//! .calendar.calendar-beckon .calendar-day1  { animation: anim-beckon 3s infinite -2.875s; }
//! .calendar.calendar-beckon .calendar-day2  { animation: anim-beckon 3s infinite -2.750s; }
//! .calendar.calendar-beckon .calendar-day3  { animation: anim-beckon 3s infinite -2.625s; }
//! .calendar.calendar-beckon .calendar-day4  { animation: anim-beckon 3s infinite -2.500s; }
//! .calendar.calendar-beckon .calendar-day5  { animation: anim-beckon 3s infinite -2.375s; }
//! .calendar.calendar-beckon .calendar-day6  { animation: anim-beckon 3s infinite -2.250s; }
//! .calendar.calendar-beckon .calendar-day7  { animation: anim-beckon 3s infinite -2.125s; }
//! .calendar.calendar-beckon .calendar-day8  { animation: anim-beckon 3s infinite -2.000s; }
//! .calendar.calendar-beckon .calendar-day9  { animation: anim-beckon 3s infinite -1.875s; }
//! .calendar.calendar-beckon .calendar-day10 { animation: anim-beckon 3s infinite -1.750s; }
//! .calendar.calendar-beckon .calendar-day11 { animation: anim-beckon 3s infinite -1.625s; }
//! .calendar.calendar-beckon .calendar-day12 { animation: anim-beckon 3s infinite -1.500s; }
//! .calendar.calendar-beckon .calendar-day13 { animation: anim-beckon 3s infinite -1.375s; }
//! .calendar.calendar-beckon .calendar-day14 { animation: anim-beckon 3s infinite -1.250s; }
//! .calendar.calendar-beckon .calendar-day15 { animation: anim-beckon 3s infinite -1.125s; }
//! .calendar.calendar-beckon .calendar-day16 { animation: anim-beckon 3s infinite -1.000s; }
//! .calendar.calendar-beckon .calendar-day17 { animation: anim-beckon 3s infinite -0.875s; }
//! .calendar.calendar-beckon .calendar-day18 { animation: anim-beckon 3s infinite -0.750s; }
//! .calendar.calendar-beckon .calendar-day19 { animation: anim-beckon 3s infinite -0.625s; }
//! .calendar.calendar-beckon .calendar-day20 { animation: anim-beckon 3s infinite -0.500s; }
//! .calendar.calendar-beckon .calendar-day21 { animation: anim-beckon 3s infinite -0.375s; }
//! .calendar.calendar-beckon .calendar-day22 { animation: anim-beckon 3s infinite -0.250s; }
//! .calendar.calendar-beckon .calendar-day23 { animation: anim-beckon 3s infinite -0.125s; }
//! .calendar.calendar-beckon .calendar-day24 { animation: anim-beckon 3s infinite -0.000s; }
//! .calendar.calendar-beckon > span { animation-name: anim-beckon-dark ! important; }
//! @keyframes anim-beckon {
//!   0%   { color: #666666; }
//!   80%  { color: #666666; }
//!   90%  { color: #00ff00; }
//!   100% { color: #666666; }
//! }
//! @keyframes anim-beckon-dark {
//!   0%   { color: #333333; }
//!   80%  { color: #333333; }
//!   90%  { color: #009900; }
//!   100% { color: #333333; }
//! }
//!
//! .calendar-bkg {
//!   position: fixed;
//!   left: 0;
//!   right: 0;
//!   bottom: 0;
//!   z-index: -1;
//!   color: #ffffff;
//!   cursor: default;
//!   -webkit-user-select: none;
//!   -khtml-user-select: none;
//!   -moz-user-select: -moz-none;
//!   -o-user-select: none;
//!   user-select: none;
//! }
//! .calendar-bkg div:nth-child(1) { opacity: .04; }
//! .calendar-bkg div:nth-child(2) { opacity: .08; }
//! .calendar-bkg div:nth-child(3) { opacity: .12; }
//! .calendar-bkg div:nth-child(4) { opacity: .16; }
//! .calendar-bkg div {
//!   text-align: justify;
//!   margin-bottom: -1.2em;
//! }
//! .calendar-bkg div::after {
//!   content: "_";
//!   display: inline-block;
//!   visibility: hidden;
//!   width: 100%;
//! }
//! </style>
//!
//! <pre class="calendar">
//! <a href="d25/index.html" class="calendar-day25">                        *                          <span class="calendar-day">25</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d24/index.html" class="calendar-day24">                       &gt;<span class="calendar-ornament1">o</span>&lt;                         <span class="calendar-day">24</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d23/index.html" class="calendar-day23">                      &gt;&gt;<span class="calendar-ornament0">O</span>&lt;&lt;                        <span class="calendar-day">23</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d22/index.html" class="calendar-day22">                     &gt;<span class="calendar-ornament2">@</span>&gt;&gt;<span class="calendar-ornament0">O</span>&lt;&lt;                       <span class="calendar-day">22</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d21/index.html" class="calendar-day21">                    &gt;<span class="calendar-ornament3">*</span>&lt;&lt;<span class="calendar-ornament2">@</span>&gt;&gt;<span class="calendar-ornament2">@</span>&lt;                      <span class="calendar-day">21</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d20/index.html" class="calendar-day20">                   &gt;<span class="calendar-ornament2">@</span>&lt;&lt;&lt;<span class="calendar-ornament3">*</span>&gt;<span class="calendar-ornament2">@</span>&lt;&lt;&lt;                     <span class="calendar-day">20</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d19/index.html" class="calendar-day19">                  &gt;<span class="calendar-ornament0">O</span>&gt;&gt;<span class="calendar-ornament3">*</span>&gt;&gt;&gt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;&lt;                    <span class="calendar-day">19</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d18/index.html" class="calendar-day18">                 &gt;<span class="calendar-ornament2">@</span>&lt;<span class="calendar-ornament2">@</span>&lt;&lt;&lt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;<span class="calendar-ornament2">@</span>&lt;&lt;&lt;                   <span class="calendar-day">18</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d17/index.html" class="calendar-day17">                &gt;&gt;<span class="calendar-ornament2">@</span>&lt;&lt;&lt;<span class="calendar-ornament1">o</span>&gt;&gt;<span class="calendar-ornament2">@</span>&gt;&gt;<span class="calendar-ornament1">o</span>&gt;&gt;<span class="calendar-ornament3">*</span>&lt;                  <span class="calendar-day">17</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d16/index.html" class="calendar-day16">               &gt;&gt;<span class="calendar-ornament3">*</span>&lt;&lt;<span class="calendar-ornament0">O</span>&gt;&gt;<span class="calendar-ornament0">O</span>&gt;<span class="calendar-ornament0">O</span>&lt;&lt;<span class="calendar-ornament2">@</span>&lt;&lt;&lt;<span class="calendar-ornament0">O</span>&lt;                 <span class="calendar-day">16</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d15/index.html" class="calendar-day15">              &gt;<span class="calendar-ornament1">o</span>&gt;&gt;<span class="calendar-ornament1">o</span>&gt;&gt;<span class="calendar-ornament0">O</span>&lt;<span class="calendar-ornament1">o</span>&gt;&gt;<span class="calendar-ornament2">@</span>&lt;<span class="calendar-ornament3">*</span>&lt;&lt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;                <span class="calendar-day">15</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d14/index.html" class="calendar-day14">             &gt;&gt;<span class="calendar-ornament0">O</span>&lt;&lt;<span class="calendar-ornament0">O</span>&lt;<span class="calendar-ornament2">@</span>&gt;&gt;<span class="calendar-ornament0">O</span>&lt;&lt;<span class="calendar-ornament2">@</span>&gt;&gt;&gt;<span class="calendar-ornament1">o</span>&lt;&lt;<span class="calendar-ornament2">@</span>&lt;&lt;               <span class="calendar-day">14</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d13/index.html" class="calendar-day13">            &gt;<span class="calendar-ornament3">*</span>&lt;&lt;<span class="calendar-ornament0">O</span>&lt;&lt;&lt;<span class="calendar-ornament2">@</span>&gt;&gt;&gt;<span class="calendar-ornament1">o</span>&gt;&gt;&gt;<span class="calendar-ornament3">*</span>&lt;&lt;&lt;<span class="calendar-ornament1">o</span>&lt;&lt;<span class="calendar-ornament3">*</span>&lt;              <span class="calendar-day">13</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d12/index.html" class="calendar-day12">           &gt;&gt;<span class="calendar-ornament3">*</span>&lt;<span class="calendar-ornament3">*</span>&lt;&lt;&lt;<span class="calendar-ornament1">o</span>&gt;&gt;&gt;<span class="calendar-ornament2">@</span>&lt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;<span class="calendar-ornament3">*</span>&gt;&gt;&gt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;&lt;             <span class="calendar-day">12</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d11/index.html" class="calendar-day11">          &gt;&gt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;<span class="calendar-ornament3">*</span>&lt;&lt;&lt;<span class="calendar-ornament2">@</span>&lt;&lt;<span class="calendar-ornament0">O</span>&gt;&gt;&gt;<span class="calendar-ornament1">o</span>&lt;&lt;<span class="calendar-ornament2">@</span>&lt;&lt;<span class="calendar-ornament1">o</span>&gt;<span class="calendar-ornament3">*</span>&lt;&lt;&lt;            <span class="calendar-day">11</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d10/index.html" class="calendar-day10">         &gt;<span class="calendar-ornament3">*</span>&gt;&gt;<span class="calendar-ornament2">@</span>&gt;&gt;&gt;<span class="calendar-ornament0">O</span>&lt;&lt;<span class="calendar-ornament1">o</span>&lt;<span class="calendar-ornament2">@</span>&lt;<span class="calendar-ornament3">*</span>&lt;<span class="calendar-ornament0">O</span>&lt;&lt;<span class="calendar-ornament1">o</span>&lt;&lt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;<span class="calendar-ornament3">*</span>&lt;&lt;&lt;           <span class="calendar-day">10</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d09/index.html" class="calendar-day9">        &gt;<span class="calendar-ornament3">*</span>&gt;<span class="calendar-ornament1">o</span>&gt;&gt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;<span class="calendar-ornament1">o</span>&lt;&lt;<span class="calendar-ornament2">@</span>&gt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;<span class="calendar-ornament2">@</span>&gt;&gt;&gt;<span class="calendar-ornament0">O</span>&lt;<span class="calendar-ornament1">o</span>&gt;<span class="calendar-ornament0">O</span>&gt;&gt;&gt;<span class="calendar-ornament2">@</span>&lt;          <span class="calendar-day"> 9</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d08/index.html" class="calendar-day8">       &gt;<span class="calendar-ornament1">o</span>&gt;<span class="calendar-ornament0">O</span>&lt;<span class="calendar-ornament3">*</span>&gt;&gt;<span class="calendar-ornament0">O</span>&gt;&gt;<span class="calendar-ornament1">o</span>&gt;<span class="calendar-ornament1">o</span>&gt;<span class="calendar-ornament3">*</span>&gt;&gt;<span class="calendar-ornament0">O</span>&gt;&gt;&gt;<span class="calendar-ornament3">*</span>&gt;&gt;<span class="calendar-ornament0">O</span>&lt;&lt;<span class="calendar-ornament0">O</span>&gt;&gt;&gt;<span class="calendar-ornament1">o</span>&lt;&lt;         <span class="calendar-day"> 8</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d07/index.html" class="calendar-day7">      &gt;&gt;<span class="calendar-ornament2">@</span>&gt;<span class="calendar-ornament3">*</span>&lt;&lt;<span class="calendar-ornament0">O</span>&lt;<span class="calendar-ornament3">*</span>&gt;<span class="calendar-ornament1">o</span>&gt;&gt;&gt;<span class="calendar-ornament3">*</span>&gt;&gt;&gt;<span class="calendar-ornament0">O</span>&gt;&gt;<span class="calendar-ornament2">@</span>&gt;<span class="calendar-ornament1">o</span>&lt;<span class="calendar-ornament0">O</span>&gt;<span class="calendar-ornament2">@</span>&lt;&lt;<span class="calendar-ornament2">@</span>&gt;<span class="calendar-ornament3">*</span>&lt;<span class="calendar-ornament2">@</span>&lt;        <span class="calendar-day"> 7</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d06/index.html" class="calendar-day6">     &gt;&gt;<span class="calendar-ornament2">@</span>&lt;&lt;<span class="calendar-ornament0">O</span>&lt;&lt;&lt;<span class="calendar-ornament0">O</span>&lt;<span class="calendar-ornament3">*</span>&lt;&lt;&lt;<span class="calendar-ornament0">O</span>&lt;&lt;&lt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;<span class="calendar-ornament1">o</span>&gt;&gt;&gt;<span class="calendar-ornament0">O</span>&gt;&gt;&gt;<span class="calendar-ornament0">O</span>&gt;&gt;&gt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;       <span class="calendar-day"> 6</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d05/index.html" class="calendar-day5 calendar-complete">    &gt;<span class="calendar-ornament0">O</span>&gt;&gt;&gt;<span class="calendar-ornament0">O</span>&gt;&gt;&gt;<span class="calendar-ornament1">o</span>&lt;&lt;&lt;<span class="calendar-ornament2">@</span>&lt;&lt;&lt;<span class="calendar-ornament0">O</span>&lt;<span class="calendar-ornament0">O</span>&gt;<span class="calendar-ornament1">o</span>&lt;<span class="calendar-ornament1">o</span>&gt;<span class="calendar-ornament0">O</span>&gt;<span class="calendar-ornament1">o</span>&lt;&lt;<span class="calendar-ornament2">@</span>&gt;&gt;&gt;<span class="calendar-ornament0">O</span>&gt;<span class="calendar-ornament1">o</span>&gt;&gt;<span class="calendar-ornament2">@</span>&lt;      <span class="calendar-day"> 5</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d04/index.html" class="calendar-day4 calendar-verycomplete">   &gt;<span class="calendar-ornament2">@</span>&gt;&gt;<span class="calendar-ornament2">@</span>&gt;&gt;&gt;<span class="calendar-ornament1">o</span>&lt;<span class="calendar-ornament2">@</span>&gt;&gt;&gt;<span class="calendar-ornament2">@</span>&lt;&lt;<span class="calendar-ornament0">O</span>&lt;<span class="calendar-ornament0">O</span>&gt;&gt;<span class="calendar-ornament0">O</span>&lt;&lt;&lt;<span class="calendar-ornament2">@</span>&lt;<span class="calendar-ornament1">o</span>&lt;&lt;<span class="calendar-ornament2">@</span>&lt;&lt;<span class="calendar-ornament1">o</span>&lt;<span class="calendar-ornament0">O</span>&gt;&gt;<span class="calendar-ornament2">@</span>&gt;<span class="calendar-ornament1">o</span>&lt;     <span class="calendar-day"> 4</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d03/index.html" class="calendar-day3 calendar-verycomplete">  &gt;<span class="calendar-ornament3">*</span>&gt;<span class="calendar-ornament2">@</span>&lt;&lt;<span class="calendar-ornament3">*</span>&gt;<span class="calendar-ornament0">O</span>&lt;&lt;&lt;<span class="calendar-ornament0">O</span>&lt;&lt;&lt;<span class="calendar-ornament2">@</span>&lt;<span class="calendar-ornament3">*</span>&gt;<span class="calendar-ornament3">*</span>&lt;&lt;<span class="calendar-ornament3">*</span>&gt;&gt;&gt;<span class="calendar-ornament2">@</span>&lt;&lt;<span class="calendar-ornament3">*</span>&lt;&lt;&lt;<span class="calendar-ornament3">*</span>&lt;&lt;&lt;<span class="calendar-ornament2">@</span>&lt;&lt;&lt;<span class="calendar-ornament3">*</span>&lt;&lt;    <span class="calendar-day"> 3</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d02/index.html" class="calendar-day2 calendar-verycomplete"> &gt;<span class="calendar-ornament2">@</span>&lt;<span class="calendar-ornament0">O</span>&lt;<span class="calendar-ornament3">*</span>&lt;&lt;&lt;<span class="calendar-ornament2">@</span>&lt;&lt;&lt;<span class="calendar-ornament1">o</span>&lt;<span class="calendar-ornament0">O</span>&gt;<span class="calendar-ornament0">O</span>&gt;&gt;<span class="calendar-ornament0">O</span>&lt;&lt;<span class="calendar-ornament2">@</span>&lt;&lt;&lt;<span class="calendar-ornament2">@</span>&lt;&lt;&lt;<span class="calendar-ornament0">O</span>&lt;&lt;<span class="calendar-ornament2">@</span>&gt;&gt;<span class="calendar-ornament0">O</span>&gt;&gt;&gt;<span class="calendar-ornament1">o</span>&lt;<span class="calendar-ornament3">*</span>&lt;&lt;&lt;   <span class="calendar-day"> 2</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d01/index.html" class="calendar-day1 calendar-verycomplete">&gt;<span class="calendar-ornament2">@</span>&gt;&gt;<span class="calendar-ornament2">@</span>&gt;<span class="calendar-ornament2">@</span>&lt;&lt;<span class="calendar-ornament0">O</span>&lt;&lt;<span class="calendar-ornament3">*</span>&gt;<span class="calendar-ornament0">O</span>&gt;<span class="calendar-ornament2">@</span>&lt;&lt;<span class="calendar-ornament1">o</span>&gt;&gt;&gt;<span class="calendar-ornament0">O</span>&lt;&lt;&lt;<span class="calendar-ornament2">@</span>&gt;&gt;<span class="calendar-ornament0">O</span>&gt;&gt;&gt;<span class="calendar-ornament3">*</span>&gt;<span class="calendar-ornament3">*</span>&gt;&gt;&gt;<span class="calendar-ornament3">*</span>&lt;<span class="calendar-ornament3">*</span>&lt;&lt;&lt;<span class="calendar-ornament0">O</span>&lt;&lt;  <span class="calendar-day"> 1</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <span class="calendar-trunk">                      |   |
//!                       |   |
//!            _  _ __ ___|___|___ __ _  _           </span>
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
