# Apollo-11
[![NASA][1]][2]

*Available in: [English][EN], [简体中文][ZH_CN], [正體中文][ZH_TW], [Português][PT_BR], [Español][ES]*

Código fuente original del ordenador de abordo del Apollo 11 (AGC)
para el módulo de órdenes (Comanche055) y el módulo lunar (Luminary099).
Digitalizado por los amigos de [Virtual AGC][3] y [el museo del MIT][4].
El objetivo es ser un repositorio del código fuente original del Apollo 11.
De este modo, las PRs serán bienvenidas para cualquier cuestión que se
identifique entre las transcripciones en este repositorio y los escaneos
del código fuente original para [Luminary 099][5] y 
[Comanche 055][6], así como cualquier fichero que se haya olvidado.

## Cómo contribuir
Por favor, lea [CONTRIBUTING.md][7] antes de abrir una pull request.

## Compilar
Si está interesado en compilar el código fuente original, eche un vistazo a
[Virtual AGC][8].

## Attribuciones
```plain
Derechos de autor: Dominio público.
Nombre del archivo:  CONTRACT_AND_APPROVALS.agc
Propósito:   Parte del código fuente es para el Colossus 2A, también conocido
        como Comanche 055. Es parte del código fuente del módulo de órdenes
        (CM) Ordenador de abordo (AGC) para el Apollo 11.
Ensamblador: yaYUL
Contacto:   Ron Burkey <info@sandroid.org>.
Sitio Web:   www.ibiblio.org/apollo.
Histórico de modificaciones:   2009-05-06 RSB  Transcrito a partir de imágenes de las páginas 

Este código fuente se ha transcrito o adaptado de imágenes digitalizadas
de una copia en papel presente en el museo del MIT. Los empleados del museo
Paul Fjeld y Deborah Douglas relizaron la digitalización y la gestión de las
imágenes respectivamente. Muchas gracias a ambos. Las imágenes (con la
consecuente reducción de su tamaño para almacenamiento y de su calidad) están
disponibles en línea en la dirección www.ibiblio.org/apollo. Si por alguna razón
considera que algunas imágenes son ilegibles, contacte comigo en info@sandroid.org
para obtener acceso a la imágenes de (más) alta calidad las cuales Paul creó en su
momento.

Algunas notas en la copia en papel del documento muestran la siguiente información:

Revisión de ensamblado 055 del programa Comanche de AGC por NASA
2021113-051.  10:28 APR. 1, 1969

Página 1

#************************************************************************
#                                                                       *
#       ESTE PROGRMA DEBE TAMBIÉN LLAMARSE:                             *
#                                                                       *
#                                                                       *
#               COLOSSUS 2A                                             *
#                                                                       *
#                                                                       *
#   ESTE PROGRMA SE CREÓ PARA UTILIZARSE EN EL CM TAL Y COMO            *
#   SE ESPECIFICA EN EL INFORME R-577.  ESTE PROGRMA SE PREPARÓ         *
#   EN EL PROYECTO DSR 55-23870, ESPONSORIZADO POR EL CENTRO DE         *
#   NAVES TRIPULADAS DEL CENTRO NACIONAL DE AERONAÚTICA Y               *
#   ADMINISTRACIÓN ESPACIAL A TRAVÉS DEL CONTACTO NAS 9-4065            *
#   DEL LABORATORIO DE INSTRUMENTACIÓN DEL INSTITUTO DE TECNOLOGÍA      *
#   DE MASSACHUSSETS, CAMBRIDGE, MASS.                                  *
#                                                                       *
#************************************************************************
ENVIADO:  MARGARET H. HAMILTON        FECHA:   28 MAR 69
    M.H.HAMILTON, RESPONSABLE DE PROGRAMACIÓN DE COLOSSUS
    GUÍA Y NAVEGACIÓN DE APOLLO

APROBADO:   DANIEL J. LICKLY        FECHA:   28 MAR 69
    D.J.LICKLY, DIRECTOR DE DESARROLLO DEL PROGRAMA DE LA MISIÓN
    PROGRAMA DE GUÍA Y NAVEGACIÓN DE APOLLO

APROBADO:   FRED H. MARTIN          FECHA:   28 MAR 69
    FRED H. MARTIN, JEFE DEL PROYECTO COLOSSUS
    PROGRAMA DE GUÍA Y NAVEGACIÓN DE APOLLO

APROBADO:   NORMAN E. SEARS         FECHA:   28 MAR 69
    N.E. SEARS, DIRECTOR DEL DESARROLLO DE LA MISIÓN
    PROGRAMA DE GUÍA Y NAVEGACIÓN DE APOLLO

APROBADO:   RICHARD H. BATTIN       FECHA:   28 MAR 69
    R.H. BATTIN, DIRECTOR DEL DESARROLLO DE LA MISIÓN
    PROGRAMA DE GUÍA Y NAVEGACIÓN DE APOLLO

APROBADO:   DAVID G. HOAG           FECHA:   28 MAR 69
    D.G. HOAG, DIRECTOR
    PROGRAMA DE GUÍA Y NAVEGACIÓN DE APOLLO

APROBADO:   RALPH R. RAGAN          FECHA:   28 MAR 69
    R.R. RAGAN, DEPUTY DIRECTOR
    LABORATORIO DE INSTRUMENTACIÓN
```

[EN]:README.md
[ZH_CN]:README.zh_cn.md
[ZH_TW]:README.zh_tw.md
[PT_BR]:README.pt_br.md
[ES]:README.es.md
[1]:https://cdn.rawgit.com/aleen42/badges/c9246f74/src/nasa.svg
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/CONTRIBUTING.md
[8]:https://github.com/rburkey2005/virtualagc
