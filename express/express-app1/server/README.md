# express-app1

# 構築手順

## expressの環境構築
```
npm init
npm install express
npm install -g express-generator
```

## TypeScriptに対応するための環境構築
```
npm install -g typescript
npx tsc --init
npm install -D @types/node   
npm install -D @types/express
npm install -D ts-node
npm install -g nodemon
```


# denomonについて
https://www.npmjs.com/package/nodemon

https://github.com/remy/nodemon

## グローバルインストール
npm install nodemon -g
## ローカルインストール
npm install nodemon --save-dev
## 実行方法
nodemon server.js 
nodemon server.js option
## help
nodemon --help
```
  Options:

  --config file ............ alternate nodemon.json config file to use
  -e, --ext ................ extensions to look for, ie. js,pug,hbs.
  -x, --exec app ........... execute script with "app", ie. -x "python -v".
  -w, --watch path ......... watch directory "path" or files. use once for
                             each directory or file to watch.
  -i, --ignore ............. ignore specific files or directories.
  -V, --verbose ............ show detail on what is causing restarts.
  -- <your args> ........... to tell nodemon stop slurping arguments.
```

```
  Examples:

  $ nodemon server.js
  $ nodemon -w ../foo server.js apparg1 apparg2
  $ nodemon --exec python app.py
  $ nodemon --exec "make build" -e "styl hbs"
  $ nodemon app.js -- --config # pass config to app.js

  All options are documented under: nodemon --help options
```

## configのサンプル
https://github.com/remy/nodemon/blob/master/doc/sample-nodemon.md


# Expressについて  
公式：  
  https://expressjs.com/ja/api.html#express.json  
参考：  
  https://www.geeksforgeeks.org/express-js-express-json-function/?ref=lbp  
  https://qiita.com/atlansien/items/c587a0bf2f7f9022107c  


# REST Clientについて
公式：
  https://marketplace.visualstudio.com/items?itemName=humao.rest-client
サンプル:
```
### JSONの確認
POST http://localhost:3000/api/sayhellojson
content-type: application/json

{
    "key": 1
}

### Rawのbodyの確認
POST http://localhost:3000/api/sayhellobin
content-type: application/octet-stream
bin.data

```
# nexe
## nexeのインストール
npm install nexe -g

## nasmのインストール
https://www.nasm.us/
https://www.nasm.us/pub/nasm/releasebuilds/2.16.01/win64/

## ビルド
nexe -i dist\src\index.js --resource "./../app/dist/**/*" --verbose --output output/app.exe --target windows-x64-6.11.2