---
description: Información a saber antes de usar esta API
---

# ℹ Información

## Uso de tus datos

Esta API es de uso anónimo, y no se recoge nada de información que se pueda usar para identificar a un usuario, excepto en casos específicos, entre estos está:

### &#x20;- Abuso de la API

Esta API tiene un sistema de detección de uso indebido, como por ejemplo un ataque de DDOS, peticiones masivas u otro tipo de ataque que resulte en una posible interrupción del servicio.&#x20;

Para tomar medidas contra estas acciones se ha implementado un sistema de **blacklist por IP**, se guardará la IP del usuario que ha intentado atacar al servicio con la intención de bloquear futuras peticiones.&#x20;

Por lo general estos bloqueos suelen ser temporales, pero puede darse el caso de un **bloqueo permanente del servicio**.

### &#x20;- Claves de acceso

Al convertirte en [revisor para la API](colaborar/convertirse-en-revisor.md), se te otorgará una **clave de acceso**, con la que podrás aprobar, denegar y editar dominios. Esta clave de acceso guardará **todas las IPs** que han sido usadas con esa clave y el **User-agent** de la última petición.&#x20;

Esto se hace con la intención de garantizar la seguridad de los usuarios de la API y de la propia API, ya que si una clave de acceso cae en las manos equivocadas puede afectar negativamente al servicio.

En el momento en el que se detecte una actividad sospechosa en una clave de acceso, esta quedará suspendida de manera indefinida hasta que un administrador revise el caso. Mientras esta clave esté suspendida no podrá usar el servicio.

{% hint style="info" %}
La información que guardamos es privada, y solo pueden acceder a ella administradores internos del sistema con acceso a la base de datos o una entidad legal con una orden que requiera dichos datos.\
Toda la información que guardamos está encriptada, y no se vende ni se usa para otra cosa que no sea el buen funcionamiento de esta API
{% endhint %}

## Proyectos usando esta API

Esta API está siendo usada en producción por los siguientes proyectos, si quieres que tu proyecto aparezca en esta lista puedes [abrir una pull request](https://github.com/Discusss/phishing/pulls) al repositorio del proyecto

{% embed url="https://lacabra.app" %}
LA CABRA, Bot de Disord centrado en la moderación de servidores
{% endembed %}

{% embed url="https://tp.tutoclub.xyz" %}
TutoPro, Bot de Discord multifunción centrado en la moderación
{% endembed %}

{% embed url="https://kirobot.cc" %}
deKiro, Bot de Discord de uso general
{% endembed %}

{% embed url="https://kenabot.xyz" %}
Kenamaster, Bot de Discord para la moderación exclusiva del servidor de Ibai
{% endembed %}

## Seguridad del proyecto

Esta API está protegida por [Cloudflare](https://cloudflare.com) para evitar interrupciones del servicio por ataques tipo DDOS o XSS. También sigue un sistema de análisis de comportamiento UEBA para evitar usos malintencionados de usuarios o revisores. Más información en:

{% embed url="https://learn.microsoft.com/en-us/defender-cloud-apps/tutorial-suspicious-activity" %}

{% embed url="https://www.cloudflare.com/es-es/security/" %}

{% hint style="danger" %}
Si se encuentra algún error/bug/exploit en el servicio, por favor, comunícanoslo cuanto antes abriendo una issue en el [repositorio del proyecto](https://github.com/Discusss/phishing)
{% endhint %}
