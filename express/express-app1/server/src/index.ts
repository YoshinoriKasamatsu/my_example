import express from "express";
import path from "path";
import { Config } from "./config";
import fs from "fs";
import { ExcelDataStore } from "./data-store/excel-data-store";

const port = 3000;
const app = express();

// データロード
const excelDataStore = new ExcelDataStore();
excelDataStore.loadData();


app.use(express.json());
app.use(express.raw());


app.post('/api/login', (req, res) => {
  console.log(req.body);
});

app.get('/api/todos', (req, res) => {
  const todos = excelDataStore.getTodos();
  res.set('content-type', 'application/json');
  res.send(todos);
});


app.use('/', express.static(path.join(__dirname, Config.staticFilesPath)));
app.use('/*', express.static(path.join(__dirname, Config.staticFilesPath)));

app.listen(port, () => {
  console.log(`start http://localhost:${port}`)
})
