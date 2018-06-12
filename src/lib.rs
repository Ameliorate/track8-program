#![feature(try_from)]#[macro_use]extern crate derive_more;
use std::io::{BufRead,self};use std::fmt::{Display, self};use std::convert::TryInto;use std::iter::Iterator;use std::num::ParseIntError;#[cfg(test)]mod test;
#[doc="Represents a single character in one of the programs on the tape.\n"]
#[doc="Because unicode doesn't define valid characters for every single 32-bit bitmask, this value allows switching between interpreting the character as a number or as a unicode codepoint."]
#[derive(Clone,Copy,Debug,PartialOrd,Ord,PartialEq,Eq,From)]pub enum C{C(char),N(u32)}
impl C{#[doc="Converts self into a C::C when self is C::N, identity otherwise\n\n#Panics\nWhen `C::N` is an invalid unicode codepoint."]
 pub fn norm(&self)->C{match self{C::C(c)=>C::C(*c),C::N(_n)=>C::C((*self).into())}}}
impl From<C> for char{fn from(c:C)->char{match c{C::C(c)=>c,C::N(n)=>n.try_into().expect(&format!("{} not valid unicode codepoint",n))}}}
impl From<C> for u32{fn from(c:C)->u32{match c{C::N(n)=>n,C::C(c)=>c.into()}}} impl Display for C{fn fmt(&self,f:&mut fmt::Formatter)->Result<(),fmt::Error>{char::from(*self).fmt(f)}}
#[derive(Clone,Debug,PartialOrd,Ord,PartialEq,Eq,From,Into)]pub struct CSt(Vec<C>);impl CSt{pub fn n(&self)->Result<u32,ParseIntError>{self.to_string().parse()}}
impl Display for CSt{fn fmt(&self,f:&mut fmt::Formatter)->Result<(),fmt::Error>{let g:String=self.clone().0.into_iter().map(|v|char::from(v)).collect(); g.fmt(f)}}
#[derive(Clone,Debug,PartialOrd,Ord,PartialEq,Eq)]pub enum S{Main,Read{buf:CSt},Write{buf:CSt},Stack{buf:CSt},Print{escap:bool,buf:CSt},Stopped}impl Default for S{fn default()->S{S::Main}}
#[derive(Clone,Debug,PartialOrd,Ord,PartialEq,Eq,Default)]pub struct I{pub program:usize,pub head_pos:usize,pub tape:Vec<Vec<C>>,pub state:S,pub stack:Vec<C>,pub max_stack:usize,pub max_programs:usize}
#[derive(Clone,Copy,Debug,PartialOrd,Ord,PartialEq,Eq)]pub enum R{Running,Stopped}impl Default for R{fn default()->R{R::Stopped}}
impl I{pub fn new()->I{let mut i=I::default();i.max_stack=8;i.max_programs=8;i}
 pub fn rd<R:io::Read>(&mut self,r:R){let b=io::BufReader::new(r);for(i,l)in(1..).zip(b.lines()){if i>self.max_programs{panic!("max programs")}
   self.tape.push(l.unwrap().chars().map(|c|C::C(c)).collect::<Vec<_>>().into())};self.tape.resize(self.max_programs,vec![]);
  let targ=self.tape.iter().map(|v|v.len()).fold(0,|a,v|if v>a{v}else{a});self.tape.iter_mut().map(|v|v.resize(targ,C::C(' '))).collect()}
 pub fn sp(&mut self)->C{self.stack.pop().expect("empty stack")}
 pub fn step(&mut self)->R{let c=self.tape[self.program][self.head_pos];let mut ns=self.state.clone();match self.state{
  S::Stopped=>return R::Stopped,S::Main=>{match c.into(){'"'=>ns=S::Print{escap:false,buf:vec![].into()},'>'=>ns=S::Stack{buf:vec![].into()},
   '|'=>ns=S::Read{buf:vec![].into()},']'=>ns=S::Write{buf:vec![].into()},'!'=>{let a=self.stack.pop().expect("empty stack");if u32::from(a)==0{self.stack.push(1.into())}else{self.stack.push(0.into())}}
   '#'=>if self.tape.len()-1!=self.program{self.program+=1}else{ns=S::Stopped},'^'=>if self.program!=0{self.program-=1}else{ns=S::Stopped}
   ','=>{let _=self.stack.pop();()},'~'=>{let v=self.sp();self.stack.push(v);self.stack.push(v)},'d'=>{let a=self.sp();print!("{}",u32::from(a))},'D'=>{let a=self.sp();eprint!("{}",u32::from(a))}
   '+'=>{let a=self.sp();let b=self.sp();self.stack.push(C::from(u32::from(a)+u32::from(b)))},'-'=>{let a=self.sp();let b=self.sp();self.stack.push(C::from(u32::from(a)-u32::from(b)))}
   '*'=>{let a=self.sp();let b=self.sp();self.stack.push(C::from(u32::from(a)* u32::from(b)))},'%'=>{let a=self.sp();let b=self.sp();self.stack.push(C::from(u32::from(a)/u32::from(b)))}
   '/'=>{let a=self.sp();if a!=C::N(0){if self.program!=0{self.program-=1}else{ns=S::Stopped}}},
   '\\'=>{let a=self.sp();if a!=C::N(0){if self.tape.len()-1!=self.program{self.program+=1}else{ns=S::Stopped}}},
   '='=>{let a=self.sp();let b=self.sp();if u32::from(a)==u32::from(b){self.stack.push(1.into())}else{self.stack.push(0.into())}},' '|'\0'|'\t'=>{},c=>panic!("unknown instruction {}",c)}}
  S::Print{escap,ref buf}=>{let mut b:Vec<C>=buf.clone().into();let mut es_new=false;let mut repla=true;if!escap{match c.into(){'\\'=>es_new=true,
    '"'=>{ns=S::Main;repla=false;print!("{}",CSt::from(b.clone()))},'`'=>{ns=S::Main;repla=false;eprint!("{}",CSt::from(b.clone()))},_=>b.push(c)}}
   else{b.push(C::from(match c.into(){'n'=>'\n','`'=>'`','"'=>'"','\\'=>'\\',c=>panic!("unknown escape character \\{}",c)}))}
   if repla{ns=S::Print{escap:es_new,buf:b.into()}}},
  S::Stack{ref buf}=>{let mut b:Vec<C>=buf.clone().into();match c.into(){'0'...'9'=>{b.push(c);ns=S::Stack{buf:b.into()}}
    '.'=>{ns=S::Main;self.stack.push(CSt::from(b).n().unwrap().into());if self.stack.len()>self.max_stack{panic!("max stack entries exceeded")}}
    c=>panic!("unknown character in stack push: {}",c)}},
  S::Read{ref buf}=>{let mut b:Vec<C>=buf.clone().into();match c.into(){'0'...'9'=>{b.push(c);ns=S::Read{buf:b.into()}}
    '.'=>{ns=S::Main;self.stack.push(self.tape[(CSt::from(b).n().unwrap()+1)as usize][self.head_pos])},
    c=>panic!("unknown character in tape read: {}",c)}},
  S::Write{ref buf}=>{let mut b:Vec<C>=buf.clone().into();match c.into(){'0'...'9'=>{b.push(c);ns=S::Write{buf:b.into()}}
    '.'=>{ns=S::Main;self.tape[(CSt::from(b).n().unwrap()+1)as usize][self.head_pos]=self.stack.pop().expect("empty stack")},
  c=>panic!("unknown character in tape read: {}",c)}}}
 self.state=ns;if self.head_pos!=self.tape[self.program].len()-1{self.head_pos+=1}else{self.head_pos=0}R::Running}}
