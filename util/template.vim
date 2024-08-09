" custom syntax highlighting for .template files.
" save this file to syntax folder:
" /usr/share/nvim/runtime/syntax/mailtemplate.vim (nvim)
" /usr/share/vim/vimfiles/syntax/mailtemplate.vim (vim)
"
" use with
"   :setfiletype mailtemplate


syn match tlocale /{[^{]*}/ contained
syn match tvariable /{{[^{]*}}/ contained

syn match tlocaleName /^\v\@\w*\ze:\n/ nextgroup=tsubjectA contained
syn match tsubjectA /\v:\n^(\s*).*$/ contains=tsubject contained
syn match tsubject /\v^[ ]*\zs.*/ contains=tvariable contained
syn match tlocalizationKey /\v^\s*(\w|\.)+/ nextgroup=tlocalizationValue contained
syn match tlocalizationValue /\v(\=)@<=.*$/ contains=tvariable contained

syn region tlocalizationField start=/^@[A-Za-z_]\+\s*:/ end=/^$/ contains=tlocaleName, tlocalizationKey, tlocalizationValue


syn include @html syntax/html.vim

syn match tplaintextregion /\v\#PLAINTEXT(\n|.){-}\#END/ contains=tvariable, tlocale, tregion
syn region thtmlregion start='#HTML' end='#END' contains=tvariable, tlocale, tregion, @html

syn match tregion /^#HTML$/ contained
syn match tregion /^#PLAINTEXT$/ contained
syn match tregion /^#END$/ contained


hi def link tlocale                 Identifier
hi def link tvariable               Type
hi def link tregion                 Keyword

hi def link tlocaleName             Keyword
hi def link tsubject                Underlined
hi def link tlocalizationKey        String
hi def link tlocalizationValue      Value
