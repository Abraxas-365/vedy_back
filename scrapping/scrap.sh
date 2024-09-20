#!/bin/bash

pagina=1
curl -s 'https://century21.pe/asesores?pagina=${pagina}' -o test.html
