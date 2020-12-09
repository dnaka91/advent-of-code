//! # Year 2020
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
//! .calendar .calendar-color-b { color:#333399; }
//! .calendar .calendar-color-l { color:#ccccff; }
//! .calendar .calendar-color-r { color:#ff0000; }
//! .calendar .calendar-color-w { color:#ffffff; }
//! .calendar .calendar-color-g { color:#00cc00; }
//! .calendar .calendar-color-a { color:#cccccc; }
//! </style>
//!
//! <pre class="calendar">
//! <a href="d01/index.html" class="calendar-day1 calendar-verycomplete">              <span class="calendar-color-l">..........</span><span class="calendar-color-r">|</span><span class="calendar-color-l">..........</span>                <span class="calendar-day"> 1</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d02/index.html" class="calendar-day2 calendar-verycomplete">   <span class="calendar-color-l">.....''''''</span> <span class="calendar-color-w">.'</span>  <span class="calendar-color-w">-</span>  <span class="calendar-color-w">-</span>  <span class="calendar-color-a">\</span><span class="calendar-color-w">-</span> <span class="calendar-color-w">.''</span><span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span> <span class="calendar-color-l">''''''.....</span>     <span class="calendar-day"> 2</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d03/index.html" class="calendar-day3 calendar-verycomplete"><span class="calendar-color-l">'''</span> <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span> <span class="calendar-color-w">'.'.</span> <span class="calendar-color-w">-</span>   <span class="calendar-color-w">-</span> <span class="calendar-color-a">\</span> <span class="calendar-color-w">-'':</span>  <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>  <span class="calendar-color-l">'''</span>  <span class="calendar-day"> 3</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d04/index.html" class="calendar-day4 calendar-verycomplete"> <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span> <span class="calendar-color-w">''..'''</span><span class="calendar-color-a">_[]</span><span class="calendar-color-w">.'</span>  <span class="calendar-color-b">~</span>    <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>   <span class="calendar-day"> 4</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d05/index.html" class="calendar-day5 calendar-verycomplete"><span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>  <span class="calendar-color-g">.'.</span> <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span> <span class="calendar-color-a">____/</span> <span class="calendar-color-w">''</span>  <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>     <span class="calendar-color-b">~</span>    <span class="calendar-color-b">~</span>  <span class="calendar-day"> 5</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d06/index.html" class="calendar-day6 calendar-verycomplete">  <span class="calendar-color-b">~</span>    <span class="calendar-color-b">~</span> <span class="calendar-color-g">''</span>  <span class="calendar-color-g">..</span><span class="calendar-color-a">_____/</span> <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>      <span class="calendar-color-b">~</span>           <span class="calendar-day"> 6</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d07/index.html" class="calendar-day7 calendar-verycomplete"> <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span> <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span> <span class="calendar-color-g">:</span><span class="calendar-color-a">[]</span><span class="calendar-color-g">'.</span>   <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span>      <span class="calendar-color-b">~</span>                   <span class="calendar-day"> 7</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <a href="d08/index.html" class="calendar-day8 calendar-verycomplete">       <span class="calendar-color-b">~</span>     <span class="calendar-color-g">'.</span><span class="calendar-color-a">\</span> <span class="calendar-color-b">~</span>        <span class="calendar-color-b">~</span>  <span class="calendar-color-b">~</span>                     <span class="calendar-day"> 8</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <span aria-hidden="true" class=""> <span class="calendar-color-b">~</span>   <span class="calendar-color-b">~</span>      <span class="calendar-color-b">~</span>   <span class="calendar-color-a">\</span>  <span class="calendar-color-b">~</span>                             </span>
//! <a href="d09/index.html" class="calendar-day9 calendar-verycomplete">               <span class="calendar-color-b">~</span> <span class="calendar-color-a">\</span>                                 <span class="calendar-day"> 9</span> <span class="calendar-mark-complete">*</span><span class="calendar-mark-verycomplete">*</span></a>
//! <span aria-hidden="true" class="">                  <span class="calendar-color-a">\</span> <span class="calendar-color-b">~</span>                            </span>
//! <span aria-hidden="true" class="">                   <span class="calendar-color-a">\</span>                             </span>
//! <span aria-hidden="true" class="">                   ~<span class="calendar-color-a">\</span>                            </span>
//! <span aria-hidden="true" class="">                     <span class="calendar-color-a">\</span> ~                         </span>
//! <span aria-hidden="true" class="calendar-day10">                   ~  <span class="calendar-color-a">\</span>   ~                        <span class="calendar-day">10</span></span>
//! <span aria-hidden="true" class="">                           .                     </span>
//! <span aria-hidden="true" class="">                     <span class="calendar-color-b">~</span>  .<span class="calendar-color-g">'</span>  <span class="calendar-color-g">:</span>                    </span>
//! <span aria-hidden="true" class="">                       <span class="calendar-color-g">:</span>   <span class="calendar-color-g">.</span>'                    </span>
//! <span aria-hidden="true" class="">                         <span class="calendar-color-g">'</span>     <span class="calendar-color-b">~</span>                 </span>
//! <span aria-hidden="true" class="">                                                 </span>
//! <span aria-hidden="true" class="">                                                 </span>
//! <span aria-hidden="true" class="">                                <span class="calendar-color-b">~</span>                </span>
//! <span aria-hidden="true" class="">                           ~                     </span>
//! <span aria-hidden="true" class="">                             <span class="calendar-color-g">.</span> ..   <span class="calendar-color-g">.</span>  <span class="calendar-color-g">.</span>        ~</span>
//! <span aria-hidden="true" class="">                              '<span class="calendar-color-g">.</span>            <span class="calendar-color-b">~</span>    </span>
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
