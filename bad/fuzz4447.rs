
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4447(_: S7, _: S1, _: S7, _: S2) {}
        
        fn test4447() { foo4447(S6, S6, S1, S6, S5, S3, S1, S4); }
    