---
layout: post
title: "Eine persönliche Anzeigetafel für Tramabfahrtszeiten – Hardware Teil"
author: "Jakob Meier"
categories: IOT
tags: [iot,esp8266,esp-01s,hardware,electronics]
image: 18/tram_station_on_tree.jpg
image_tooltip: 
lang: de
ref: tram-station-board-hardware
nextref: tram-station-board-software
---

<p class="intro">Die Abfahrt des nächsten Trams immer im Blick, dank einer persönlichen Anzeigetafel. Heute wird die Hardware dafür zusammengestellt.</p>

Im [letzten Beitrag]({% include link-by-ref.html ref="esp8266-intro" %}) habe ich das ESP-01S Modul vorgestellt und gezeigt, wie man den darauf installierten ESP8266 Chip programmieren kann. Jetzt geht es um ein konkretes Projekt, welches ich damit realisiert habe: Eine über das Internet aktualisierte LED-Anzeigetafel für Tramabfahrtszeiten.

Der Beitrag behandelt die Zusammenstellung der Hardware. Die Programmierung der Software wird im nächsten Beitrag an der Reihe sein.

## Das Ziel vor Augen

Vorweg schon einmal das fertige Produkt, funktionstüchtig  mit der fertigen Software installiert und eingeschaltet:

![Bild: Ansicht vorne](/assets/img/18/station_board_on.jpg)

Jede LED, die an ist, steht für 10 Sekunden bis zur Ankunft des nächsten Trams, das in Richtung Stadtzentrum fährt. Somit weiss ich immer ganz genau, wie viel Zeit mir noch bleibt, bis ich beispielsweise zur Arbeit aufbrechen muss, ohne die Gefahr, dass mir das Tram gerade vor der Nase wegfährt.

Die Anzeige wird ständig aktualisiert mit den aktuellsten Daten, bereitgestellt durch [Open Data](https://transport.opendata.ch/).

## Verwendete Komponente
 - ESP-01S Modul
 - 8x8 LED Matrix
 - 1x Rohe Lochrasterplatine
 - 3x 74HCN595 8-bit Shift-Register
 - 47Ω und 3.3kΩ Widerstände
 - 0.1 μF Elektrolytkondensator
 
![Bild: Materialien](/assets/img/18/tram_station_components.jpg)

Auf dem Bild zu sehen sind alle Komponente, ausser der LED Matrix. Ausserdem sind auf dem Bild 74HCN595**N**, tatsächlich verwendet wurden aber die funktional identischen, aber kleineren, 74HCN595**D**. Ich empfehle aber sehr, die grösseren Shift-Register zu nehmen, da sie viel einfacher zu verlöten sind und die Abstände der Pins passgenau auf die Löcher der Platine abgestimmt sind. Leider besass ich die grösseren noch nicht, als ich mit dem Löten begann, weshalb ich die kleineren benutzt habe.

## Auslegung der Hauptelemente

Die Auslegung der Komponente sollte für meinen Geschmack vor allem praktisch sein, nicht unbedingt wunderschön. Insbesondere will ich nicht zwingend die ganze Technik verstecken, ein Beobachter darf ruhig direkt ins Herzen des Gerätes sehen. Daher befindet sich das ESP-01S Modul klar ersichtlich vorne und auch der Kondensator schaut oben heraus. Aus praktischen Gründen befindet sich der Rest dann doch auf der Rückseite.

Dort muss nun eine Verbindung der GPIOs des Chips zu den Pins der LED-Matrix gewährleistet werden. Die Herausforderung hierbei: Der ESP-01S hat nur 4 GPIOs aber die LED-Matrix hat 24 Pins zum Steuern(8 Reihen, 8 Kolonnen (rot) und 8 Kolonnen (gelb)). 

An diesem Punkt kommen die 74HCN595 8-bit Shift-Register ins Spiel. Mit nur 3 GPIO können nahezu beliebig viele davon angesteuert werden, wenn man sie in Reihe schaltet. Jedes Register hat dann 8 Ausgänge die mit der LED Matrix verbunden werden können. In unserem Fall brauchen wir genau 3 solcher Register. Die genaue Funktionalität derer werde ich hier aber nicht beschreiben, dazu gibt es genügend ausgezeichnete Ressourcen im Internet und das Datenblatt zum verwendeten Modell sei [hier](https://www.sparkfun.com/datasheets/IC/SN74HC595.pdf) verlinkt.

Hier der Schaltplan mit den Verbindungen der Register und der LED Matrix. Hellgrün ist der Takter (SRCLK), dunkelgrün die Ausgabe (RCLK) und gelb ist, wo die einzelnen Bits in die Register geschrieben werden (SER), beziehungsweise wo sie ans nächste Register weitergeben werden. Jede dieser Farben ist mit einem GPIO des ESP-01S verbunden. 

![Grafik: Verbindung der Shift Register](/assets/img/18/station_board_logic.png)

### Anforderungen an Stromstärke und Leistung

Damit keines der Teile eine zu hohe Leistung abkriegt, braucht es noch ein paar regulierende Komponente.

Zunächst darf jedes Register nicht mehr als 70mA Stromstärke ausgesetzt werden. Dies können wir mit Widerständen regulieren. Das schönste Design wäre dabei, bei jedem Registerausgang einen Widerstand zu platzieren. Dadurch fliesst immer die gleiche Menge Strom durch jeder LED, unabhängig davon wie viele gleichzeitig an sind, womit die LEDs immer gleich hell leuchten.

Der Nachteil dieser Designidee: Sie braucht 24 Widerstände und alle müssen einzeln verlötet werden. Dafür habe ich weder den Platz auf der Platine noch die Geduld in meinem Geist. Also beschloss ich, einen Widerstand vor jedem Shift-Register zu platzieren, womit sich die Arbeit von 24 auf 3 reduziert. Wie wir dank guter Software trotzdem immer die gleiche Helligkeit der LEDs hinbekommen, erkläre ich im nächsten Beitrag.

Wie gross müssen die Widerstände nun sein? Die angeschlossene Spannung ist 3.3V, die Zielstromstärke ist 70mA. Mit dem Ohmschen Gesetz rechnen wir aus: 

\\[ R = {U \over I} = { 3.3V \over 0.07A} = 47.14\Omega \\]

Dies gibt uns den kleinsten Widerstand den wir sicher verwenden können. Da ich gerade Widerstände der Grösse 47Ω zur Hand hatte, nahm ich diese. Die Angaben auf den Widerständen sind ohnehin nicht so genau und Ausfallsicherheit ist auch kein relevantes Thema für mein Hobbyprojekt. Sichererer wäre es aber natürlich, einen etwas grösseren Widerstand zu verwenden, zum Beispiel 56Ω.

Ebenfalls wichtig, die Widerstände die ich verwende, sind auf eine maximale Leistung von 0.25W ausgelegt, bei höheren Werten könnten sie durchbrennen. Damit ergibt eine weitere unterste Limite für den Widerstand. Diese kann wie folgt berechnet werden.

Zuerst die Beziehung zwischen der Leistung *P* und dem Widerstand *R* herleiten:
\\[ P = {U \times I} = {U \times {U \over R}} = {U^2 \over R} \\]

Dann nach dem Widerstand umformen und die Zahlen einfügen.
\\[ R = {U^2 \over P} = { 3.3V \times 3.3V \over 0.25W} = 43.56\Omega\\]

Sehr gut, der Widerstand von 47Ω erfüllt auch diese Bedingung (43.56Ω ist das erlaubte Minimum). Damit ist sichergestellt, dass die Shift-Register und die Widerstände beide nicht durchbrennen.

Die LED-Matrix hätte natürlich auch noch Einschränkungen auf die maximale Stromstärke, aber der maximale Ausgang der Pins der Shift-Register ist bereits kleine genug, dass wir uns nicht weiter darum kümmern müssen.

Noch eine Bemerkung zu den Pins des ESP8266. Diese sind auf 12mA maximale Ausgangsspannung ausgelegt. Für das Ein- und Ausschalten einzelner Pins des Registers ist dies mehr als genug. Es ist aber durchaus notwendig, dass die Shift-Register direkt an die Stromquelle angeschlossen werden und nicht etwa Strom von ESP8266 Pins beziehen. Dies würde die Pins verbrennen und den Chip unbrauchbar machen.

## Schaltplan der Stromversorgung

Auf der vorhin gezeigten Grafik fehlte die Stromversorgung gänzlich. Dieser Schaltplan nun für sich allein, mit den eben berechneten Widerstände am richtigen Ort platziert.

![Grafik: Stromverbindungen](/assets/img/18/station_board_power.png)

Ein paar weitere Überlegungen sind in obige Grafik eingeflossen, weshalb es auch mehr als nur eine positive und eine negative Verbindung pro Register gibt. 

Ersten, die Pins bei jedem Register müssen aktiviert werden, indem der **OE** Pin mit negativer Spannung verbunden wird. 

Zweitens, analog muss der Reset Pin **RST** bei allen Registern an positive Spannung angeschlossen sein.

Somit laufen einmal alle Register. Aber man erinnere sich an den letzten Beitrag, auch die Pins vom ESP-01S müssen in einer gewissen Konfiguration sein beim Start. Insbesondere muss der **IO0** mit positiver Spannung verbunden sein oder der Chip wird nicht richtig starten. Den gleichen Pin haben wir aber bereits als Ausgangspin belegt.

Hier findet nun der 3.3kΩ Widerstand Verwendung. Statt den **IO0** direkt an die Quelle anzuschliessen, wird der Widerstand dazwischen geschaltet. Dadurch können wir den Pin weiterhin als Ausgabepin für die Register verwenden, denn wenn der **IO0** auf *LOW* geht, ist dieses *LOW* dank dem Widerstand an der Quelle, bildlich gesprochen, stark genug um das konstante *HIGH* von der Stromquelle zu bezwingen und den eigenen Wert durchzusetzen. 


## Stützkondensator (Decoupling capacitor)

Um das Stromsignal immer schön stabil zu halten, können verschiedene Arten von Kondensatoren in Gebrauch kommen. Denn sowohl die integrierten Komponenten (in unserem Fall dem ESP-01S und den Shift-Registern) sowie die Stromquelle selbst verursachen Störsignale auf dem Stromnetz. Dies kann dann wiederum zu unerwartetem Verhalten der Einzelteile führen.

In diesem Gebiet bin ich ein blutiger Anfänger und einzig für dieses Projekt habe ich angefangen über Stützkondensatoren (Decoupling capacitors) und Koppelkondensatoren(Coupling capacitors) zu lernen. Grob gesprochen, wird die erste Art verwendet, um die Spitzen im Stromverbrauch zwischen verschiedenen Komponenten auszugleichen, die zweite Art dient dazu die Unregelmässigkeiten direkt von der Stromquelle her zu eliminieren. Soweit mein Verständnis vom Thema. Dem interessierten Leser empfehle ich aber, die Informationen andernorts von Profis einzuholen. Und falls ein Profi dies lesen sollte, darf er oder sie mich auch gerne belehren und ich werde es hier verbessern.

Das vorgestellte Projekt funktioniert wahrscheinlich auch ohne jegliche Kondensatoren, zumindest meistens. Allerdings hatte ich manchmal Unregelmässigkeiten beobachtet beim Ein- oder Ausschalten der LEDs in der Matrix. Diese Unregelmässigkeiten wollte ich dann mit einem Stützkondenstor so nah wie möglich am Plus / Minus des ESP-01S beheben. Die Idee dabei ist, dass der Chip ruckartige Änderungen in der Stromnutzung hat, wenn beispielsweise die Wi-Fi Funktionalität ein- und wieder ausgeschaltet wird. Der Stützkondensator soll dabei helfen, diese Spitzen im Stromnetz etwas zu ebnen und somit die gegenseitige Störung des Chips und des restlichen Schaltkreises zu reduzieren.

Nachdem der Stützkondensator eingebaut wurde und ich später auch noch ein paar Lötstellen nochmals sauber nachgelötete habe, konnte ich keine Unregelmässigkeiten mehr beobachten. Ob der Kondensator wirklich nötig war, kann ich nicht mit Sicherheit sagen. Aufgrund meiner Beobachtungen vermute ich aber zumindest eine positive Wirkung. 

Ebenfalls unklar ist, ob ein Koppelkondensator allenfalls eine bessere Lösung gewesen wäre, was ich nicht mehr ausprobiert habe, denn es hat ja schon alles funktioniert. 😉

## Ergebnis

Zusammenfassend ist dies also der Schaltplan, den ich für mein Projekt von Hand gelötet habe:

![Grafik: Kompletter Schaltplan](/assets/img/18/station_board_complete.png)

Alle benötigten Teile auf engstem Raum richtig zusammen zu löten, hat eigentlich ganz gut geklappt, wenn man bedenkt, dass es meine ersten Löterfahrungen sind. Zugegeben, einzelne Lötstellen berührten nach dem ersten Versuch die Nachbarn und wieder andere hatten einen Wackelkontakt. Aber alle Fehler wurden früher oder später entdeckt und behoben.

Der Anblick von hinten ist, nun ja, doch etwas chaotisch. Ich denke jedem wird hier klar, dass keine professionelle Arbeit geleistet wurde und ich würde dem Schaltkreis jetzt nicht gerade mein Leben anvertrauen.

Nichtsdestotrotz, bin ich eigentlich froh und sogar ein bisschen stolz, dass am Ende alles funktioniert hat.

![Bild: Ansicht hinten](/assets/img/18/station_board_backside.jpg)


In meinem nächsten Beitrag werde ich dann endlich zur Software kommen, die auf dem Chip läuft, um die Daten richtig anzuzeigen. 

