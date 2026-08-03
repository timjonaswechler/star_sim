# Physikalisches Prozessmodell für solare und stellare Flares

Research-Notiz für ein simulationsgeeignetes Flare-Modell. Quellen wurden am 2026-08-03 geprüft. Die Prozesskette stützt sich auf solare Beobachtungen und MHD-/Hydrodynamik-Originalarbeiten; ihre Übertragung auf räumlich meist nicht aufgelöste stellare Flares ist als `PhysicalProxy` zu behandeln, nicht als direkt beobachtete 3D-Geometrie eines beliebigen Sterns.

## Kurzentscheidung

Ein Flare ist **kein einzelner leuchtender Plasmabogen**. Er ist eine vorübergehende Energiefreisetzung in einer magnetisch komplexen aktiven Region:

```text
langsame Einspeisung magnetischer Energie
    -> nichtpotenzielles Feld / Stromschicht
    -> Auslöser und schnelle magnetische Rekonnexion
    -> Energieaufteilung in Heizung, Teilchen und Strömung
    -> Energietransport entlang neu verbundener Feldlinien
    -> helle Footpoints/Ribbons + chromosphärische Verdampfung
    -> heiße, dichte Post-Flare-Schleifen
    -> leitende und radiative Abkühlung
```

Die sichtbaren Bögen sind damit überwiegend **Folgen der Rekonnexion**: heißes Plasma macht einen Teil des Magnetfelds sichtbar. In einem großen Ereignis entstehen nacheinander viele Schleifen beziehungsweise ein Arcade; ein einziger starrer Torus ist kein ausreichendes physikalisches Modell. Direkte EUV- und Röntgenbeobachtungen zeigen einströmende kühlere Schleifen, ausströmende neu verbundene heiße Schleifen und Plasma über `10 MK` am erwarteten Rekonnexionsort ([Su et al. 2013](https://doi.org/10.1038/nphys2675)).

## Begriffe, die getrennt bleiben müssen

| Begriff | Physikalische Rolle | Beziehung zum Flare |
|---|---|---|
| **Korona** | Heiße, dünne äußere Sternatmosphäre und räumliche Umgebung der koronalen Magnetfelder. | Ort eines wesentlichen Teils von Energiespeicherung, Rekonnexion und heißen Flare-Schleifen; kein einzelnes Ereignis. |
| **Aktive Region** | Zeitlich begrenzte Region mit starkem, komplexem Magnetfeld und häufig Stern-/Sonnenflecken. | Stellt Magnetfluss, Footpoints und den freien magnetischen Energievorrat bereit. Flares entstehen bevorzugt dort ([NASA Solar Flare FAQ](https://science.nasa.gov/blogs/solar-cycle-25/2022/06/10/solar-flares-faqs/)). |
| **Flare** | Rasche Freisetzung magnetischer Energie mit Erwärmung, Strahlung, Teilchenbeschleunigung und Plasmabewegung. | Das zu simulierende Ereignis; kann eruptiv oder räumlich eingeschlossen sein. |
| **Flare-Schleife** | Nach der Rekonnexion geschlossene Feldverbindung, die mit heißem und verdichtetem Plasma gefüllt wird. | Sichtbare Folge eines Rekonnexionsschritts, nicht der ursprüngliche Energiespeicher allein. |
| **Footpoints / Flare-Ribbons** | Schnittbereiche neu verbundener Feldlinien mit der dichten unteren Atmosphäre. Viele benachbarte Footpoints ergeben Ribbons. | Dort wird Energie deponiert; die Regionen werden impulsiv hell. Die überstrichene magnetische Ribbon-Flussmenge korreliert stark mit dem Röntgenmaximum ([Kazachenko et al. 2017](https://doi.org/10.3847/1538-4357/aa7ed6)). |
| **Filament / Prominenz** | Relativ kühles, dichtes Plasma, das magnetisch in der heißen Korona getragen wird; vor der Scheibe heißt dieselbe Struktur Filament, am Rand Prominenz ([NASA Heliophysics Vocabulary](https://science.nasa.gov/heliophysics/resources/vocabulary/)). | Kann stabil bleiben, kollabieren oder bei einem eruptiven Ereignis mit aufsteigen. Es ist nicht der Flare selbst. |
| **CME** | Großräumiger Auswurf von magnetisiertem Plasma aus der Korona in den interplanetaren Raum. | Optionaler eruptiver Ausgang. Starke eingeschlossene Flares ohne CME sind beobachtet; umgekehrt können ruhige Filamenteruptionen ohne großen Flare einen CME erzeugen ([Gopalswamy et al. 2009](https://doi.org/10.1017/S174392130902941X), [NOAA CME description](https://www.swpc.noaa.gov/index.php/news/coronal-mass-ejections-cme-space-weather-phenomena)). |

## Prozesskette

### 1. Aktive Region und langsamer Energieaufbau

Magnetischer Fluss tritt aus dem Sterninneren durch die Photosphäre aus und bildet bipolar oder multipolar strukturierte aktive Regionen. Konvektion, differentielle Rotation und lokale photosphärische Bewegungen verschieben, verdrehen und scheren die verankerten Feldbereiche. Dadurch wird Poynting-Fluss in die Atmosphäre eingebracht und ein nichtpotenzielles koronares Feld mit elektrischen Strömen und **freier magnetischer Energie** aufgebaut. Eine MHD-Flussemergenzsimulation reproduziert den Aufbau von Scherung und freier Energie durch Emergenz und Flussauslöschung ([Fang et al. 2012](https://doi.org/10.1088/0004-637X/754/1/15)); HMI-Beobachtungen einer eruptiven aktiven Region zeigen während früher Emergenz eine Zunahme von Strom und freier Energie ([Sun et al. 2012](https://doi.org/10.1088/2041-8205/748/2/L28)).

Für die Simulation bedeutet das: Ein Flare darf erst Energie aus einem langsam gefüllten `MagneticFreeEnergy`-Reservoir entnehmen. Effektive Temperatur und Radius des Sterns bestimmen dieses Reservoir nicht allein. Feldstärke, beteiligter Magnetfluss, aktive Fläche und Grad der Nichtpotenzialität sind eigene Größen.

Wichtig ist die Unsicherheit des Auslösers: Neue Flussemergenz ist weder notwendige noch hinreichende Bedingung für einen starken Flare; in einer Stichprobe von 100 Regionen fehlte bei 11 eine neue Emergenz im untersuchten Vorlauf vollständig ([Kutsenko et al. 2024](https://doi.org/10.1088/1674-4527/ad2e4d)). Ein universeller Schwellenwert „genug Twist = Flare“ wäre daher nicht `Empirical`.

### 2. Instabilität, Stromschicht und Rekonnexion

Wird die Konfiguration instabil oder durch lokale Rekonnexion umgebaut, können stark unterschiedliche Feldrichtungen in einer dünnen Stromschicht zusammengebracht werden. Magnetische Rekonnexion ändert die Feldverknüpfung und wandelt freie magnetische Energie rasch in Wärme, beschleunigte Teilchen, Wellen und gerichtete Plasmaausströmungen um. Beobachtete Inflows, heiße Outflows und eine heiße Quelle am Rekonnexionsort stützen diesen Kernmechanismus direkt ([Su et al. 2013](https://doi.org/10.1038/nphys2675)).

Der klassische zweidimensionale Bogen ist nur ein Schnitt durch den Prozess. Dreidimensionale MHD-Modelle erzeugen räumlich ausgedehnte Rekonnexion, scherende Schleifen und J-förmige Ribbons; die Feldverbindungen können scheinbar entlang der Ribbons gleiten ([Aulanier, Janvier & Schmieder 2012](https://doi.org/10.1051/0004-6361/201219311)). Ein beobachtetes eruptives Ereignis entwickelte sich anfangs deutlich dreidimensional und später arcade-artiger ([Li et al. 2017](https://doi.org/10.3847/1538-4357/835/2/190)).

Für das Prozessmodell ist `ReconnectionEpisode` deshalb eine Folge diskreter oder überlappender Rekonnexionsschritte mit jeweils beteiligtem Magnetfluss, Rate und Energie — keine einmalige Umschaltung eines kompletten Bogens.

### 3. Energieaufteilung und Teilchenbeschleunigung

Die frei werdende Energie wird nicht vollständig zu sichtbarem Licht. Sie geht unter anderem in:

- direkte Plasmaheizung und Wärmeleitung,
- nichtthermische Elektronen und Ionen,
- Rekonnexions-Outflows und andere Plasmabewegung,
- Strahlung über Radio, optisch/UV, EUV sowie weiche und harte Röntgenstrahlung,
- bei einem eruptiven Ereignis in kinetische und potenzielle CME-Energie.

Für 38 große eruptive Sonnenereignisse war die verfügbare freie magnetische Energie ausreichend für CME, beschleunigte Teilchen und heißes Plasma; die Energie beschleunigter Teilchen war groß genug, die bolometrische Flare-Strahlung zu speisen. Die gemessenen Anteile streuen jedoch stark und sind keine universellen Konstanten ([Emslie et al. 2012](https://doi.org/10.1088/0004-637X/759/1/71)).

Beschleunigte Elektronen bewegen sich bevorzugt entlang des Magnetfelds in Richtung dichter unterer Atmosphäre. Dort verlieren sie Energie durch Stöße und erzeugen unter anderem harte Röntgen-Bremsstrahlung an den Footpoints. Direkte Beobachtungen zeigen nichtthermische Footpoint-Quellen zusammen mit den folgenden Aufströmen ([Antonucci, Marocchi & Simnett 1984](https://doi.org/10.1016/0273-1177(84)90168-6)). Teilchenpakete sollten im Modell daher an konkrete neu verbundene Feldpfade und deren beide unteren Enden gekoppelt sein.

### 4. Footpoints, Ribbons und chromosphärische Verdampfung

Energieteilchen und/oder Wärmeleitung deponieren Energie in der dichten Chromosphäre. Sie wird rasch erhitzt und strahlt stark. Steigt Druck und Temperatur genügend an, expandiert Material entlang der Feldlinie in die Korona: **chromosphärische Verdampfung**. Gleichzeitig kann kühleres Material als chromosphärische Kondensation nach unten gedrückt werden.

Hydrodynamische Simulationen unterscheiden eine sanfte und eine explosive Verdampfung abhängig von Energiefluss und Kühlvermögen der Chromosphäre ([Fisher, Canfield & McClymont 1985](https://doi.org/10.1086/162902)). IRIS-Beobachtungen fanden am Footpoint vollständig blauverschobenes heißes Fe XXI bei ungefähr `260 km/s` zusammen mit Rotverschiebungen kühler Linien, konsistent mit heißem Aufstrom und kühlerem Abstrom ([Tian et al. 2014](https://doi.org/10.1088/2041-8205/797/2/L14)). Diese Einzelgeschwindigkeit ist eine Ereignismessung, kein Standardwert für alle Sterne.

Ribbons sind die zeitliche Spur vieler solcher Footpoints. Wenn fortschreitende Rekonnexion neue Flussflächen erfasst, wandern beziehungsweise erweitern sich die Ribbons und darüber erscheinen nacheinander neue Schleifen. Die magnetische Flussmenge innerhalb der Ribbons ist deshalb die geeignetere Kopplungsgröße als eine frei gewählte Bogenanzahl ([Kazachenko et al. 2017](https://doi.org/10.3847/1538-4357/aa7ed6)).

### 5. Heiße Schleifen, Arcade und Abklingphase

Verdampftes Material erhöht Dichte und Temperatur der neu verbundenen koronalen Schleife. Ein Ereignis ist typischerweise multi-threaded: Während ältere Schleifen bereits abkühlen, werden benachbarte oder höher liegende Flussflächen erst neu verbunden und geheizt. Ein Modell aus zeitversetzt gebildeten und abkühlenden Schleifen reproduziert beobachtete Lichtkurven besser als eine Einzelschleife ([Warren, Winebarger & Hamilton 2002](https://doi.org/10.1016/S0964-2749(02)80066-X)).

Nach sinkender Heizrate verliert das Plasma Energie zunächst durch Wärmeleitung entlang des Felds und anschließend zunehmend durch Strahlung; Massenabfluss und mögliche fortgesetzte schwache Heizung können ebenfalls beitragen. Für ein beobachtetes C-Ereignis wurde eine Abkühlung von mindestens `10 MK` auf etwa `0.25 MK` in ungefähr 45 Minuten rekonstruiert, zuerst leitungs- und später strahlungsdominiert ([Raftery et al. 2009](https://doi.org/10.1051/0004-6361:200810437)). Auch diese Zeiten gehören zum jeweiligen Ereignis und müssen in der Simulation skaliert werden.

## Simulationsgeeigneter Zustandsautomat

| Zustand | Gespeicherter physikalischer Zustand | Zulässiger Übergang |
|---|---|---|
| `QuiescentActiveRegion` | Magnetfluss-Topologie, Footpoint-Flächen, Feldstärke, freie Energie | langsame Energieeinspeisung; bei einem modellierten Trigger zu `Precursor` oder direkt `ImpulsiveReconnection` |
| `Precursor` | kleine Rekonnexionsraten, lokale Heizung, mögliche langsame Anhebung einer Flussröhre | kann abklingen oder in schnelle Rekonnexion übergehen; nicht jedes Ereignis benötigt beobachtbaren Vorläufer |
| `ImpulsiveReconnection` | Rekonnexionsrate, pro Schritt neu verbundener Fluss und freigesetzte Energie | erzeugt Outflows, Teilchenpakete, Footpoint-Heizung und neue Schleifen |
| `ChromosphericResponse` | deponierter Energiefluss pro Footpoint, Strahlungsverluste, Auf-/Abstrom | füllt die zugehörige neue Schleife; sanft oder explosiv |
| `HotLoopGrowth` | Temperatur, Dichte und Masse pro Schleifen-Thread; zugleich können neue Threads entstehen | mit sinkender Heizung zu `CoolingArcade` |
| `CoolingArcade` | Wärmeleitung, Strahlung und Drainage pro Thread | zurück zu einer veränderten aktiven Region |
| `EruptiveBranch` | aufsteigende Flussröhre, überwundene oder einschließende Hintergrundfeldspannung | optional parallel zum Flare; `Confined`, `FailedEruption` oder `CmeEjection` |

Die Zustände überlappen räumlich und zeitlich: Ein Thread kann bereits abkühlen, während ein anderer noch impulsiv geheizt wird. Deshalb sollte der Flare als übergeordnetes Ereignis viele `ReconnectedFluxThread`-Instanzen verwalten.

## Mindestgrößen und Erhaltung

Ein späteres quantitatives Modell braucht mindestens:

- zwei oder mehr photosphärische Magnetflussgebiete mit Polarität, Fläche und Feldstärke;
- eine koronale Verbindungstopologie, nicht nur eine sichtbare Kurve;
- `free_magnetic_energy` und `reconnected_flux` als getrennte Größen;
- eine zeitabhängige `reconnection_rate`;
- eine normalisierte Energieaufteilung auf thermisches Plasma, nichtthermische Teilchen, Strömung/Wellen, Strahlung und optional CME;
- für jeden Schleifen-Thread Länge, Querschnitt, Entstehungszeit, Temperatur, Dichte/Masse und Heizrate;
- je Footpoint den deponierten Energiefluss sowie Verdampfungs- und Kondensationsantwort;
- eine eruptive Entscheidung, die vom einschließenden Hintergrundfeld abhängen darf und nicht aus der Flare-Energie allein folgt.

Folgende Invarianten verhindern optisch plausible, aber physikalisch widersprüchliche Abläufe:

1. Freigesetzte Energie darf die verfügbare freie magnetische Energie nicht überschreiten.
2. Die Summe aller Energiekanäle muss die freigesetzte Energie innerhalb definierter numerischer Verluste ergeben.
3. Jede neue geschlossene Schleife besitzt Footpoints in magnetisch gekoppelten Flussgebieten; Ribbons sind Mengen solcher Footpoints.
4. Heißes Schleifenplasma entsteht erst aus Heizung und Massenzufuhr; der komplette Bogen erscheint nicht voraussetzungslos gleichzeitig.
5. Ein CME ist ein optionaler, eigener Massenauswurf. `Flare != CME` und `Flare != Prominence`.
6. Eine Prominenz kann als kühles Plasma in einer Flussröhre existieren, ohne aktuell zu flaren; ihre Instabilität kann einen eruptiven Flare begleiten, ist aber keine Pflichtkomponente.

## Zeitliche Form ohne falsche Universalität

Für ein generisches Ereignis ist die Reihenfolge belastbarer als feste Dauern:

1. **Aufbau:** lange gegenüber dem Flare; freie Energie nimmt zu.
2. **Vorläufer, optional:** lokale schwache Rekonnexion oder langsame Anhebung.
3. **Impulsive Phase:** Rekonnexionsrate, Teilchenbeschleunigung und Footpoint-Emission steigen rasch.
4. **Thermische Aufbauphase:** Verdampfung erhöht Masse und Emissionsmaß heißer Schleifen; sie kann die impulsive Phase überlappen.
5. **Gradual-/Abklingphase:** Rekonnexion und Heizung nehmen ab; ein multi-thermales Arcade kühlt und drainiert.
6. **Erholung:** Die aktive Region besitzt weniger freie Energie und eine veränderte Topologie, kann aber erneut Energie speichern und weitere Flares erzeugen.

Minuten bis Stunden sind für solare Flares beobachtet ([NASA Solar Flare FAQ](https://science.nasa.gov/blogs/solar-cycle-25/2022/06/10/solar-flares-faqs/)); das ist keine universelle Zeitspanne für andere Sterne. Stellare Photometrie zeigt zwar eine breite Energie- und Zeitverteilung, löst aber die zugrunde liegenden Bögen normalerweise nicht räumlich auf. Kepler-Beobachtungen von Superflares auf sonnenähnlichen Sternen und deren Zusammenhang mit Starspots stützen magnetische Energiespeicherung als stellaren `PhysicalProxy` ([Maehara et al. 2012](https://doi.org/10.1038/nature11063), [Notsu et al. 2013](https://doi.org/10.1088/0004-637X/771/2/127)). Sie rechtfertigen nicht, solare Schleifengröße, Teilchengeschwindigkeit oder Energieanteile unverändert auf Roten Zwerg und Roten Riesen zu kopieren.

## Evidenzgrenzen für die Implementierung

- **`Empirical` im solaren Quellbereich:** Auftreten von heißen Rekonnexions-Outflows, Footpoint-/Ribbon-Struktur, chromosphärischer Verdampfung, multi-threaded Schleifen und getrennten Flare-/CME-Ausgängen.
- **`PhysicalProxy` für andere magnetisch aktive Sterne:** dieselbe kausale MHD-Kette aus magnetischer Speicherung, Rekonnexion, Energietransport und Atmosphärenantwort.
- **Nicht belegt als universell:** eine feste Bogenform, exakt zwei sichtbare Schleifen, feste Lebensdauer, feste Partikelgeschwindigkeit, konstante Energieaufteilung oder deterministische CME-Kopplung.
- **`Decorative`, bis ein eigenes Modell vorliegt:** zufällige Plasmafetzen, Spiralbewegungen oder zusätzliche Lichtbänder, die weder aus Magnettopologie noch aus einem Energie-/Massentransportzustand folgen.

Damit ist der geeignete nächste Modellierungsschritt nicht „Partikel auf einen Bogen setzen“, sondern zuerst eine aktive Region mit magnetischem Energiereservoir, Footpoint-Paaren und einer Folge neu verbundener Fluss-Threads zu erzeugen. Strahlung und sichtbare Materie können anschließend aus den Zuständen dieser Threads abgeleitet werden.
