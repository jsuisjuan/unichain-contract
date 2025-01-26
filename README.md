# FileManagerContract

Este projeto é um contrato inteligente desenvolvido com a framework [ink!](https://use.ink/), focado em gerenciar arquivos dentro de uma blockchain. O contrato permite a criação, leitura, atualização e exclusão (CRUD) de arquivos, associando a cada arquivo um tipo, um nome e um dono.

### Funcionalidades

- **Adicionar Arquivo**: O contrato permite que qualquer usuário registre um arquivo, informando o nome e o tipo (PDF ou DOCX).
- **Consultar Arquivo**: Permite que qualquer usuário consulte o estado atual de um arquivo através de seu ID.
- **Atualizar Arquivo**: Apenas o dono de um arquivo pode atualizar o nome e o tipo do arquivo.
- **Excluir Arquivo**: Apenas o dono de um arquivo pode deletá-lo.

### Estrutura do Contrato

O contrato `FileManagerContract` é composto pelos seguintes componentes principais:

- **FileType**: Enum que define os tipos de arquivo suportados (PDF, DOCX).
- **File**: Estrutura que representa um arquivo, com os campos: `id`, `name`, `file_type` e `owner` (dono do arquivo).
- **FileManagerContract**: Contrato que mantém o controle de arquivos e gerencia os IDs dos arquivos. Ele oferece funções para adicionar, consultar, atualizar e excluir arquivos.

### Funcionalidades e Mensagens

- `add_file(name: String, file_type: FileType) -> u64`: Adiciona um novo arquivo e retorna o ID do arquivo.
- `get_file(file_id: u64) -> Option<File>`: Retorna as informações de um arquivo a partir do seu ID.
- `update_file(file_id: u64, new_name: String, new_file_type: FileType) -> bool`: Atualiza o nome e o tipo de um arquivo, mas somente se o chamador for o dono do arquivo.
- `delete_file(file_id: u64) -> bool`: Deleta um arquivo, mas somente se o chamador for o dono do arquivo.

### Testes

O contrato vem com uma suite de testes escrita em Rust para garantir que as funcionalidades estão funcionando conforme esperado. Os testes incluem:

- **Construtor**: Verifica se o contrato é inicializado corretamente.
- **Adicionar Arquivo**: Testa a criação de um novo arquivo e se ele pode ser recuperado corretamente.
- **Consultar Arquivo**: Verifica se a consulta de um arquivo funciona corretamente.
- **Atualizar Arquivo**: Testa se um arquivo pode ser atualizado corretamente.
- **Excluir Arquivo**: Verifica se a exclusão de um arquivo é realizada corretamente e se ele é removido do mapeamento.

##### rodar get_file()
cargo contract call --contract 5FA4tbve4vBkCsYuRzekX1NbMALYXAVUL9ebvceYF5r2jRsb --message get_file --args 0 --suri //Bob --skip-dry-run --output-json

##### rodar add_file()
cargo contract call --contract 5FA4tbve4vBkCsYuRzekX1NbMALYXAVUL9ebvceYF5r2jRsb --message add_file --args "\"teste-file\"" Pdf --suri //Bob --skip-confirm --execute

##### rodar update_file()
cargo contract call --contract 5FA4tbve4vBkCsYuRzekX1NbMALYXAVUL9ebvceYF5r2jRsb --message update_file --args 0 "\"new-file\"" Docx --suri //Bob --skip-confirm --execute

##### rodar delete_file()
cargo contract call --contract 5FA4tbve4vBkCsYuRzekX1NbMALYXAVUL9ebvceYF5r2jRsb --message delete_file --args 1 --suri //Bob --skip-confirm --execute