#pragma once

using namespace System;

namespace CppCliLibrary {
    public ref class CppCliClass
    {
    public:
        String^ GetMessageFromCSharp();
    };
}


// 関数を外部に公開するための宣言
extern "C" __declspec(dllexport) const wchar_t* GetMessageFromCSharp();
