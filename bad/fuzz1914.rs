
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1914(_: S4, _: S6, _: S7, _: S8) {}
        
        fn test1914() { foo1914(S5, S6, S4, S7, S5, S4, S6, S7); }
    