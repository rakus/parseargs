#!/bin/sh
#
# Test parseargs basic functionallities
#
# shellcheck disable=SC2016

script_dir="$(cd "$(dirname "$0")" && pwd)" || exit 1
script_name="$(basename "$0")"

. "$script_dir/_test.shinc"

start_test

opt="a:aaa#aaa"
opt="$opt,b:bbb#bbb"
opt="$opt,c:ccc#ccc"
opt="$opt,d:ddd#ddd"
opt="$opt,e:eee#eee"
opt="$opt,f:fff#fff"
opt="$opt,g:ggg#ggg"
opt="$opt,h:hhh#hhh"
opt="$opt,i:iii#iii"
opt="$opt,j:jjj#jjj"
opt="$opt,k:kkk#kkk"
opt="$opt,l:lll#lll"
opt="$opt,m:mmm#mmm"
opt="$opt,n:nnn#nnn"
opt="$opt,o:ooo#ooo"
opt="$opt,p:ppp#ppp"
opt="$opt,q:qqq#qqq"
opt="$opt,r:rrr#rrr"
opt="$opt,s:sss#sss"
opt="$opt,t:ttt#ttt"
opt="$opt,u:uuu#uuu"
opt="$opt,v:vvv#vvv"
opt="$opt,w:www#www"
opt="$opt,x:xxx#xxx"
opt="$opt,y:yyy#yyy"
opt="$opt,z:zzz#zzz"

opt="$opt,A:AAA#AAA"
opt="$opt,B:BBB#BBB"
opt="$opt,C:CCC#CCC"
opt="$opt,D:DDD#DDD"
opt="$opt,E:EEE#EEE"
opt="$opt,F:FFF#FFF"
opt="$opt,G:GGG#GGG"
opt="$opt,H:HHH#HHH"
opt="$opt,I:III#III"
opt="$opt,J:JJJ#JJJ"
opt="$opt,K:KKK#KKK"
opt="$opt,L:LLL#LLL"
opt="$opt,M:MMM#MMM"
opt="$opt,N:NNN#NNN"
opt="$opt,O:OOO#OOO"
opt="$opt,P:PPP#PPP"
opt="$opt,Q:QQQ#QQQ"
opt="$opt,R:RRR#RRR"
opt="$opt,S:SSS#SSS"
opt="$opt,T:TTT#TTT"
opt="$opt,U:UUU#UUU"
opt="$opt,V:VVV#VVV"
opt="$opt,W:WWW#WWW"
opt="$opt,X:XXX#XXX"
opt="$opt,Y:YYY#YYY"
opt="$opt,Z:ZZZ#ZZZ"


test_pa 'true' -o "$opt" --

test_pa 'test -n "$aaa"' -o "$opt" -- -a
test_pa 'test -n "$aaa"' -o "$opt" -- --aaa
test_pa 'test -n "$ZZZ"' -o "$opt" -- -Z
test_pa 'test -n "$ZZZ"' -o "$opt" -- --ZZZ

end_test

