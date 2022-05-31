
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2194(_: S7, _: S1, _: S1, _: S5, _: S8, _: S8) {}
        
        fn test2194() { foo2194(S8, S7, S3, S7, S6, S5, S1); }
    