" Stones vim syntax file
" Language: stones
" Maintainer: Zack Hixon <zackh@firemail.cc>

syn case match

syn keyword STodo TODO NOTE FIXME HACK OHMYGOODNESS
syn keyword SInt red
syn keyword SConditional purple
syn keyword SOperator yellow
syn keyword SFunction green blue orange
syn keyword SDirection up down left right
syn match SComment /^red\|^orange\|^yellow\|^green\|^blue\|^purple\|[^123]\|^up\|^down\|^left\|^right\|^\s/
syn match SNumber /[123]/

hi def link STodo Todo
hi def link SInt Number
hi def link SConditional Type
hi def link SOperator Operator
hi def link SFunction Function
hi def link SNth Function
hi def link SDirection Identifier
hi def link SNumber Number
hi def link SComment Comment

" these syntax definitions are awful, deal with it

