# Blacklist

## Obtener todos los usuarios

{% hint style="warning" %}
Este endpoint solo puede ser usado por administradores con una clave de acceso maestra.
{% endhint %}

Obtener todos los usuarios que han sido vetados del servicio.

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/blacklist" method="get" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}

O obtener la información específica de un usuario

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/blacklist/{ip}" method="get" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}

## Añadir usuario a la lista

{% hint style="warning" %}
Este endpoint solo puede ser usado por administradores con una clave de acceso maestra.
{% endhint %}

Añadir manualmente un usuario por su IP a la lista de usuarios vetados.

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/blacklist" method="post" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}

## Eliminar un usuario de la lista

{% hint style="warning" %}
Este endpoint solo puede ser usado por administradores con una clave de acceso maestra.
{% endhint %}

Borrar un usuario de la lista negra, esto permitirá que el usuario vuelva a usar el servicio pero podrá volver a ser baneado en un futuro.

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/blacklist" method="delete" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}

## Modificar un usuario de la lista

{% hint style="warning" %}
Este endpoint solo puede ser usado por administradores con una clave de acceso maestra.
{% endhint %}

Modificar información de un veto del servicio.

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/blacklist" method="patch" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}

## Comprobar si he sido baneado

Este endpoint se usa para verificar si la IP del usuario que ha hecho el request está en la lista negra o no, devolviendo información del veto en caso de existir.

{% swagger src="../../.gitbook/assets/openapi.yaml" path="/blacklist/check" method="get" %}
[openapi.yaml](../../.gitbook/assets/openapi.yaml)
{% endswagger %}
