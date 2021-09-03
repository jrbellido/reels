## Ideas

- https://github.com/lemunozm/message-io


## Design

- https://excalidraw.com/#json=6547280228253696,ffm3dH8-31z0S1urn9uiaQ

## Path normalization

- Input:      "/Users/john/Downloads/file"
  Root:       ""
  Normalized: "/Users/john/Downloads/file"

- Input:      "/Users/john/Downloads/file"
  Root:       "/Users/john"
  Normalized: "/Downloads/file"

- Input:      "../../Downloads/file"
  Root:       "/Users/john"
  Normalized: "/Downloads/file"


## Model

/Users/jr/.config/mybackup/config.yml
```yaml
version: 3
local:
  - name: "Home directory"
    paths:
      - /Users/jr/
    repositories: 
      - ssh://milou:3000/mnt/backup/data
      - https://parents:8001/mnt/ds1
```

/etc/mybackup/config.yml
```yaml
version: 3
repositories:
  - name: "Data backup"
    data: /var/mybackup/data_backup
    path: /mnt/backup/data
  - name: "S3 backup"
    path: /mnt/
```  

[Node]
Un nodo puede actuar como cliente, servidor o ambos. Si su rol es servidor escuchará en un puerto determinado y será capaz de descubrir otros nodos en la red. Dependiendo de su configuración y autorización, trabajará con ellos.

[Repository]
Un nodo puede albergar repositorios. Un repositorio tiene un identificador y puede existir en varios nodos en función de su configuración.
mybackup://192.168.1.68:4200/repo-1
mybackup://192.168.1.68:4200/repo-1/snapshot-1

[Backend]
Un repositorio usará un mecanismo de backend para persistir los datos recibidos. Asimismo el backend se usará para obtener esos datos.

[Snapshot]
Representa la instantánea de un directorio determinado. El software es capaz de determinar si este directorio ha sufrido cambios en su contenido para actualizar los respaldos. Según la configuración, una snapshot se replicará en N nodos en los que exista el repositorio sobre el que se está realizando.

[Block]
Es la unidad mínima de información que puede manejar el software y son fragmentos de ficheros cuya longitud varía en función de su tamaño. Tiene un identificador único.


## Reference index

File: $REPO_PATH/index

| **length (bytes)** | **Description**              |
| 1 byte             | Database format version      |
| 20 bytes           | *Reserved*                   |
|                    | ---                          |
| 64 bytes           | File path hash               |
| 256 bytes          | Normalized path using chroot |
| 1 bit              | Reference type               |
| ? bytes            | Length in bytes              |
| 64 bytes           | First chunk hash             |
|                    | ---                          |


## Snapshot index

File: $REPO_PATH/snapshots

| **length (bytes)** | **Description**         |
| 1 byte             | Database format version |
|                    | ---                     |
| 64 bytes           | Hash ID                 |
| ? bytes            | Creation time           |
| ? bytes            | Tag list                |
|                    | ---                     |
