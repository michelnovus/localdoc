# Localdoc

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Status](https://img.shields.io/badge/Status-unreleased-red)
![OS Target](https://img.shields.io/badge/OS-GNU/Linux-blue)

## Objetivo de Localdoc

La tarea principal de Localdoc es guardar documentación web estática de
forma local para poder ser utilizada cuando no se tenga conexión a Internet
o consultarla de forma cómoda y rápida en el localhost de igual forma de como
se vería e interacturaría con ella si se consultara online.

## Componenetes

**Localdoc** tiene de dos componentes necesarios, el servicio en segundo plano
*lodosrv* y el TUI *lodoctl*.  
El programa *lodoctl* se encarga principalmente de bifurcar un proceso 
independiente de *lodosrv* que se ejecutará en segundo plano exponiendo
un socket como API. Por otro lado se puede utilizar *lodoctl* (en la consola)
u otro programa que utilice la API de *Localdoc* para controlar que debe hacer 
el servidor.

## Por que los nombres lodo-_{sufijo}_

Los programas tienen por nombre "lodoctl", "lodosrv" de las iniciales de 
**Lo**cal**Do**c(umentation){**C**on**t**ro**l**|**S**e**rv**ice}, después 
de no convencerme los nombres "localdocctl", "localdocsrv", "localdoclib"
y otras cuantas variantes.
