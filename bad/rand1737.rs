
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1737(_: S1, _: S2, _: S6, _: S8) {}
        
        fn test1737() { foo1737(S5, S3, S1, S8, S4); }
    