---
layout: post
title: "Mein erstes Master-Semester an der ETH Zürich"
author: "Jakob Meier"
categories: meta
tags: [study, ETHZ]
image: /19/eth/mugs.jpg
image_tooltip: Das Kaffeetassenregal im Informatik Departement.
lang: de
ref: msc-start
techs: 
  ant:
    title: Apache Ant
    description: Wird für das Bauen von Java Applikationen verwendet. Buildfiles werden im XML Format erstellt und es gibt Integrationen für alle grösseren IDEs. Für den oben beschriebenen Kurs ASL musste die finale Abgabe korrekt mit dem Ant Default Task kompilieren.
    url: http://ant.apache.org/
    img: ant.svg
  azure:
    title: Microsoft Azure
    description: Microsoft stellte ihre Cloud Computing Services gratis zu Verfügung (bis zu einem gewissen Limit) für das ASL. Wir nutzten diese Plattform, um die Experimente auf praxisnaher Infrastruktur zu testen.
    img: azure.png
    url: https://azure.microsoft.com/
  bash:
    title: GNU BASH shell scripts
    description: BASH ist die Standard-Shell auf den meisten UNIX-basierten Operating Systems. Mit BASH Skripten kann man sehr direkt eine Anzahl von sequentiellen Shellbefehlen aneinanderreihen und somit vieles automatisieren. Man kann dabei auch Schleifen und if-Bedingungen verwenden.
    img: bash.png
  bgl:
    title: Boost Graph Library (bgl)
    description: Die Boost Bibliotheken gehören zwar nicht zum Standard von C++, sind aber sehr nahe dran. Sie sind alle Open-Source und werden meistens sehr gut gewartet und weiterentwickelt. BGL ist ein Teil dieser Sammlung und wir haben es im Algorithms Lab verwendet. BGL beinhaltet viele nützliche Graphalgorithmen wie beispielsweise Dijkstras kürzeste Pfade.
    img: boost.png
    url: https://www.boost.org/doc/libs/1_69_0/libs/graph/doc/
  cgal:
    title: The Computational Geometry Algorithms Library (CGAL)
    description: Eine C++ Bibliothek die wir für exakte Berechnungen, geometrische Berechnungen und effizientes Berechnen von Delaunay Triangulationen verwendet haben.
    img: cgal.png
    url: https://www.cgal.org/
  cpp:
    title: C++11  und die Standard Template Library (STL)
    description: C++ ist eine objektorientierte, low-level Programmiersprache und C++11 ist der ISO zertifizierte Standard  of von 2011. Die STL besteht aus einer Anzahl von Template Klassen die häufig verwendete Algorithmen und Datenstrukturen implementieren.
    img: cpp.png
  make:
    title: GNU make
    description: Ein Tool zur Buildautomatisierung. Es ist besonders geeignet, um Dateien die von anderen Dateien abhängig sind, automatisch zu (re-)generieren. Oft wird es in C/C++ oder LaTeX Projekten verwendet.
    img: gnu.png
    url: https://www.gnu.org/software/make/
  gdb:
    title: The GNU Project Debugger (GDB)
    description: Mit GDB kann man laufende Programme unterbrechen und Schritt für Schritt den Zustand beobachten. Dies wird zum einen für allgemeines Debuggen, aber zum anderen auch für Reverse-Engineering verwendet.
    url: https://www.gnu.org/software/gdb/
    img: archer.svg
  gnuplot:
    title: gnuplot
    description: Ein gratis Tool um Daten zu visualisieren. In erster Linie wird es aus der Konsole verwendet, es gibt aber auch graphische Benutzeroberflächen. Gnuplot ist vielseitig in Funktionalitäten und kompatibel mit allen wichtigen Betriebssystemen. 
    img: gnuplot.png
    url: http://www.gnuplot.info/
  wifi:
    title: IEEE 802.11
    description: "Die Menge von Protokollen die man umgangssprachlich als  WLAN oder Wi-Fi  bezeichnet, wurden unter diesem gemeinsamen Familiennamen standardisiert. Die einzelnen Protokolle hängen dann noch ein bis zwei Buchstaben an das Ende des Namens. (Beispiel: 802.11n)"
    img: wifi.png
    url: https://en.wikipedia.org/wiki/IEEE_802.11
  intelsgx:
    title: Intel SGX
    description: Eine Erweiterung der Prozessorarchitektur von Intel die es ermöglicht sogenannte vertrauenswürdige Enklaven zu erstellen. Was in diesen Enklaven ausgeführt wird, kann von einem externen Server verifiziert werden, selbst wenn das Betriebssystem nicht vertrauenswürdig ist. Einzig dem Hauptprozessor (CPU) muss vertraut werden.
    img: intelsgx.jpg
    url: https://software.intel.com/en-us/sgx
  java:
    title: Java 8
    description: Java ist eine high-level Programmiersprache die in der Java Virtual Maschine ausgeführt wird. Java 8 hat einige Features neu in die Sprache eingeführt, am wichtigsten wahrscheinlich funktionale Ansätze mit Lambdaausdrücken.
    img: java.png
  memcached:
    title: Memcached
    description: Ein Open-Source Projekt, welches eine Cacheing-Schicht für verteile Key-Value Datenbanken bereitstellt. Das System kann einfach integriert werden und es skaliert sehr gut mit grossen Datenmengen. Facebook und Wikipedia sind zwei bekannte Namen die Memcached verwenden.
    img: memcached.png
    url: https://memcached.org/
  python:
    title: Python 3
    description: Heutzutage eine sehr beliebte Programmiersprache in vielen Anwendungszwecken, wegen der Einfachheit und der sehr ausgereiften Sammlung an verfügbaren Bibliotheken. Für das ASL habe ich Python für das Prozessieren von Daten verwendet, da es nicht kompiliert werden muss und daher sehr einfach aus Bash-Skripten und Make-Regeln aufgerufen werden kann.
    img: python.png
    url: https://www.python.org/
---

<p class="intro">Fünf Monate geprägt von viel Kaffee, kurzen Nächten und jeder Menge an lehrreichen neuen Einblicken. Lies diesen Artikel um herauszufinden, welche Fächerwahl mich das ganze Semester durch 100% motivieren konnte.</p>

Wie ursprünglich geplant, ging ich letzten Sommer wieder zurück an die [ETH Zürich](https://www.ethz.ch), nachdem ich ein volles Jahr als Softwareentwickler gearbeitet hatte. 

Mein [vorheriger Arbeitgeber](https://www.bsi-software.com/) ist mir schon etwas ans Herz gewachsen, dank der freundlichen Mitarbeiter, den spannend täglichen Herausforderungen und den allgemein sehr guten Arbeitsbedingungen die dort herrschen. Trotzdem war ich hoch-motiviert für mein erstes Semester im Masterstudiengang der Informatik.

Ich war so begeistert von den angebotenen Fächern, dass ich es schwierig fand, meine Fächerwahl weit genug einzuschränken. Nach Stunden des Vergleichens und Rücksprache mit Bekannten, die schon Erfahrungen mit den Fächern hatten, bin dann mit 44 ECTS Kreditpunkten in das Semester gestartet, 14 über der allgemeinen Empfehlung.

Ebenfalls in diesem Semester nahm ich auch eine Stelle als Hilfsassistent für das brandneue Fach *Computer Systems* an, welches sich an Informatik Studenten im fünften Bachelorsemester richtet. Wöchentlich bereitete ich dafür die Übungsstunde vor und präsentierte diese dann einer Gruppe von Studenten.

Im Laufe des Semesters musste ich dann Massnahmen ergreifen, damit ich bei keinem der Fächer zurückfalle. Schliesslich wollte ich möglichst viel von dem Lehrangebot profitieren und meine Kapazität ist irgendwo durch dann doch begrenzt. Am Ende hielt ich noch an 33 ECTS Kreditpunkten fest und ich bin heute sehr zufrieden mit meiner Auswahl.

In diesem Artikel will ich nun jedes der darin eingeschlossenen Fächer kurz zusammenfassen. Dem Leser / der Leserin soll dies einen Einblick in das Informatikstudium an der ETHZ verschaffen und vielleicht hilft es auch zukünftigen Studenten bei der Fächerwahl.

###  Liste der belegten Kurse 
{:.no_toc} 
* TOC
{:toc}

![Bild: Das Hauptgebäude der ETH Zürich.](/assets/img/19/eth/ETHZ.JPG)

## Advanced Systems Lab
Zeitmanagement und Selbständigkeit sind beide von hohem Stellenwert für das *ASL*. In keinem der Fächer, welche ich bisher belegt hatte, war dies auch nur annähernd so wichtig. 

In der ersten Woche wurde die Aufgabenstellung des Projektes veröffentlicht zusammen mit dem Abgabedatum, welches in der letzten Semesterwoche lag. Ausserdem gab es einige Einführungspräsentationen in das Thema der Queueing-Theorie während des ersten Drittels des Semesters. Aber das war es dann auch schon von dem was den Studenten direkt mitgegeben wurde. Es war eine klare Botschaft: Die ist ein Kurs für Masterstudenten, also wird ein gewisses Niveau an Professionalität erwartet und die Zeiten des Babysitten sind vorbei.

Das Buch [The Art of Computer Systems Performance Analysis](https://www.amazon.com/Art-Computer-Systems-Performance-Analysis/dp/0471503363) liefert die theoretische Grundlage für das Projekt und das Buch sollte man sicher während dem Semester lesen.

Die praktischen Fähigkeiten sollte man wohl besser bereits weitgehend mitbringen. Man kann zwar durchaus während dem Semester noch alles über Netzwerksockets lernen und Buildtools wie **Apache Ant** von ganz vorne her kennenlernen. Und auch die Kommunikation mit Remote-Servern und wie die Ausführung von Skripten auf denselben funktioniert, das kann man sich alles noch schnell aneignen. Ressourcen dazu im Web gibt es sicher mehr als genug. Aber man ist dabei halt komplett auf sich alleine gestellt und je mehr Zeit man braucht um diese Grundlagen zu lernen, umso weniger Zeit bleibt für das eigentliche Projekt, welches ohnehin überaus umfänglich ist.

Der Auftrag variiert etwas von Jahr zu Jahr. Dieses Mal musste man eine Middleware in **Java 8** programmieren die mit standardisierten **Memcached** Clients und Servern kommunizieren soll. Währenddessen soll die Middleware auch Statistiken über die bearbeiteten Daten ansammeln und auf eine geeignete Weise ausgeben. 

In vielen Bereichen hat man als Student Freiheiten im Design. Hier eine Illustration, die ganz grob aufzeigt, wie die Systemkomponenten in meinem Design zusammen spielen.

![Bild: ASL Designüberblick.](/assets/img/19/eth/asl.png)

Doch die Implementation des Systems ist lediglich etwa 20% des Arbeitsaufwandes. Das eigentliche Projekt besteht aus einer Reihe von Experimenten mit dem System installiert in der **Azure-Cloud**. Und dabei wiederum liegt der Fokus auf der korrekten Auswertung der Resultate. 

Bewertet wurde, was man über das System herausgefunden hat, wie man es begründet hat und wie sauber der wissenschaftliche Prozess dahinter ausgeführt und dokumentiert wurde.

Die durchzuführenden Experimente sind eigentlich ziemlich stark strukturiert vorgegeben. Aber welche Hilfsmittel man einsetzt, um diese Experimente zu automatisieren, die Daten zu sammeln und letztendlich auszuwerten, das ist dann alles den Studenten überlassen. 

Mit der Hilfe von **Bash-Skripten** habe ich die Experimente mit den richtigen Parametern auf allen nötigen Maschinen gestartet. Für die Auswertung habe ich dann eine Kombination aus **Python 3**, **gnuplot** und **Make** verwendet.

Nachfolgend ein Beispiel Graph aus meinem Bericht. Er wurde mit *gnuplot* gezeichnet, aufgrund einer Reihe von *Make-Regeln*, die ich definiert habe um die Rohdaten zusammenzufassen und aufzubereiten. Man muss sich bewusst sein, dass die Daten auf verschiedenen Maschinen erzeugt werden, während mehreren Wiederholungen und mit verschiedenen Einstellungen.

![Bild: ASL Beispielgraph.](/assets/img/19/eth/asl_graph.png)

## Algorithms Lab
Zusammengefasst geht es im Algorithms Lab darum, in C++ Aufgaben zu lösen, die von der Art her in einem Programmierwettbewerb vorkommen könnten.

Jede Woche gib es eine Vorlesung, in der neue Themen und Konzepte vorgestellt werden. Danach werden praktische Programmieraufgaben freigeschaltet, die man online so oft abgeben kann wie man will, mit automatisierter Korrektur. Es gibt Teilpunkte für korrekte, aber nicht optimal effiziente Lösungen. Für die volle Punktzahl muss man jeweils genau auf die Beschreibung der Eingabedaten achten und je nach Grösse der verschiedenen Parameter optimieren. 

Eine typische Aufgabe kombiniert verschiedene Techniken aus *dynamischer Programmierung*, *geometrischen Algorithmen*, *Graphenalgorithmen* und *linearer oder quadratischer Programmierung*. Und um die Probleme richtig zu modellieren und zu vereinfachen wird ein solides Grundwissen in Geometrie, Graphentheorie und kombinatorischen Konzepten vorausgesetzt.

Insgesamt gibt es dann rund 70 praktische Aufgaben, die aber nur ein Bruchteil der Studenten wirklich komplett zu lösen vermag bis zum Prüfungsdatum. Dies wird ersichtlich aus der online Rangliste, die während dem ganzen Semester verfügbar ist. Die Rangliste ist aber nur für einen freundlichen Wettbewerb zwischen den Studenten gedacht und ist nicht relevant für die Notengebung.

Die Prüfung findet dann an zwei verschiedenen Tagen statt, mit jeweils 3 Aufgaben und 6 Stunden Zeit. Auch während der Prüfung kann man die Aufgaben online abgeben und man bekommt sofort eine Bewertung zurück.

Um zu punkten bei einer Aufgabe, muss man meistens schon mit originellen Ideen aufkommen, wie man das Problem vom Text in eine geeignete mathematische Formulierung übersetzen kann. Ich denke von den vielen Studenten die das Fach jedes Jahr nicht bestehen, scheitern die meisten an diesem ersten Schritt, denn selbst die aufwändigste Vorbereitung kann nicht garantieren, dass man die richtige Idee hat an der Prüfung.

Hat man dann die ideale mathematische Formulierung gefunden, dann muss man noch flink beim Implementieren sein, oder ansonsten wird man nicht alle Aufgaben bearbeiten können. Dafür braucht es Erfahrung mit **C++ 11** und den Bibliotheken **Standard Template Library (STL)**, **Boost graph library (BGL)** und  **Computational Geometry Algorithms Library (CGAL)**. Dies kann man sich durch viel Übung unter dem Semester und in der Lernphase aneignen. 

Das Fach gilt als relativ schwierig aber das Lösen der Aufgaben macht den meisten Informatikern Spass. Studenten die das Fach bestanden haben und diejenigen, die es nicht bestanden haben, sind sich dabei eigentlich einig, wenn man eine Aufgabe richtig lösen konnte, ist es immer ein sehr befriedigendes Gefühl.

## System Security
Wahrscheinlich der beste Einführungskurs für angehende Hacker. Aber auch spannend für alle sicherheitsbewussten Informatiker. Ein gutes Vorwissen in Kryptologie und in Systemen ist aber sehr empfohlen.

In diesem Kurs werden einige allgemeine Themen von der theoretischen Seite her betrachtet. Beispielsweise Side-Channel-Attacks, IOT-Sicherheit und Plattform-Sicherheit. Diese Themen sind dann auch an aktuelle, wissenschaftliche Publikationen gebunden welche die Studenten aufgerufen werden zu lesen. 

Für den Side-Channel Angriff der den Energieverbauch eines Prozessors analysiert, gibt es sogar ein praktisches Lab, wo man in Gruppen einen RSA Schlüssel aus einem embedded System auslesen kann. Der benötigte Aufbau des Oszilloskops wird aber von den Assistenten übernommen.

Dann gibt es noch ein paar eher spezifische Technologien und Angriffe die genauer betrachtet werden. Bekannte Beispiele dazu wären die **Intel SGX** Technologie und Angriffe wie [Meltdown](https://meltdownattack.com/) und [Foreshadow](https://foreshadowattack.eu/). Bei diesen Themen wird aber auf praktische Übungen gänzlich verzichtet.

Es gibt aber schon praktische Übungen. Diese decken aber nur die einfachsten der Angriffe ab, die im Unterricht behandelt werden. Eine Aufgabe behandelte zum Beispiel, wie man *Root-Rechte* in einer Linux Distribution bekommt, durch Ausnutzen von Buffer-Overflow Schwächen in einer ausführbaren Datei. Dabei musste man dann auch auf Techniken wie [return-oriented programming](https://de.wikipedia.org/wiki/Return_Oriented_Programming) zurückgreifen. Dabei hilft es, wenn man bereits Übung im Umgang mit **gdb** hat, aber ich denke, das kann man auch problemlos unter dem Semester dazulernen.

## Wireless Networking and Mobile Computing
Dieser Kurs ist eine regelrechte Ein-Mann-Show von [Stefan Mangold](https://www.lovefield.ch/~smangold). Er kann die Themen sehr anschaulich erklären und kennt sich natürlich bestens aus, da er auch schon Jahre auf dem Gebiet forscht und arbeitet. Unter anderem hat er an WLAN Standards mitgewirkt, wie etwa der QoS Erweiterung [IEEE 802.11e](https://www.cs.ccu.edu.tw/~yschen/course/93-1/2.pdf) wo er sogar Hauptautor war.

Die Protokolle unter dem **IEEE 802.11** Standard sind sicher der Kern der Vorlesung, aber es werden auch alle möglichen anderen Arten von kabelloser Kommunikation betrachtet, wie etwa Mobilfunk oder auch Kommunikation über Schall und Licht. Für die Hausaufgaben muss man dann Experimente in den vorgestellten Themen durchführen, entweder durch Simulation oder zum Teil mit richtiger Hardware.

Hier im Bild, ein Beispiel von einer Aufgabe. Mit Python haben wir zu zweit eine kleine Chat-App entwickelt, die dann über serielle Kommunikation mit einem [Arduino](https://www.arduino.cc/) spricht. Dieser nutzt dann eine einzelne LED, um mit einem anderen Arduino die nötigen Chat-Daten auszutauschen.

![Bild: Arduino Setup.](/assets/img/19/eth/led_communication.jpg)

Ungefähr jede zweite Woche muss man dann einen Bericht von den Experimenten abgeben, der benotet wird. Für das Arduino + LED Experiment war dies einer der Graphen die  aus den Daten hervor gingen.

![Graph: Lichtkommunikation Experimentergebnisse für verschiedene Paketgrössen und verschiedene Distanzen.](/assets/img/19/eth/led_communication_graph.png)

## System Construction
Wenn man diesen Kurs besucht, hat man die Chance Systeme wirklich von A bis Z zu verstehen.

In der Vorlesung wird erklärt, welche Schwierigkeiten es zu bewältigen gilt und welche Ansätze es dazu gibt. Danach gibt es direkt anschliessend die Aufgaben, die man unter Aufsicht des Dozenten vor Ort lösen kann und soll. Dabei bekommt man meistens eine grösstenteils funktionierende Code-Basis und muss diese dann erweitern mit den vorhin besprochenen Funktionen. 

Im Gegensatz zu anderen Kursen die Betriebssysteme behandeln, bekommt man hier eigentlich immer den gesamten Quellcode zu sehen und man kann eigentlich auch die Gesamtheit verstehen. (Beispiel: Ein Scheduler für ein System mit mehreren Threads.) Dies ist möglich, weil der Kurs sich mit absichtlich klein und einfach gehaltenen Betriebssystemen befasst, die im Laufe der Jahre um das [Projekt Oberon](http://www.projectoberon.com/) herum gebaut wurden.

Studenten die das Fach wählen, sollten aber auch offen sein was Programmiersprachen angeht. Im Kurs wird hauptsächlich **Oberon** verwendet, was sich syntaktisch etwas unterscheidet von den heute dominanten Sprachen. Aber trotz des hohen Alters der Sprache, hat es einig sehr interessant Features in der Sprache, die ich auch in modernen Sprachen gerne sehen würde. 

## Informal Methods
Wenn ein Unternehmen neue Software braucht, dann wird der geschriebene Code wohl kaum formell verifiziert. Zu beweisen, dass der Code korrekt ist, wäre schlicht und einfach zu aufwendig. Und mit einem Test-Framework kann man oftmals mit sehr viel kleinerem Aufwand, ebenfalls eine hohe Softwarequalität erreichen.

Nun, wenn man aber trotzdem Code verifizieren will, dann greift man wahrscheinlich auf Hilfsmittel, wie [Isabelle/Isar](https://www.ethz.ch/content/dam/ethz/special-interest/infk/inst-infsec/information-security-group-dam/research/publications/pub2007/Isabelle_Isar.pdf) zurück die den rein formellen Teil automatisieren können.

Aber eben, nur der *Formal Methods* Teil kann wirklich automatisiert werden. Es gibt auch noch informelle Schritte die man nicht schematisch lösen und daher nicht automatisieren kann. Ein gutes Beispiel dafür ist das Finden einer Loop-Invariante, die sich selbst erhalten kann und die ausreicht, um gewünschtes Verhalten zu beweisen. 

In diesem Kurs geht es genau um diese informellen Schritte. Man bekommt im Laufe des Semesters ein besseres Gefühl dafür. 

Die dabei erlernten Techniken sind aber nicht nur nützlich für das Beweisen der Korrektheit einer Software, sondern eben auch, wenn man ohne formelle Methoden will gute Qualität an Code schreiben. Während dem Kurs muss man auch nie Tools für formelle Methoden verwenden. Alle Aufgaben basieren auf Annotationen in der Form von Kommentaren zwischen den einzelnen Zeilen im Quellcode.

Meiner Meinung nach trainiert man hier Fähigkeiten die für alle professionellen Programmierer nützlich sind. Man bekommt eine etwas andere Sichtweise auf Programmcode und man kann damit Fehler erkennen und verhindern ohne das Programm jemals auszuführen.

#  Abschliessende Worte
{:.no_toc} 
In den obigen Beschreibungen gab ich mir Mühe, objektiv zu bleiben. Allerdings will ich doch noch anmerken, dass ich sämtliche genannten Fächer persönlich sehr genossen habe und ich will mich bei den Organisatoren für ihre Arbeite bedanken.

Als letztes will ich noch den Arbeitsaufwand ansprechen. Ich muss zugeben, es war nicht immer einfach in allen Kursen mit vollem Einsatz dabei zu bleiben. Besonders da die beiden Labs doch *sehr* aufwendig sind und es bei den anderen Fächern dann ständig Abgabetermine gab, hatte ich wenig Freiraum unter dem Semester zum Atmen. Aber mit genügend Kaffee und dank ausreichend Motivation und Ausdauer, war es am Schluss möglich für mich, alle Fächer abzuschliessen ohne grössere Kompromisse eingehen zu müssen.
