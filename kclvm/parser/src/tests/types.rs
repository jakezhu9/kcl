use crate::tests::{parse_type_node_snapshot, parse_type_snapshot};

parse_type_snapshot!(basic_type_0, r####"bool"####);
parse_type_snapshot!(basic_type_1, r####"int"####);
parse_type_snapshot!(basic_type_2, r####"float"####);
parse_type_snapshot!(basic_type_3, r####"str"####);
parse_type_snapshot!(any_type, r####"any"####);
parse_type_snapshot!(list_type_0, r####"[]"####);
parse_type_snapshot!(list_type_1, r####"[int]"####);
parse_type_snapshot!(list_type_2, r####"[any]"####);
parse_type_snapshot!(list_type_3, r####"[[]]"####);
parse_type_snapshot!(list_type_4, r####"[[str]]"####);
parse_type_snapshot!(dict_type_0, r####"{:}"####);
parse_type_snapshot!(dict_type_1, r####"{str:}"####);
parse_type_snapshot!(dict_type_2, r####"{:[]}"####);
parse_type_snapshot!(dict_type_3, r####"{str:{:float}}"####);
parse_type_snapshot!(dict_type_4, r####"{str:{:float}, int:[]}"####);
parse_type_snapshot!(union_type_0, r####"int|str"####);
parse_type_snapshot!(union_type_1, r####"int | str | [] | {:}"####);
parse_type_snapshot!(named_type_0, r####"Person"####);
parse_type_snapshot!(named_type_1, r####"some.pkg.Person"####);
parse_type_snapshot!(literal_type_0, r####"True"####);
parse_type_snapshot!(literal_type_1, r####"False"####);
parse_type_snapshot!(literal_type_2, r####"123"####);
parse_type_snapshot!(literal_type_3, r####"123.0"####);
parse_type_snapshot!(literal_type_4, r####""abc""####);
parse_type_snapshot!(literal_type_5, r####"''"####);

parse_type_node_snapshot!(type_str_0, r####"int"####);
parse_type_node_snapshot!(type_str_1, r####"  int    "####);
parse_type_node_snapshot!(type_str_2, r####"bool | True |  int  | str|str"####);
parse_type_node_snapshot!(type_str_3, r####"[ [{str: float}] | int]"####);
