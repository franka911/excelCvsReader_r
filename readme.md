skrypt który będzie pracował na 2 plikach jednocześnie:
 - excel i CVS w którym bedą tabeli ( x wierszy , 2 kolumny)


Skrypt powinien


1. Skanować cały plik CVS po kolei

2. Po napotkaniu sie na pierwszą ramke tekstowa w pliku CVS, 
powinien zacząć szukać takiego samego tekstu w Excel w pierwszej kolumnie, 
po czym gdy go znajdzie, kopiuje zawartośc tego samego wiersza, ale z drugiej kolumny 
i wkleja go w ramkę tekstową CVS (taka podmiana tekstów)


3. Po zakończonej pracy, program podświetla na zielono te wiersze ktore udalo mu sie 
znaleźć w projekcie i podmienić, a na czerwono te których nie znalazł.


4. Program powinien mieć funkcje w które będzie można zaznaczyć, aby podczas
 porównywania ze sobą tekstów, można było uwzględnić że jeżeli np teksty są 
 takie same ale np w jednym jest enter/kilka spacji więcej to wtedy to ignoruje 
 i i tak podmienia teksty, to znaczy aby ignorowal biale znaki.


