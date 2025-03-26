with Ada.Text_IO;         use Ada.Text_IO;
with Ada.Integer_Text_IO; use Ada.Integer_Text_IO;

procedure Simple_Calculator is
   A, B, Result : Integer;
   Operator     : Character;
begin
   Put("Ilk sayiyi gir");
   Get(A);
   Skip_Line;  
   Put("Islem girin (+, -, *, /): ");
   Get(Operator);
   Skip_Line;
   Put("Ikinci sayiyi gir");
   Get(B);
   case Operator is
      when '+' =>
         Result := A+B;
      when '-' =>
         Result := A-B;
      when '*' =>
         Result := A*B;
      when '/' =>
         if B = 0 then
            Put_Line("Divided by zero");
            return;
         else
            Result := A/B;
         end if;
      when others =>
         Put_Line("Invalid operator!!");
         return;
   end case;
   Put("Sonuc: ");
   Put(Result);
   New_Line;
end Simple_Calculator;
