
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2252(_: S6, _: S8) {}
        
        fn test2252() { foo2252(S3, S6, S3, S1, S1); }
    