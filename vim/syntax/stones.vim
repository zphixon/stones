" Stones vim syntax file
" Language: stones
" Maintainer: Zack Hixon <zackh@firemail.cc>

syn case match

syn keyword STodo TODO NOTE FIXME HACK OHMYGOODNESS
syn match SRed /red \(up\|down\|left\|right\) \(one\|two\|three\)/
syn match SOrange /orange \(up\|down\|left\|right\) \(one\|two\)/
syn match SYellowGreen /\(yellow\|green\) \(up\|down\|left\|right\)/
syn match SBlue /blue \(up\|down\|left\|right\)/
syn match SPurple /purple \(up\|down\|left\|right\)/

hi def link STodo Todo
hi def link SComment Comment

hi def link SRed Number

hi def link SOrange Operator
hi def link SYellowGreen Operator

hi def link SBlue Function

hi def link SPurple Conditional

