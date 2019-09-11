var N=null,E="",T="t",U="u",searchIndex={};
var R=["An iterator over the substrings of a string which, after…","as_str","View the underlying data (the part yet to be iterated) as…","usize","graphemeincomplete","result","option","graphemes","Returns an iterator over substrings of `self` separated on…","Returns an iterator over substrings of `self`, split on…","into_iter","try_from","try_into","borrow_mut","type_id","borrow","typeid","next_back","size_hint","graphemeindices","graphemecursor","uwordbounds","uwordboundindices","unicodesentences","usentencebounds","usentenceboundindices","GraphemeIncomplete","GraphemeIndices","Graphemes","GraphemeCursor","UWordBoundIndices","UWordBounds","UnicodeSegmentation","UnicodeWords","USentenceBounds","USentenceBoundIndices","UnicodeSentences","TruncateToBoundary"];
searchIndex["trunc"]={"doc":E,"i":[[8,R[37],"trunc",E,N,N],[10,"truncate_to_boundary",E,E,0,[[["self"],[R[3]]],["self"]]],[10,"truncate_to_byte_offset",E,E,0,[[["self"],[R[3]]],["self"]]]],"p":[[8,R[37]]]};
searchIndex["unicode_segmentation"]={"doc":"Iterators which split strings on Grapheme Cluster, Word or…","i":[[3,R[28],"unicode_segmentation","External iterator for a string's grapheme clusters.",N,N],[3,R[27],E,"External iterator for grapheme clusters and byte offsets.",N,N],[3,R[29],E,"Cursor-based segmenter for grapheme clusters.",N,N],[3,R[31],E,"External iterator for a string's word boundaries.",N,N],[3,R[30],E,"External iterator for word boundaries and byte offsets.",N,N],[3,R[33],E,R[0],N,N],[3,R[34],E,"External iterator for a string's sentence boundaries.",N,N],[3,R[35],E,"External iterator for sentence boundaries and byte offsets.",N,N],[3,R[36],E,R[0],N,N],[4,R[26],E,"An error return indicating that not enough content was…",N,N],[13,"PreContext",E,"More pre-context is needed. The caller should call…",0,N],[13,"PrevChunk",E,"When requesting `prev_boundary`, the cursor is moving past…",0,N],[13,"NextChunk",E,"When requesting `next_boundary`, the cursor is moving past…",0,N],[13,"InvalidOffset",E,"An error returned when the chunk given does not contain…",0,N],[11,R[1],E,R[2],1,[[["self"]],["str"]]],[11,R[1],E,R[2],2,[[["self"]],["str"]]],[11,"new",E,"Create a new cursor. The string and initial offset are…",3,[[[R[3]],["bool"]],[R[20]]]],[11,"set_cursor",E,"Set the cursor to a new location in the same string.",3,[[["self"],[R[3]]]]],[11,"cur_cursor",E,"The current offset of the cursor. Equal to the last value…",3,[[["self"]],[R[3]]]],[11,"provide_context",E,"Provide additional pre-context when it is needed to decide…",3,[[["self"],["str"],[R[3]]]]],[11,"is_boundary",E,"Determine whether the current cursor location is a…",3,[[["self"],["str"],[R[3]]],[[R[5],["bool",R[4]]],[R[4]],["bool"]]]],[11,"next_boundary",E,"Find the next boundary after the current cursor position.…",3,[[["self"],["str"],[R[3]]],[[R[4]],[R[5],[R[6],R[4]]],[R[6],[R[3]]]]]],[11,"prev_boundary",E,"Find the previous boundary after the current cursor…",3,[[["self"],["str"],[R[3]]],[[R[4]],[R[5],[R[6],R[4]]],[R[6],[R[3]]]]]],[11,R[1],E,R[2],4,[[["self"]],["str"]]],[11,R[1],E,R[2],5,[[["self"]],["str"]]],[17,"UNICODE_VERSION",E,"The version of Unicode that this version of…",N,N],[8,R[32],E,"Methods for segmenting strings according to Unicode…",N,N],[10,R[7],E,"Returns an iterator over the [grapheme…",6,[[["self"],["bool"]],[R[7]]]],[10,"grapheme_indices",E,"Returns an iterator over the grapheme clusters of `self`…",6,[[["self"],["bool"]],[R[19]]]],[10,"unicode_words",E,"Returns an iterator over the words of `self`, separated on…",6,[[["self"]],["unicodewords"]]],[10,"split_word_bounds",E,R[8],6,[[["self"]],[R[21]]]],[10,"split_word_bound_indices",E,R[9],6,[[["self"]],[R[22]]]],[10,"unicode_sentences",E,R[8],6,[[["self"]],[R[23]]]],[10,"split_sentence_bounds",E,R[8],6,[[["self"]],[R[24]]]],[10,"split_sentence_bound_indices",E,R[9],6,[[["self"]],[R[25]]]],[11,R[10],E,E,2,[[],["i"]]],[11,R[11],E,E,2,[[[U]],[R[5]]]],[11,"into",E,E,2,[[],[U]]],[11,"from",E,E,2,[[[T]],[T]]],[11,R[12],E,E,2,[[],[R[5]]]],[11,R[15],E,E,2,[[["self"]],[T]]],[11,R[13],E,E,2,[[["self"]],[T]]],[11,R[14],E,E,2,[[["self"]],[R[16]]]],[11,R[10],E,E,1,[[],["i"]]],[11,R[11],E,E,1,[[[U]],[R[5]]]],[11,"into",E,E,1,[[],[U]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[12],E,E,1,[[],[R[5]]]],[11,R[15],E,E,1,[[["self"]],[T]]],[11,R[13],E,E,1,[[["self"]],[T]]],[11,R[14],E,E,1,[[["self"]],[R[16]]]],[11,R[11],E,E,3,[[[U]],[R[5]]]],[11,"into",E,E,3,[[],[U]]],[11,"from",E,E,3,[[[T]],[T]]],[11,R[12],E,E,3,[[],[R[5]]]],[11,R[15],E,E,3,[[["self"]],[T]]],[11,R[13],E,E,3,[[["self"]],[T]]],[11,R[14],E,E,3,[[["self"]],[R[16]]]],[11,R[10],E,E,5,[[],["i"]]],[11,R[11],E,E,5,[[[U]],[R[5]]]],[11,"into",E,E,5,[[],[U]]],[11,"from",E,E,5,[[[T]],[T]]],[11,R[12],E,E,5,[[],[R[5]]]],[11,R[15],E,E,5,[[["self"]],[T]]],[11,R[13],E,E,5,[[["self"]],[T]]],[11,R[14],E,E,5,[[["self"]],[R[16]]]],[11,R[10],E,E,4,[[],["i"]]],[11,R[11],E,E,4,[[[U]],[R[5]]]],[11,"into",E,E,4,[[],[U]]],[11,"from",E,E,4,[[[T]],[T]]],[11,R[12],E,E,4,[[],[R[5]]]],[11,R[15],E,E,4,[[["self"]],[T]]],[11,R[13],E,E,4,[[["self"]],[T]]],[11,R[14],E,E,4,[[["self"]],[R[16]]]],[11,R[10],E,E,7,[[],["i"]]],[11,R[11],E,E,7,[[[U]],[R[5]]]],[11,"into",E,E,7,[[],[U]]],[11,"from",E,E,7,[[[T]],[T]]],[11,R[12],E,E,7,[[],[R[5]]]],[11,R[15],E,E,7,[[["self"]],[T]]],[11,R[13],E,E,7,[[["self"]],[T]]],[11,R[14],E,E,7,[[["self"]],[R[16]]]],[11,R[10],E,E,8,[[],["i"]]],[11,R[11],E,E,8,[[[U]],[R[5]]]],[11,"into",E,E,8,[[],[U]]],[11,"from",E,E,8,[[[T]],[T]]],[11,R[12],E,E,8,[[],[R[5]]]],[11,R[15],E,E,8,[[["self"]],[T]]],[11,R[13],E,E,8,[[["self"]],[T]]],[11,R[14],E,E,8,[[["self"]],[R[16]]]],[11,R[10],E,E,9,[[],["i"]]],[11,R[11],E,E,9,[[[U]],[R[5]]]],[11,"into",E,E,9,[[],[U]]],[11,"from",E,E,9,[[[T]],[T]]],[11,R[12],E,E,9,[[],[R[5]]]],[11,R[15],E,E,9,[[["self"]],[T]]],[11,R[13],E,E,9,[[["self"]],[T]]],[11,R[14],E,E,9,[[["self"]],[R[16]]]],[11,R[10],E,E,10,[[],["i"]]],[11,R[11],E,E,10,[[[U]],[R[5]]]],[11,"into",E,E,10,[[],[U]]],[11,"from",E,E,10,[[[T]],[T]]],[11,R[12],E,E,10,[[],[R[5]]]],[11,R[15],E,E,10,[[["self"]],[T]]],[11,R[13],E,E,10,[[["self"]],[T]]],[11,R[14],E,E,10,[[["self"]],[R[16]]]],[11,R[11],E,E,0,[[[U]],[R[5]]]],[11,"into",E,E,0,[[],[U]]],[11,"from",E,E,0,[[[T]],[T]]],[11,R[12],E,E,0,[[],[R[5]]]],[11,R[15],E,E,0,[[["self"]],[T]]],[11,R[13],E,E,0,[[["self"]],[T]]],[11,R[14],E,E,0,[[["self"]],[R[16]]]],[11,"fmt",E,E,0,[[["self"],["formatter"]],[R[5]]]],[11,"eq",E,E,0,[[["self"],[R[4]]],["bool"]]],[11,"ne",E,E,0,[[["self"],[R[4]]],["bool"]]],[11,R[17],E,E,1,[[["self"]],[R[6]]]],[11,R[17],E,E,2,[[["self"]],[[R[6],["str"]],["str"]]]],[11,R[17],E,E,7,[[["self"]],[[R[6],["str"]],["str"]]]],[11,R[17],E,E,4,[[["self"]],[R[6]]]],[11,R[17],E,E,5,[[["self"]],[[R[6],["str"]],["str"]]]],[11,"next",E,E,1,[[["self"]],[R[6]]]],[11,R[18],E,E,1,[[["self"]]]],[11,R[18],E,E,2,[[["self"]]]],[11,"next",E,E,2,[[["self"]],[[R[6],["str"]],["str"]]]],[11,"next",E,E,7,[[["self"]],[[R[6],["str"]],["str"]]]],[11,"next",E,E,4,[[["self"]],[R[6]]]],[11,R[18],E,E,4,[[["self"]]]],[11,R[18],E,E,5,[[["self"]]]],[11,"next",E,E,5,[[["self"]],[[R[6],["str"]],["str"]]]],[11,"next",E,E,10,[[["self"]],[[R[6],["str"]],["str"]]]],[11,R[18],E,E,8,[[["self"]]]],[11,"next",E,E,8,[[["self"]],[[R[6],["str"]],["str"]]]],[11,"next",E,E,9,[[["self"]],[R[6]]]],[11,R[18],E,E,9,[[["self"]]]],[11,"clone",E,E,1,[[["self"]],[R[19]]]],[11,"clone",E,E,2,[[["self"]],[R[7]]]],[11,"clone",E,E,3,[[["self"]],[R[20]]]],[11,"clone",E,E,5,[[["self"]],[R[21]]]],[11,"clone",E,E,4,[[["self"]],[R[22]]]],[11,"clone",E,E,10,[[["self"]],[R[23]]]],[11,"clone",E,E,8,[[["self"]],[R[24]]]],[11,"clone",E,E,9,[[["self"]],[R[25]]]]],"p":[[4,R[26]],[3,R[27]],[3,R[28]],[3,R[29]],[3,R[30]],[3,R[31]],[8,R[32]],[3,R[33]],[3,R[34]],[3,R[35]],[3,R[36]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);