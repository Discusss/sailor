# Phishing
Una base de datos colaborativa con enlaces maliciosos centrada pero no dedicada en la moderación de Discord.

## Información de la API

- Categorías
  - Phishing
  - Malware
  - Session Hijacking
  - Cross-site Scripting (XSS)
  - Click-jacking
  - IP Grabber
  - Scam
  - Other

- Notes: Notas añadidas por los usuarios.
- Severity: Del 1 al 10, con base en cómo puede afectar a los usuarios.

## Comandos del Bot

- reportar <enlace> <razón> [tipo] [severidad] [nota] - Disponible para todos los usuarios.
- comprobar <enlace>

## Librerías oficiales
Estas son las librerías oficiales del paquete, mantenidas activamente por el equipo de **Discusss**:

### Kotlin

Reemplazar `x.y.z` por la versión que se quiera usar, puedes ver todas las versiones en la [página de releases](https://github.com/Discusss/sailor/releases).

* Repository: https://jitpack.io
* Artifact: **com.github.discusss:sailor:x.y.z**

Using in Gradle:
```gradle
repositories {
  maven {
    url 'https://jitpack.io'
  }
}

dependencies {
  implementation 'com.github.discusss:sailor:x.y.z'
}
```

Using in Maven:
```xml
<repositories>
    <repository>
        <id>jitpack</id>
        <url>https://jitpack.io</url>
    </repository>
</repositories>

<dependency>
  <groupId>com.github.discusss</groupId>
  <artifactId>sailor</artifactId>
  <version>x.y.z</version>
</dependency>
```

## Configuración del archivo `.env`

Copia el contenido del archivo `.env.example` y rellena las variables.

- BOT_TOKEN: token del bot con el que se revisan los enlaces.
- REVIEWING_CHANNEL_ID: canal al que se mandan los mensajes de revisión.
- WEBHOOK_HASH_KEY: clave para la comunicación entre la API y el bot. Puede ser cualquiera.
- API_BASE_URL: enlace base de la API. Algo como http://localhost:8000
- PROMETHEUS_KEY: la clave de prometheus para las estadísticas. Puedes usar cualquier cosa.
- API_AUTH_KEY: La clave de la API para mandar ciertas requests. Se genera desde la API. Puede ser la MASTER_KEY.
- MASTER_KEY: clave maestra de la API para generar otras claves. Debe ser un UUID sin guiones ni espacios.
