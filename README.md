## txt2csv

Codi utilitzat per convertir els fitxers TXT a CSV.

### Com utilitzar

El programa espera 2 paràmetres:

- La carpeta on es troben les dades (Ex: ./archive/BBC News Summary).
- Un fitxer on guardar el resultat (CSV).

> txt2csv directori_dades fitxer_resultat

### Com compilar

Es necessita tenir instal.lat [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```shell
git clone git@github.com:JoanAmoros/5073_6_txt2csv.git txt2csv
cd txt2csv
cargo build --release
```
