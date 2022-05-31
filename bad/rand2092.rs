
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo2092(_: S4, _: S1) {}
        
        fn test2092() { foo2092(S3, S2, S7, S4, S7, S0); }
    