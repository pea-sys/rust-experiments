# sqlite インストール手順

- 1.vcpkg リポジトリをクローン

```
git clone https://github.com/microsoft/vcpkg
```

- 2.リポジトリ直下の bootstrap-vcpkg.bat を実行すると、vcpkg.exe が生成される。
- 3.sqlite をインストール

```
vcpkg.exe install sqlite3:x64-windows
```
