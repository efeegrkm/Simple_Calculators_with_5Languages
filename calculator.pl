:- initialization(main).

main :-
    write('Ilk sayiyi gir: '),
    read_line_to_string(user_input, A),
    atom_number(A,A_num),
    write('Islem gir (+, -, *, /): '),
    read_line_to_string(user_input, OpString),
    atom_string(Op, OpString),
    write('Ikinci sayiyi gir: '),
    read_line_to_string(user_input, B),
    atom_number(B,B_num),
    
    calculate(A_num,Op,B_num, Result),
    format('Sonuc: ~w~n',[Result]),
    halt.

calculate(A, '+', B,Result) :- 
    Result is A + B.
calculate(A, '-', B,Result) :- 
    Result is A - B.
calculate(A, '*', B,Result) :- 
    Result is A * B.
calculate(A, '/', B,Result) :- 
    (   B =:= 0
    ->  write('Dhivided by zero exception'), nl,fail
    ;   Result is A / B
    ).
    
calculate(_, Op, _, _) :-
    format('Invalid operator ~w~n',[Op]),
    fail.
