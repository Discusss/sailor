# Domains

## Información de un dominio

Este es el endpoint principal, donde se comprueba si un dominio está o no en la base de datos. Si la petición se hace con una clave de acceso, este responderá con más información.

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/domain" method="get" expanded="false" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}

## Añadir un dominio

Añadir un dominio a la base de datos, que luego será revisado por un moderador del servicio para posteriormente ser aprobado o denegado. Si es aprobado, el dominio estará disponible en el endpoint principal.

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/domain" method="post" expanded="false" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}

## Modificar un dominio

Hacer modificaciones a un dominio en la base de datos. Todos los campos son opcionales y se puede hacer una petición solo con los campos a cambiar.&#x20;

{% hint style="warning" %}
Este endpoint solo puede ser usado por moderadores con una clave de acceso valida.
{% endhint %}

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/domain" method="patch" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}

## Eliminar un dominio

Borrar un dominio de la base de datos de manera permanente.

{% hint style="warning" %}
Este endpoint solo puede ser usado por moderadores con una clave de acceso valida.
{% endhint %}

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/domain" method="delete" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}
