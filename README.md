# AEI TAG PARSER

Ce programme a été écrit pour déserializer le contenu des tags RFID ferroviaires. Il peut être utilisé à la fois en ligne de commande et en tant que librairie externe.

# Utilisation 

## CLI

Désérialisation de tags passés en paramètres :
```bash 
# Un seul tag
$ ./aei-tag-parser 9EA488C030426A179000000000000331
# Ou plusieurs tags
$ ./aei-tag-parser 2F3E06C007DB1E139000000000000331 9EA488C030426A179000000000000331 9EA488C5320CC01B9000000000000331
```

Désérialisation de tags depuis un fichier :
```bash
# Contenu de tags.txt :
$ cat tags.txt
2F3E06C007DB1E139000000000000331
9EA488C030426A179000000000000331
9EA488C5320CC01B9000000000000331
$ ./aei-tag-parser -f test.txt
```

Désérialisation de tags depuis un pipe :
```bash
# Contenu de tags.txt :
$ cat tags.txt
2F3E06C007DB1E139000000000000331
9EA488C030426A179000000000000331
9EA488C5320CC01B9000000000000331
$ cat tags.txt | ./aei-tag-parser
```

## Librairie

Le projet est aussi disponible en tant que librairie. Pour voir la documention :
```
$ cargo doc --open
```

# Build

Pour build le projet, il faut avoir rust d'installé (cf [RustUp](https://rustup.rs/)).

```bash
$ cargo build --release
```

L'executable est généré dans ./target/release/aei-tag-parser