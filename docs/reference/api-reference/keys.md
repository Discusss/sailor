# Keys

## Obtener información de una clave

{% hint style="warning" %}
Este endpoint solo puede ser usado por administradores con una clave de acceso maestra.
{% endhint %}

Obtener toda la información de una clave existente.

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/keys" method="get" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}

## Crear una clave de acceso

{% hint style="warning" %}
Este endpoint solo puede ser usado por administradores con una clave de acceso maestra.
{% endhint %}

Crear una clave de acceso para un nuevo moderador del servicio

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/keys" method="post" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}

## Borrar una clave de acceso

{% hint style="warning" %}
Este endpoint solo puede ser usado por administradores con una clave de acceso maestra.
{% endhint %}

Borrar permanentemente una clave de acceso de un moderador del servicio.

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/keys" method="delete" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}

## Modificar una clave de acceso

{% hint style="warning" %}
Este endpoint solo puede ser usado por administradores con una clave de acceso maestra.
{% endhint %}

Modificar información de una clave existente.

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/keys" method="patch" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}
