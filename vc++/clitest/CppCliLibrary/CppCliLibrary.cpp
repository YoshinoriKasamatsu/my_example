#include "pch.h"

#include "CppCliLibrary.h"


using namespace CSharpLibrary;

namespace CppCliLibrary {
    String^ CppCliClass::GetMessageFromCSharp()
    {
        

		MyClass^ myClass = gcnew MyClass();


        return myClass->GetMessage();
    }
}

// 外部に公開する関数の実装
const wchar_t* GetMessageFromCSharp()
{
	const wchar_t* message = L"Hello from C++!";
	return message;
}