<p align="center"><img src=".github/assets/logo.png" width="200px" alt="logo"/></p>
<h1 align="center">Passman</h1>
<p align="center"><strong>personal password manager</strong></p>

<p align="center">
  <a href="https://opensource.org/licenses/gpl-3.0.html">
    <img alt="License" src="https://img.shields.io/github/license/eminarican/passman?color=success&style=for-the-badge">
  </a>

  <a href="https://github.com/eminarican/passman/issues">
    <img alt="GitHub Issues" src="https://img.shields.io/github/issues/eminarican/passman?style=for-the-badge">
  </a>

  <a href="https://github.com/eminarican/passman/stargazers">
    <img alt="GitHub Stars" src="https://img.shields.io/github/stars/eminarican/passman?style=for-the-badge">
  </a>
</p>

## ⭐️ Features
- password generating
- secure password storing (uses a master password)

## 💡 Simple usage
```shell
# note: it auto-generates password file when first time using with '-s' master password
passman -s=<secret> set <provider> <password>
passman -s=<secret> get <provider>
passman -s=<secret> del <provider>
passman -s=<secret> gen <provider>
passman -s=<secret> list
```
