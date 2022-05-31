
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo1539(_: S4, _: S3, _: S5, _: S1, _: S8) {}
        
        fn test1539() { foo1539(S1, S2, S3, S4, S5, S6, S7); }
    