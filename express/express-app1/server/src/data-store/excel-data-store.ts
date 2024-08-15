import { Todo } from "../models/todo";
import XLSX from 'XLSX';
import fs from 'fs';

export class ExcelDataStore{

    readonly dataFilepath = "./data/data.xlsx"
    private isReadLock = false;

    public todos: Todo[] = [];

    constructor(){

    }

    loadData(){

        const dataWb = fs.watch(this.dataFilepath, { },async () => {
            if(this.isReadLock){
                return;
            }
            this.isReadLock = true;
            while(true){
                let isSuccess = false;
                try{
                    this.readExcelData();
                    isSuccess = true;
                }catch(e){
    
                }                
                if(isSuccess){
                    console.log("データ更新");
                    this.isReadLock = false;
                    break;
                }
            }
        });
    }

    private readExcelData() {
        const book = XLSX.readFile(this.dataFilepath, {});
        const sheet = book.Sheets['todo'];
        this.todos = [];
        this.todos = XLSX.utils.sheet_to_json(sheet);
    }

    public getTodos(): Todo[] {
        return this.todos;
    }
}