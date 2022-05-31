
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2670(_: S3, _: S7, _: S8) {}
        
        fn test2670() { foo2670(S1, S2, S3, S4, S6, S8); }
    