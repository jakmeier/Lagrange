---
layout: post
title: "Eine persönliche Anzeigetafel für Tramabfahrtszeiten - Software Teil"
author: "Jakob Meier"
categories: IOT
tags: [iot,esp8266,esp-01s,c,low-level]
image: /18/church_ottobeuren.jpg
image_tooltip: Früher hat man die Zeit noch an der Kirchenuhr nachgeschaut.
lang: de
ref: tram-station-board-software
techs: 
  esp8266:
    title: ESP8266
    description: Die Basis von diesem Projekt ist dieser Internet-der-Dinge Chip, entworfen von Espressif. Klein, günstig und nicht schlecht dokumentiert. Ziemlich cool für Hobbyprojekte.
    url: https://www.espressif.com/en/products/hardware/esp8266ex/overview
    img: esp8266.jpg
  c:
    title: "C"
    description: "Wenn man so nahe am Metall arbeitet, ist C sicher die erste offensichtliche Wahl als Programmiersprache. Insgesamt finde ich C immer noch eine schöne Sprache mit der tiefen sprachlichen Komplexität und den mächtigen Möglichkeiten direkt im Speicher herumzuwühlen."
    url: https://en.wikipedia.org/wiki/C_(programming_language)
    img: c.png
  tcp_ip:
    title: "TCP / IP"
    description: "Ein grosser Teil des heutigen Internets baut auf TCP auf und IP steht immerhin für Internet-Protokoll, womit es eigentlich definiert was wir heute Internet nennen. "
    url: https://en.wikipedia.org/wiki/Internet_protocol_suite
    img: tcpip.png
  http:
    title: "HTTP"
    description: "Der de facto Standard für Schnittstellen auf der Applikationsebene im OSI Modell. Das wichtigste Beispiel ist sicherlich das World-Wide-Web."
    url: https://en.wikipedia.org/wiki/Hypertext_Transfer_Protocol
    img: HTTP_logo.png
  transport_api:
    title: "Transport API von OpenData.ch"
    description: "Diese API liefert die aktuellen Daten des öffentlichen Verkehrs. Sie ist aus meiner Sicht ganz gut dokumentiert und auch sehr flexibel aufgebaut."
    url: https://transport.opendata.ch/
    img: OpenData.ch-Logo.png
  ntp:
    title: "NTP"
    description: "Das Netzwerk-Zeit-Protokoll gibt es schon seit ungefähr 1985 und es hat auch heute noch einen hohen Stellenwert, denn so gut wie jede Plattform unterstützt den alten Standard noch immer.Im OSI Modell gehört es auch zur Applikationsebene."
    url: https://en.wikipedia.org/wiki/Network_Time_Protocol
    img: ntp.jpg
---

<p class="intro">Wie man eine Wanduhr mit einer LED-Matrix ersetzen kann. Ein kleiner Einblick in die Programmierung eines Mikrocontrollers.</p>


## Überblick

Das Ziel dieses Hobbyprojektes ist das Bauen einer Anzeigetafel, die angibt, wann das nächste Tram in die Stadt fährt.


[Im letzten Beitrag]({% include link-by-ref.html ref="tram-station-board-hardware" %}) habe ich gezeigt, wie ich die Elektronik dafür zusammen gestellt habe. Die kleine Platine besteht hauptsächlich aus einer 8-mal-8 LED-Matrix und dem ESP8266 als Prozessor. 

Hier geht es nun darum, die Software dafür zu beschreiben. Die Installation derselben wurde in einem [älteren Beitrag]({% include link-by-ref.html ref="esp8266-intro" %}) erläutert.

## Architektur

Auf dem Chip gibt es einen Speicherbereich, der nur gelesen werden kann. Darauf sind gewisse Funktionen bereits vorhanden, zum Beispiel die Berechnung gängiger Funktionen wie etwa MD5 Hash-Funktionen, oder eine komplette Implementation des 802.11 Protokolls. Die komplette Liste kann aus [dieser Datei](https://github.com/espressif/ESP8266_NONOS_SDK/blob/master/ld/eagle.rom.addr.v6.ld) ausgelesen werden.

Des Weiteren gibt es noch Flash-Speicher. Hier kommt die eigene Firmware, die später ausgeführt werden soll darauf.

Der Quellcode für die Firmware teilt sich wiederum in zwei Teile. Einmal das [SDK](https://github.com/espressif/ESP8266_NONOS_SDK) und dann der *User-Code*, also der Teil, der von mir geschrieben wurde.

## Die LED-Matrix ansteuern

Zuerst schauen wir an, wie die LED-Matrix kontrolliert wird. Auf der Hardware-Seite hat sie 24 Pins, die indirekt über Shift-Register angeschlossen sind und alle irgendwie kontrolliert werden müssen. Der detailliert Aufbau ist in [diesem Beitrag]({% include link-by-ref.html ref="tram-station-board-hardware" %}) beschrieben.

### Eine Schnittstelle als Abstraktion

Von der Software aus betrachtet, interessieren die 24 Pins eher weniger. Interessant sind viel mehr die 64 LEDs die in zwei Farben ein- und ausgeschaltet werden können. Um den Status der gesamten Matrix zu definieren, sind also zwei 64-Bit Werte geeignet, einer für jede Farbe.

Mit dieser Überlegung habe ich folgende Schnittstelle definiert, um eine einfache Anzeige zu ermöglichen. (Teil von [led_matrix.h](https://github.com/jakmeier/esp8266/blob/master/led_matrix/tram_station/led_matrix.h).)

```c
void display_full_matrix(long long yellow, long long red);
```

Die ganzen Details, wie die 24 Pins in den drei Shift-Registern richtig gesetzt werden, damit das gewünschte Bild angezeigt wird, versteckt sich dann in der Datei [led_matrix.c](https://github.com/jakmeier/esp8266/blob/master/led_matrix/tram_station/led_matrix.c).

### Zusammenspiel mit dem Hardwaredesign

Als ich die Verlötung erklärt habe im [letzten Beitrag]({% include link-by-ref.html ref="tram-station-board-hardware" %}), erwähnte ich auch, dass es noch eine Herausforderung gibt. Nämlich teilen sich alle LEDs in einer Reihe die Stromstärke. Das bedeutet, dass die Intensität des Lichts davon abhängt, wie viele Lämpchen gleichzeitig an sind. 

Um das zu umgehen, kann man einfach immer nur eine LED auf einmal einschalten. Will man damit dann ein Bild erzeugen mit mehreren Punkten, dann wird jede LED einfach nur \\( {1 \over 64} \\) der Zeit an sein. Mit der richtigen Geschwindigkeit bemerkt ein menschliches Auge gar nichts davon. Allerdings gibt es flackernde Effekte bei der Videoaufnahme mit einer Kamera.

Hier ein Bild, welches beim Debugging entstanden ist. Es zeigt in Rot den Anfang einer HTTP Antwort an, wobei jede Reihe ein Buchstabe (ein Byte) ist. Es soll zeigen, dass jede erdenkliche Kombination von LEDs möglich ist.

![Bild: LEDs in willkürlicher Formation](/assets/img/18/led_combination.jpg)

## Scheduling

Scheduling ist eines der Dinge die normalerweise von einem Betriebssystem (OS) übernommen werden. Scheduling nennen wir die Organisation davon, in welcher Reihenfolge verschiedene Aufgaben ausgeführt werden. Im einfachsten Fall, wenn ein Prozess fertig ist, entscheidet das OS welcher Prozess als Nächstes kommt. Oder, wenn ein Prozess gerade auf etwas wartet, kann das OS in der Zwischenzeit einen anderen Auftrag auf dem Prozessor ausführen lassen. Darüber hinaus kann ein OS auch laufende Prozesse unterbrechen und pausieren, falls wichtigere Aufträge ausführbar sind.

Auf kleinen Systemen wie dem hier vorgestellten, gibt es meisten kein OS, oder dann halt nur ein minimales. Für den ESP8266 gibt es zwar sogar eine [RTOS Implementation](https://github.com/espressif/ESP8266_RTOS_SDK) die auf dem [FreeRTOS Kernel](https://www.freertos.org/) basiert, worin unter anderem auch Scheduling Funktionen bestehen. Allerdings habe ich für dieses Projekt beschlossen, dass es nicht wirklich ein ganzes OS braucht und deshalb habe ich nur das erwähnte SDK gebraucht, ohne OS.

Ohne Betriebssystem gibt es also kein Scheduling. Somit ist jeweils der gesamte Chip blockiert, während der User-Code läuft. Das heisst, keine Netzwerkoperationen werden in der Zeit ausgeführt. Daher wird in der [SDK Referenz](https://www.espressif.com/sites/default/files/2C-ESP8266_Non_OS_SDK_API_Reference__EN.pdf) erwähnt, dass man nie länger als 15 ms im User Code bleiben soll.

Aber wie kann man dann den User Code richtig ausführen? Dies wird ermöglicht mit *Timern* und *Warteschlangen*. Beides sind Funktionen, die direkt auf dem Chip verfügbar sind und das SDK erlaubt dann auch noch die bequeme Anbindung durch die darin definierten Funktionssignaturen.

Ein *Timer* ruft im regelmässigen Abstand eine definierte User-Code Funktion auf. 

Eine *Warteschlange* ruft eine User-Code Funktion einmalig auf, wenn es das nächste Mal gerade gut passt. Je nachdem werden zuerst noch Netzwerkoperationen wie etwa TCP-Keep-Alive Nachrichten ausgeführt.

Hier ein Auszug aus dem Quellcode, wie der User-Code mit dem SDK zusammen funktioniert.

```c
void user_init(){

  // configure GPIOS
  ...

  // setup timer to run user code twice a second
  os_timer_setfn(&some_timer, (os_timer_func_t *)user_loop, NULL);
  os_timer_arm(&some_timer, 500, 1);

  // setup Matrix display loop
  system_os_task(display_matrix, MATRIX_DISPLAY_TASK_PRIO, MATRIX_DISPLAY_LOOP_QUEUE, MATRIX_DISPLAY_LOOP_QUEUE_LEN);
  system_os_post(MATRIX_DISPLAY_TASK_PRIO, 0, 0);

  init_wifi_station();
  init_clock();
}

void user_loop(void *arg) {
  // User code that must be called frequently goes here
}

void display_matrix(os_event_t *e) {
  // turn on and of the LED of the matrix 
  // the image to display is defined in a global variable
}

void init_wifi_station() {
  // Initialize the 802.11 connection procedure to the local router
  // register wifi_event_handler
}
void init_clock() {
  // Initialize the NTP connection
}

void wifi_event_handler(System_Event_t *evt) {
  // Handle different events (connected, disconnected, got IP, ...)
}
```
Die ganze Datei ist [hier](https://github.com/jakmeier/esp8266/blob/master/led_matrix/tram_station/main.c) verfügbar.
 
## Aktuelle Daten aus dem Internet beziehen

### Verbindungsaufbau

Um Daten von [transport.opendata.ch](transport.opendata.ch) zu erhalten, müssen mehrere Schritte durchlaufen werden. Hier ein grober Überblick über diese:

 1. Bekomme eine IP-Adresse und baue eine Internetverbindung auf via Wi-Fi und einem Router
 2. Führe eine DNS Anfrage aus für *transport.opendata.ch*
 3. Baue eine TCP-Verbindung auf
 4. Stelle eine HTTP Anfrage zusammen und schicke sie ab
 5. Lies die HTTP Antwort

Die Zeit zwischen den Schritten ist stark vom Netzwerk abhängig und kann unter Umständen lange dauern. Da User-Code aber nicht beliebig lange laufen kann, muss jeder Schritt einzeln angestossen werden und ein sogenannter *Callback* registriert werden. Der Callback ist eine Funktion, die ausgeführt wird, wenn der angestossene Auftrag fertig ist. Von da aus kann dann der nächste Schritt angestossen werden, ohne dass dazwischen der gesamte Chip blockiert ist.

### Die User-Schleife

Im Codeausschnitt oben werden die zwei Funktionen `user_loop` und `display_matrix` definiert und so registriert, dass sie wiederholt immer wieder aufgerufen werden. In der erst genannten Funktion wird entschieden, wann es Zeit ist die Daten neu vom Server abzufragen. In der anderen wird immer wieder berechnet, wie das auf der LED-Matrix angezeigte Bild gerade aussehen soll.

Damit die beiden Funktionen ihren Auftrag erfüllen können, müssen sie einiges über den aktuellen Zustand des Systems wissen, zum Beispiel in welchem Zustand sich die Verbindung zum Server gerade befindet.

Hier eine Grafik die alle möglichen Zustände sowie die Übergangsmöglichkeiten zusammenfasst.

![Bild: Die Finte-State-Machine](/assets/img/18/sbb_FSM.png)

In den Zuständen die grün hinterlegt sind, wird auf der LED-Matrix die Zeit bis zur Abfahrt des nächsten Trams angezeigt. Die meiste Zeit wird sich das System in einem dieser drei Zustände befinden.

Im Zustand **TCP Connected** besteht eine Verbindung zu *transport.opendata.ch* und es werden gerade Daten empfangen. Sobald alle Informationen angekommen sind, geht das System in den Zustand **TCP Disconnected** bis es wieder Zeit ist, die Daten zu aktualisieren.

Über die Nacht, wenn kein öffentlicher Verkehr fährt, geht das System in den Schlafmodus, also den Zustand **Sleeping**. Dann werden nur noch sehr wenige Anfragen an den Server geschickt und auf der LED-Matrix sieht man ein langsam blinkendes Muster.

Sollte etwas schiefgehen, geht das System in den **Error** Zustand. Dann wird ein rotes Muster mit Fehlerinformationen angezeigt auf der LED-Matrix. Normalerweise kann etwas später aber wieder erfolgreich eine Verbindung aufgebaut werden.

Damit man immer schön sieht, was das System gerade macht, wird in den anderen Zuständen ein eindeutiges Muster angezeigt. Im Video eine kleine Demonstration wie es aussieht, wenn das System startet und initial eine Verbindung aufbaut.

<video width="640" height="320" controls>
  <source src="/assets/vid/18/sbb_boot_up.mp4" type="video/mp4">
  Dein Browser unterstützt keine HTML5 Videos.
</video> 

Der erste Zustand **Init** ist zu kurz um etwas zu sehen. Danach sieht man aber das Muster vom **Wifi Connected** Zustand für etwa 5 Sekunden.

## Zeit

Den Umgang mit Zeit richtig zu programmieren, kann erstaunlich schwierig sein, wenn man sich nicht weitgehend auf eine Standardimplementation verlassen kann. Die kombinierte Komplexität von Zeitzonen, Sommerzeit und Schaltjahren kann durchaus zu fehlerhafter Software führen.

Glücklicherweise beinhaltet der ESP8266 bereits eine Implementation von NTP. Damit braucht es lediglich eine kleine Konfiguration vom Programmierer. Der Chip sorgt dann dafür, dass über das SDK immer eine synchronisierte Zeit verfügbar ist.

Das Einlesen der Ankunftszeit des nächsten Trams ist dann aber nochmals eine andere Geschichte. Das Zeitformat im HTTP Body entspricht zwar dem [ISO 8601 Standard](https://en.wikipedia.org/wiki/ISO_8601), aber leider hat es keine Hilfsfunktionen dafür auf dem Chip. Stattdessen musste ich die textuelle Repräsentation des Datums in Zahlen umwandeln und letztendlich daraus einen [UNIX-Zeitstempel](https://www.unixtimestamp.com/) berechnen. Das war zwar etwas mühsam, aber unterm Strich ist es auch keine Hexerei.

Hat man nun einen Zeitstempel, kann man den einfach mit dem aktuellen Zeitstempel aus dem NTP vergleichen und damit ausrechnen, wie viele LEDs gerade leuchten sollen.

## Der Quellcode

Das ist auch schon alles. Die Gesamtheit des Quellcodes ist natürlich auf meinem [GitHub](https://github.com/jakmeier/esp8266/blob/master/led_matrix/tram_station/) einsehbar. Ich muss den Leser aber noch warnen, dass es sich hier nur um ein kurzes Hobbyprojekt gehandelt hat und der Quellcode entsprechend bei Weitem nicht der Qualität entspricht die man bei der Arbeit oder bei einer Teamarbeit einchecken würde.
