# Architettura ad alto livello 

                    +-----------------------+
                    |                       |
                    |   Frontend            |
                    |   (localhost:3001)    |
                    +-----------+-----------+
                                |
                                | HTTP Requests
                                |
                    +-----------v-----------+
                    |                       |
                    |  API REST (Express)   |
                    |   (localhost:3000)    |
                    +-----------+-----------+
                                |
                                | HTTP Requests
                                |
                    +-----------v-----------+
                    |                       |
                    |  rustedhttpd Server   |
                    |       (Custom)        |
                    |                       |
                    +-----------------------+
                                |
                                | File Requests (HTTP)
                                |
                    +-----------v-----------+
                    |                       |
                    |   File System (web)    |
                    |                       |
                    +-----------------------+


# Backend

Questo documento fornisce una relazione tecnica sull'API REST sviluppata con Express e Bun (ts-node non voleva funzionare.) , inclusa una panoramica delle funzionalità offerte, delle tecnologie utilizzate e dell'integrazione con il server web per servire i file statici.

## Panoramica dell'API REST
L'API REST è progettata per fornire un'interfaccia robusta per la gestione dei libri e degli utenti di una piattaforma di biblioteca digitale. Utilizza un'architettura basata su HTTP per consentire agli utenti di registrarsi, autenticarsi, caricare libri e cercare libri per ISBN o nome.

## Funzionalità Principali
Le principali funzionalità dell'API REST includono:

### Gestione degli Utenti:

- Registrazione degli utenti fornendo un'email e una password.
- Autenticazione degli utenti mediante email e password, con generazione di token JWT per l'autorizzazione.

### Gestione dei Libri:

- Caricamento dei libri insieme alle informazioni associate (ISBN, titolo, autore, ecc.) e ai file del libro e alla miniatura.
- Ricerca dei libri per ISBN o per nome.
- Visualizzazione di una lista paginata di libri.

## Tecnologie Utilizzate
L'API REST è sviluppata utilizzando le seguenti tecnologie:

- Bun e Express.js: Utilizzati per la creazione del backend dell'applicazione e per la gestione delle richieste HTTP.
- MongoDB e Mongoose: Utilizzati per la memorizzazione dei dati degli utenti e dei libri in un database NoSQL.
- JWT (JSON Web Token): Utilizzato per l'autenticazione e l'autorizzazione degli utenti.
- Multer e body-parser: Utilizzati per il parsing dei dati delle richieste e la gestione dei file inviati.
- cors: Utilizzato per la gestione delle richieste CORS (Cross-Origin Resource Sharing).

## Integrazione con "rustedhttpd"
Il server web personalizzato "rustedhttpd" viene utilizzato per servire i file statici associati ai libri, come i file del libro e le miniature, dalla directory rusted_files/web.

### Funzionamento di "rustedhttpd"
"rustedhttpd" è un server web custom sviluppato in Rust che serve i file statici tramite protocollo HTTP. Utilizzando un'interfaccia di configurazione, è possibile specificare la directory dei file da servire e le impostazioni del server.

Nel contesto dell'API REST, "rustedhttpd" è integrato come parte del sistema di distribuzione dei contenuti statici. Quando un utente effettua una richiesta per un file associato a un libro (come il file del libro o la miniatura), l'API REST instrada la richiesta a "rustedhttpd", che a sua volta restituisce il file richiesto al client.

## Documentazione endpoint:

### /upload-book (POST)
Questo endpoint consente agli utenti autenticati di caricare un nuovo libro insieme alle informazioni associate e ai file del libro e alla miniatura.

- **Metodo HTTP:** `POST`
- **Endpoint:** `/upload-book`
- **Headers:**
  - Authorization: Token JWT per l'autenticazione dell'utente
- **Body (form-data):**
  - ```json
    {
      "isbn": "Stringa, ISBN del libro",
      "title": "Stringa, titolo del libro",
      "plot": "Stringa, trama del libro",
      "year": "Numero intero, anno di pubblicazione del libro",
      "language": "Stringa, lingua del libro",
      "pages": "Numero intero, numero di pagine del libro",
      "author": "Stringa, autore del libro",
      "publisher": "Stringa, editore del libro",
      "categories": "Array di stringhe, categorie del libro",
      "book": "File, file del libro",
      "thumbnail": "File, miniatura del libro"
    }
    ```
- **Risposte:**
  - ```json
    {
      "200 OK": "Il libro è stato caricato con successo.",
      "400 Bad Request": "I file del libro e/o della miniatura non sono stati forniti.",
      "403 Forbidden": "Token di autenticazione mancante.",
      "500 Internal Server Error": "Errore interno del server."
    }
    ```

### /login (POST)
Questo endpoint consente agli utenti di autenticarsi fornendo email e password.

- **Metodo HTTP:** `POST`
- **Endpoint:** `/login`
- **Body:**
  - ```json
    {
      "email": "Stringa, email dell'utente",
      "password": "Stringa, password dell'utente"
    }
    ```
- **Risposte:**
  - ```json
    {
      "200 OK": "L'autenticazione è avvenuta con successo, viene restituito un token JWT.",
      "401 Unauthorized": "Email o password non validi.",
      "500 Internal Server Error": "Errore interno del server."
    }
    ```

### /create (POST)
Questo endpoint consente agli utenti di creare un nuovo account fornendo email e password.

- **Metodo HTTP:** `POST`
- **Endpoint:** `/create`
- **Body:**
  - ```json
    {
      "email": "Stringa, email dell'utente",
      "password": "Stringa, password dell'utente"
    }
    ```
- **Risposte:**
  - ```json
    {
      "201 Created": "L'account è stato creato con successo.",
      "400 Bad Request": "L'utente esiste già.",
      "500 Internal Server Error": "Errore interno del server."
    }
    ```



### /search-by-isbn/:isbn (GET)

Questo endpoint consente agli utenti di cercare un libro per ISBN.

**Endpoint:** `/search-by-isbn/:isbn`

**Metodo HTTP:** `GET`

**Parametri**:

isbn: ISBN del libro (parametro di percorso)

**Risposte**:

200 OK: Il libro è stato trovato e viene restituito.

404 Not Found: Il libro non è stato trovato.

500 Internal Server Error: Errore interno del server.


### /search-by-name/:name (GET)

Questo endpoint consente agli utenti di cercare libri per nome.

**Endpoint**: `/search-by-name/:name`

**Metodo HTTP**: `GET`

**Parametri**:

name: Nome del libro (parametro di percorso)

**Risposte**:

  - ```json
    {
      "200 OK": "La lista dei libri è stata restituita con successo.",
      "404 Not Found": "Nessun libro trovato.",
      "500 Internal Server Error": "Errore interno del server."
    }
    ```

### /books (GET)

Questo endpoint consente agli utenti di ottenere una lista paginata di libri.

**Metodo HTTP:** `GET`

**Endpoint:** `/books`

**Parametri:**

page: Numero di pagina (parametro di query, opzionale)

**Risposte:**

  - ```json
    {
      "200 OK": "La lista dei libri è stata restituita con successo.",
      "500 Internal Server Error": "Errore interno del server."
    }
    ```

### /search-by-name/:name (GET)
Questo endpoint consente agli utenti di cercare libri per nome.

- **Endpoint:** /search-by-name/:name
- **Metodo HTTP:** `GET`
- **Parametri:**
  - name: Nome del libro (parametro di percorso)
- **Risposte:**
  - ```json
    {
      "200 OK": "I libri sono stati trovati e vengono restituiti.",
      "404 Not Found": "Nessun libro trovato.",
      "500 Internal Server Error": "Errore interno del server."
    }
    ```

### /books (GET)
Questo endpoint consente agli utenti di ottenere una lista paginata di libri.

- **Metodo HTTP:** `GET`
- **Endpoint:** `/books`
- **Parametri:**
  - page: Numero di pagina (parametro di query, opzionale)
- **Risposte:**
  - ```json
    {
      "200 OK": "La lista dei libri è stata restituita con successo.",
      "500 Internal Server Error": "Errore interno del server."
    }
    ```
